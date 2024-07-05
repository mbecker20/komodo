use std::{
  collections::{HashMap, HashSet},
  sync::OnceLock,
};

use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use futures::TryStreamExt;
use monitor_client::{
  api::read::*,
  entities::{
    build::{Build, BuildActionState, BuildListItem, BuildState},
    config::core::CoreConfig,
    permission::PermissionLevel,
    update::UpdateStatus,
    user::User,
    Operation,
  },
};
use mungos::{
  find::find_collect,
  mongodb::{bson::doc, options::FindOptions},
};
use resolver_api::{Resolve, ResolveToString};

use crate::{
  config::core_config,
  resource,
  state::{
    action_states, build_state_cache, db_client, github_client, State,
  },
};

impl Resolve<GetBuild, User> for State {
  async fn resolve(
    &self,
    GetBuild { build }: GetBuild,
    user: User,
  ) -> anyhow::Result<Build> {
    resource::get_check_permissions::<Build>(
      &build,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

impl Resolve<ListBuilds, User> for State {
  async fn resolve(
    &self,
    ListBuilds { query }: ListBuilds,
    user: User,
  ) -> anyhow::Result<Vec<BuildListItem>> {
    resource::list_for_user::<Build>(query, &user).await
  }
}

impl Resolve<ListFullBuilds, User> for State {
  async fn resolve(
    &self,
    ListFullBuilds { query }: ListFullBuilds,
    user: User,
  ) -> anyhow::Result<ListFullBuildsResponse> {
    resource::list_full_for_user::<Build>(query, &user).await
  }
}

impl Resolve<GetBuildActionState, User> for State {
  async fn resolve(
    &self,
    GetBuildActionState { build }: GetBuildActionState,
    user: User,
  ) -> anyhow::Result<BuildActionState> {
    let build = resource::get_check_permissions::<Build>(
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

impl Resolve<GetBuildsSummary, User> for State {
  async fn resolve(
    &self,
    GetBuildsSummary {}: GetBuildsSummary,
    user: User,
  ) -> anyhow::Result<GetBuildsSummaryResponse> {
    let builds = resource::list_full_for_user::<Build>(
      Default::default(),
      &user,
    )
    .await
    .context("failed to get all builds")?;

    let mut res = GetBuildsSummaryResponse::default();

    let cache = build_state_cache();
    let action_states = action_states();

    for build in builds {
      res.total += 1;

      match (
        cache.get(&build.id).await.unwrap_or_default(),
        action_states
          .build
          .get(&build.id)
          .await
          .unwrap_or_default()
          .get()?,
      ) {
        (_, action_states) if action_states.building => {
          res.building += 1;
        }
        (BuildState::Ok, _) => res.ok += 1,
        (BuildState::Failed, _) => res.failed += 1,
        (BuildState::Unknown, _) => res.unknown += 1,
        // will never come off the cache in the building state, since that comes from action states
        (BuildState::Building, _) => unreachable!(),
      }
    }

    Ok(res)
  }
}

const ONE_DAY_MS: i64 = 86400000;

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
      .find(doc! {
        "start_ts": {
          "$gte": open_ts,
          "$lt": close_ts
        },
        "operation": Operation::RunBuild.to_string(),
      })
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

impl Resolve<GetBuildVersions, User> for State {
  async fn resolve(
    &self,
    GetBuildVersions {
      build,
      major,
      minor,
      patch,
      limit,
    }: GetBuildVersions,
    user: User,
  ) -> anyhow::Result<Vec<BuildVersionResponseItem>> {
    let build = resource::get_check_permissions::<Build>(
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
        .limit(limit)
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

fn github_organizations() -> &'static String {
  static GITHUB_ORGANIZATIONS: OnceLock<String> = OnceLock::new();
  GITHUB_ORGANIZATIONS.get_or_init(|| {
    serde_json::to_string(&core_config().github_organizations)
      .expect("failed to serialize github organizations")
  })
}

impl ResolveToString<ListGithubOrganizations, User> for State {
  async fn resolve_to_string(
    &self,
    ListGithubOrganizations {}: ListGithubOrganizations,
    _: User,
  ) -> anyhow::Result<String> {
    Ok(github_organizations().clone())
  }
}

fn docker_organizations() -> &'static String {
  static DOCKER_ORGANIZATIONS: OnceLock<String> = OnceLock::new();
  DOCKER_ORGANIZATIONS.get_or_init(|| {
    serde_json::to_string(&core_config().docker_organizations)
      .expect("failed to serialize docker organizations")
  })
}

impl ResolveToString<ListDockerOrganizations, User> for State {
  async fn resolve_to_string(
    &self,
    ListDockerOrganizations {}: ListDockerOrganizations,
    _: User,
  ) -> anyhow::Result<String> {
    Ok(docker_organizations().clone())
  }
}

impl Resolve<ListCommonBuildExtraArgs, User> for State {
  async fn resolve(
    &self,
    ListCommonBuildExtraArgs { query }: ListCommonBuildExtraArgs,
    user: User,
  ) -> anyhow::Result<ListCommonBuildExtraArgsResponse> {
    let builds = resource::list_full_for_user::<Build>(query, &user)
      .await
      .context("failed to get resources matching query")?;

    // first collect with guaranteed uniqueness
    let mut res = HashSet::<String>::new();

    for build in builds {
      for extra_arg in build.config.extra_args {
        res.insert(extra_arg);
      }
    }

    let mut res = res.into_iter().collect::<Vec<_>>();
    res.sort();
    Ok(res)
  }
}

impl Resolve<GetBuildWebhookEnabled, User> for State {
  async fn resolve(
    &self,
    GetBuildWebhookEnabled { build }: GetBuildWebhookEnabled,
    user: User,
  ) -> anyhow::Result<GetBuildWebhookEnabledResponse> {
    let Some(github) = github_client() else {
      return Err(anyhow!("github_webhook_app is not configured"));
    };

    let build = resource::get_check_permissions::<Build>(
      &build,
      &user,
      PermissionLevel::Read,
    )
    .await?;

    if build.config.repo.is_empty() {
      return Ok(GetBuildWebhookEnabledResponse {
        managed: false,
        enabled: false,
      });
    }

    let mut split = build.config.repo.split('/');
    let owner = split.next().context("Build repo has no owner")?;

    let CoreConfig {
      host,
      github_webhook_base_url,
      github_webhook_app,
      ..
    } = core_config();

    if !github_webhook_app.owners.iter().any(|o| o == owner) {
      return Ok(GetBuildWebhookEnabledResponse {
        managed: false,
        enabled: false,
      });
    }

    let repo =
      split.next().context("Build repo has no repo after the /")?;

    let github_repos = github.repos();
    let webhooks = github_repos
      .list_all_webhooks(owner, repo)
      .await
      .context("failed to list all webhooks on repo")?
      .body;

    let host = github_webhook_base_url.as_ref().unwrap_or(host);
    let url = format!("{host}/listener/github/build/{}", build.id);

    for webhook in webhooks {
      if webhook.active && webhook.config.url == url {
        return Ok(GetBuildWebhookEnabledResponse {
          managed: true,
          enabled: true,
        });
      }
    }

    Ok(GetBuildWebhookEnabledResponse {
      managed: true,
      enabled: false,
    })
  }
}
