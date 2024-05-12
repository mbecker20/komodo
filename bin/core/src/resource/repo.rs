use anyhow::Context;
use monitor_client::entities::{
  permission::PermissionLevel,
  repo::{
    PartialRepoConfig, Repo, RepoConfig, RepoConfigDiff, RepoInfo,
    RepoListItem, RepoListItemInfo, RepoQuerySpecifics,
  },
  resource::Resource,
  server::Server,
  update::{ResourceTargetVariant, Update},
  user::User,
  Operation,
};
use mungos::mongodb::Collection;
use periphery_client::api::git::DeleteRepo;
use serror::serialize_error_pretty;

use crate::{
  helpers::{periphery_client, query::get_repo_state},
  state::{action_states, db_client},
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
  ) -> anyhow::Result<Self::ListItem> {
    let state = get_repo_state(&repo.id).await;
    Ok(RepoListItem {
      name: repo.name,
      id: repo.id,
      tags: repo.tags,
      resource_type: ResourceTargetVariant::Repo,
      info: RepoListItemInfo {
        last_pulled_at: repo.info.last_pulled_at,
        repo: repo.config.repo,
        branch: repo.config.branch,
        state,
      },
    })
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
        serialize_error_pretty(&e),
      ),
    }

    Ok(())
  }
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
