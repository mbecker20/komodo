use std::collections::HashMap;

use anyhow::Context;
use async_timing_util::unix_timestamp_ms;
use async_trait::async_trait;
use futures::TryStreamExt;
use monitor_client::{
  api::read::*,
  entities::{
    build::{Build, BuildActionState, BuildListItem},
    resource::AddFilters,
    update::UpdateStatus,
    Operation, PermissionLevel,
  },
};
use mungos::{
  find::find_collect,
  mongodb::{
    bson::{doc, Document},
    options::FindOptions,
  },
};
use resolver_api::Resolve;

use crate::{
  auth::RequestUser,
  db::db_client,
  helpers::resource::StateResource,
  state::{action_states, State},
};

#[async_trait]
impl Resolve<GetBuild, RequestUser> for State {
  async fn resolve(
    &self,
    GetBuild { id }: GetBuild,
    user: RequestUser,
  ) -> anyhow::Result<Build> {
    self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Read,
      )
      .await
  }
}

#[async_trait]
impl Resolve<ListBuilds, RequestUser> for State {
  async fn resolve(
    &self,
    ListBuilds { query }: ListBuilds,
    user: RequestUser,
  ) -> anyhow::Result<Vec<BuildListItem>> {
    let mut filters = Document::new();
    query.add_filters(&mut filters);
    <State as StateResource<Build>>::list_resources_for_user(
      self, filters, &user,
    )
    .await
  }
}

#[async_trait]
impl Resolve<GetBuildActionState, RequestUser> for State {
  async fn resolve(
    &self,
    GetBuildActionState { id }: GetBuildActionState,
    user: RequestUser,
  ) -> anyhow::Result<BuildActionState> {
    let _: Build = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Read,
      )
      .await?;
    let action_state =
      action_states().build.get(&id).await.unwrap_or_default();
    Ok(action_state)
  }
}

#[async_trait]
impl Resolve<GetBuildsSummary, RequestUser> for State {
  async fn resolve(
    &self,
    GetBuildsSummary {}: GetBuildsSummary,
    user: RequestUser,
  ) -> anyhow::Result<GetBuildsSummaryResponse> {
    let query = if user.is_admin {
      None
    } else {
      let query = doc! {
          format!("permissions.{}", user.id): { "$in": ["read", "execute", "update"] }
      };
      Some(query)
    };
    let total = db_client()
      .builds
      .count_documents(query, None)
      .await
      .context("failed to count all build documents")?;
    let res = GetBuildsSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}

const ONE_DAY_MS: i64 = 86400000;

#[async_trait]
impl Resolve<GetBuildMonthlyStats, RequestUser> for State {
  async fn resolve(
    &self,
    GetBuildMonthlyStats { page }: GetBuildMonthlyStats,
    _: RequestUser,
  ) -> anyhow::Result<GetBuildMonthlyStatsResponse> {
    let curr_ts = unix_timestamp_ms() as i64;
    let next_day = curr_ts - curr_ts % ONE_DAY_MS + ONE_DAY_MS;

    let close_ts = next_day - page as i64 * 30 * ONE_DAY_MS;
    let open_ts = close_ts - 30 * ONE_DAY_MS;

    let mut build_updates = db_client()
      .updates
      .find(
        doc! {
          "start_ts": {
            "$gte": open_ts,
            "$lt": close_ts
          },
          "operation": Operation::RunBuild.to_string(),
        },
        None,
      )
      .await
      .context("failed to get updates cursor")?;

    let mut days = HashMap::<i64, BuildStatsDay>::with_capacity(32);

    let mut curr = open_ts;

    while curr < close_ts {
      let stats = BuildStatsDay {
        ts: curr as f64,
        ..Default::default()
      };
      days.insert(curr, stats);
      curr += ONE_DAY_MS;
    }

    while let Some(update) = build_updates.try_next().await? {
      if let Some(end_ts) = update.end_ts {
        let day = update.start_ts - update.start_ts % ONE_DAY_MS;
        let entry = days.entry(day).or_default();
        entry.count += 1.0;
        entry.time += ms_to_hour(end_ts - update.start_ts);
      }
    }

    Ok(GetBuildMonthlyStatsResponse::new(
      days.into_values().collect(),
    ))
  }
}

const MS_TO_HOUR_DIVISOR: f64 = 1000.0 * 60.0 * 60.0;
fn ms_to_hour(duration: i64) -> f64 {
  duration as f64 / MS_TO_HOUR_DIVISOR
}

const NUM_VERSIONS_PER_PAGE: u64 = 10;

#[async_trait]
impl Resolve<GetBuildVersions, RequestUser> for State {
  async fn resolve(
    &self,
    GetBuildVersions {
      id,
      page,
      major,
      minor,
      patch,
    }: GetBuildVersions,
    user: RequestUser,
  ) -> anyhow::Result<Vec<BuildVersionResponseItem>> {
    let _: Build = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Read,
      )
      .await?;

    let mut filter = doc! {
        "target": {
            "type": "Build",
            "id": id
        },
        "operation": Operation::RunBuild.to_string(),
        "status": UpdateStatus::Complete.to_string(),
        "success": true
    };
    if let Some(major) = major {
      filter.insert("version.major", major);
    }
    if let Some(minor) = minor {
      filter.insert("version.minor", minor);
    }
    if let Some(patch) = patch {
      filter.insert("version.patch", patch);
    }

    let versions = find_collect(
      &db_client().updates,
      filter,
      FindOptions::builder()
        .sort(doc! { "_id": -1 })
        .limit(NUM_VERSIONS_PER_PAGE as i64)
        .skip(page as u64 * NUM_VERSIONS_PER_PAGE)
        .build(),
    )
    .await
    .context("failed to pull versions from mongo")?
    .into_iter()
    .map(|u| (u.version, u.start_ts))
    .filter(|(v, _)| !v.is_none())
    .map(|(version, ts)| BuildVersionResponseItem { version, ts })
    .collect();
    Ok(versions)
  }
}
