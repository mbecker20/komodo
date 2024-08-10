use std::time::Duration;

use anyhow::Context;
use formatting::format_serror;
use monitor_client::entities::{
  permission::PermissionLevel,
  repo::{
    PartialRepoConfig, Repo, RepoConfig, RepoConfigDiff, RepoInfo,
    RepoListItem, RepoListItemInfo, RepoQuerySpecifics, RepoState,
  },
  resource::Resource,
  server::Server,
  update::{ResourceTargetVariant, Update},
  user::User,
  Operation,
};
use mungos::{
  find::find_collect,
  mongodb::{bson::doc, options::FindOneOptions, Collection},
};
use periphery_client::api::git::DeleteRepo;

use crate::{
  helpers::periphery_client,
  state::{
    action_states, db_client, repo_state_cache, repo_status_cache,
  },
};

use super::get_check_permissions;

impl super::MonitorResource for Repo {
  type Config = RepoConfig;
  type PartialConfig = PartialRepoConfig;
  type ConfigDiff = RepoConfigDiff;
  type Info = RepoInfo;
  type ListItem = RepoListItem;
  type QuerySpecifics = RepoQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Repo
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().await.repos
  }

  async fn to_list_item(
    repo: Resource<Self::Config, Self::Info>,
  ) -> Self::ListItem {
    let state = get_repo_state(&repo.id).await;
    let status =
      repo_status_cache().get(&repo.id).await.unwrap_or_default();
    RepoListItem {
      name: repo.name,
      id: repo.id,
      tags: repo.tags,
      resource_type: ResourceTargetVariant::Repo,
      info: RepoListItemInfo {
        server_id: repo.config.server_id,
        builder_id: repo.config.builder_id,
        last_pulled_at: repo.info.last_pulled_at,
        last_built_at: repo.info.last_built_at,
        git_provider: repo.config.git_provider,
        repo: repo.config.repo,
        branch: repo.config.branch,
        state,
        cloned_hash: status.latest_hash.clone(),
        cloned_message: status.latest_message.clone(),
        latest_hash: repo.info.latest_hash,
        built_hash: repo.info.built_hash,
      },
    }
  }

  async fn busy(id: &String) -> anyhow::Result<bool> {
    action_states()
      .repo
      .get(id)
      .await
      .unwrap_or_default()
      .busy()
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateRepo
  }

  fn user_can_create(_user: &User) -> bool {
    true
  }

  async fn validate_create_config(
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user).await
  }

  async fn post_create(
    _created: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    refresh_repo_state_cache().await;
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateRepo
  }

  async fn validate_update_config(
    _id: &str,
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user).await
  }

  async fn post_update(
    _updated: &Self,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    refresh_repo_state_cache().await;
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteRepo
  }

  async fn pre_delete(
    _resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_delete(
    repo: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()> {
    if repo.config.server_id.is_empty() {
      return Ok(());
    }

    let server = super::get::<Server>(&repo.config.server_id).await?;
    let periphery = periphery_client(&server)?;

    match periphery
      .request(DeleteRepo {
        name: repo.name.clone(),
      })
      .await
    {
      Ok(log) => update.logs.push(log),
      Err(e) => update.push_error_log(
        "delete repo on periphery",
        format_serror(&e.context("failed to delete repo").into()),
      ),
    }

    Ok(())
  }
}

pub fn spawn_repo_state_refresh_loop() {
  tokio::spawn(async move {
    loop {
      refresh_repo_state_cache().await;
      tokio::time::sleep(Duration::from_secs(60)).await;
    }
  });
}

pub async fn refresh_repo_state_cache() {
  let _ = async {
    let repos = find_collect(&db_client().await.repos, None, None)
      .await
      .context("failed to get repos from db")?;
    let cache = repo_state_cache();
    for repo in repos {
      let state = get_repo_state_from_db(&repo.id).await;
      cache.insert(repo.id, state).await;
    }
    anyhow::Ok(())
  }
  .await
  .inspect_err(|e| {
    error!("failed to refresh repo state cache | {e:#}")
  });
}

#[instrument(skip(user))]
async fn validate_config(
  config: &mut PartialRepoConfig,
  user: &User,
) -> anyhow::Result<()> {
  match &config.server_id {
    Some(server_id) if !server_id.is_empty() => {
      let server = get_check_permissions::<Server>(
          server_id,
          user,
          PermissionLevel::Write,
        )
        .await
        .context("cannot create repo on this server. user must have update permissions on the server.")?;
      config.server_id = Some(server.id);
    }
    _ => {}
  }
  Ok(())
}

async fn get_repo_state(id: &String) -> RepoState {
  if let Some(state) = action_states()
    .repo
    .get(id)
    .await
    .and_then(|s| {
      s.get()
        .map(|s| {
          if s.cloning {
            Some(RepoState::Cloning)
          } else if s.pulling {
            Some(RepoState::Pulling)
          } else {
            None
          }
        })
        .ok()
    })
    .flatten()
  {
    return state;
  }
  repo_state_cache().get(id).await.unwrap_or_default()
}

async fn get_repo_state_from_db(id: &str) -> RepoState {
  async {
    let state = db_client()
      .await
      .updates
      .find_one(doc! {
        "target.type": "Repo",
        "target.id": id,
        "$or": [
          { "operation": "CloneRepo" },
          { "operation": "PullRepo" },
        ],
      })
      .with_options(
        FindOneOptions::builder()
          .sort(doc! { "start_ts": -1 })
          .build(),
      )
      .await?
      .map(|u| {
        if u.success {
          RepoState::Ok
        } else {
          RepoState::Failed
        }
      })
      .unwrap_or(RepoState::Ok);
    anyhow::Ok(state)
  }
  .await
  .inspect_err(|e| warn!("failed to get repo state for {id} | {e:#}"))
  .unwrap_or(RepoState::Unknown)
}
