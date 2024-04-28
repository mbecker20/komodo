use std::{collections::HashMap, str::FromStr, sync::OnceLock};

use anyhow::Context;
use async_timing_util::unix_timestamp_ms;
use async_trait::async_trait;
use futures::TryStreamExt;
use monitor_client::{
  api::read::*,
  entities::{
    build::{Build, BuildActionState, BuildListItem},
    permission::PermissionLevel,
    update::{ResourceTargetVariant, UpdateStatus},
    user::User,
    Operation,
  },
};
use mungos::{
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
  },
};
use resolver_api::{Resolve, ResolveToString};

use crate::{
  config::core_config,
  helpers::resource::{
    get_resource_ids_for_non_admin, StateResource,
  },
  state::{action_states, db_client, State},
};

#[async_trait]
impl Resolve<GetBuild, User> for State {
  async fn resolve(
    &self,
    GetBuild { build }: GetBuild,
    user: User,
  ) -> anyhow::Result<Build> {
    Build::get_resource_check_permissions(
      &build,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

#[async_trait]
impl Resolve<ListBuilds, User> for State {
  async fn resolve(
    &self,
    ListBuilds { query }: ListBuilds,
    user: User,
  ) -> anyhow::Result<Vec<BuildListItem>> {
    Build::list_resources_for_user(query, &user).await
  }
}

#[async_trait]
impl Resolve<GetBuildActionState, User> for State {
  async fn resolve(
    &self,
    GetBuildActionState { build }: GetBuildActionState,
    user: User,
  ) -> anyhow::Result<BuildActionState> {
    let build = Build::get_resource_check_permissions(
      &build,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .build
      .get(&build.id)
      .await
      .unwrap_or_default()
      .get()?;
    Ok(action_state)
  }
}

#[async_trait]
impl Resolve<GetBuildsSummary, User> for State {
  async fn resolve(
    &self,
    GetBuildsSummary {}: GetBuildsSummary,
    user: User,
  ) -> anyhow::Result<GetBuildsSummaryResponse> {
    let query = if user.admin {
      None
    } else {
      let ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Build,
      )
      .await?
      .into_iter()
      .flat_map(|id| ObjectId::from_str(&id))
      .collect::<Vec<_>>();
      let query = doc! {
        "_id": { "$in": ids }
      };
      Some(query)
    };
    let total = db_client()
      .await
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
impl Resolve<GetBuildMonthlyStats, User> for State {
  async fn resolve(
    &self,
    GetBuildMonthlyStats { page }: GetBuildMonthlyStats,
    _: User,
  ) -> anyhow::Result<GetBuildMonthlyStatsResponse> {
    let curr_ts = unix_timestamp_ms() as i64;
    let next_day = curr_ts - curr_ts % ONE_DAY_MS + ONE_DAY_MS;

    let close_ts = next_day - page as i64 * 30 * ONE_DAY_MS;
    let open_ts = close_ts - 30 * ONE_DAY_MS;

    let mut build_updates = db_client()
      .await
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
impl Resolve<GetBuildVersions, User> for State {
  async fn resolve(
    &self,
    GetBuildVersions {
      build,
      page,
      major,
      minor,
      patch,
    }: GetBuildVersions,
    user: User,
  ) -> anyhow::Result<Vec<BuildVersionResponseItem>> {
    let build = Build::get_resource_check_permissions(
      &build,
      &user,
      PermissionLevel::Read,
    )
    .await?;

    let mut filter = doc! {
      "target": {
        "type": "Build",
        "id": build.id
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
      &db_client().await.updates,
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

fn docker_organizations() -> &'static String {
  static DOCKER_ORGANIZATIONS: OnceLock<String> = OnceLock::new();
  DOCKER_ORGANIZATIONS.get_or_init(|| {
    serde_json::to_string(&core_config().docker_organizations)
      .expect("failed to serialize docker organizations")
  })
}

#[async_trait]
impl ResolveToString<ListDockerOrganizations, User> for State {
  async fn resolve_to_string(
    &self,
    ListDockerOrganizations {}: ListDockerOrganizations,
    _: User,
  ) -> anyhow::Result<String> {
    Ok(docker_organizations().clone())
  }
}
