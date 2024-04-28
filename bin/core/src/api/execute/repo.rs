use std::str::FromStr;

use anyhow::anyhow;
use async_trait::async_trait;
use monitor_client::{
  api::execute::*,
  entities::{
    monitor_timestamp, optional_string,
    permission::PermissionLevel,
    repo::Repo,
    server::Server,
    update::{Log, ResourceTarget, Update, UpdateStatus},
    user::User,
    Operation,
  },
};
use mungos::mongodb::bson::{doc, oid::ObjectId};
use periphery_client::api;
use resolver_api::Resolve;
use serror::serialize_error_pretty;

use crate::{
  helpers::{
    periphery_client,
    resource::StateResource,
    update::{add_update, update_update},
  },
  state::{action_states, db_client, State},
};

#[async_trait]
impl Resolve<CloneRepo, User> for State {
  #[instrument(name = "CloneRepo", skip(self, user))]
  async fn resolve(
    &self,
    CloneRepo { repo }: CloneRepo,
    user: User,
  ) -> anyhow::Result<Update> {
    let repo = Repo::get_resource_check_permissions(
      &repo,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // get the action state for the repo (or insert default).
    let action_state =
      action_states().repo.get_or_insert_default(&repo.id).await;

    // This will set action state back to default when dropped.
    // Will also check to ensure repo not already busy before updating.
    let _action_guard =
      action_state.update(|state| state.cloning = true)?;

    if repo.config.server_id.is_empty() {
      return Err(anyhow!("repo has no server attached"));
    }

    let server = Server::get_resource(&repo.config.server_id).await?;

    let periphery = periphery_client(&server)?;

    let start_ts = monitor_timestamp();

    let mut update = Update {
      operation: Operation::CloneRepo,
      target: ResourceTarget::Repo(repo.id.clone()),
      start_ts,
      status: UpdateStatus::InProgress,
      operator: user.id.clone(),
      success: true,
      ..Default::default()
    };

    update.id = add_update(update.clone()).await?;

    let logs = match periphery
      .request(api::git::CloneRepo {
        args: (&repo).into(),
      })
      .await
    {
      Ok(logs) => logs,
      Err(e) => {
        vec![Log::error("clone repo", serialize_error_pretty(&e))]
      }
    };

    update.logs.extend(logs);
    update.finalize();

    if update.success {
      let res = db_client().await
          .repos
          .update_one(
            doc! { "_id": ObjectId::from_str(&repo.id)? },
            doc! { "$set": { "info.last_pulled_at": monitor_timestamp() } },
            None,
          )
          .await;
      if let Err(e) = res {
        warn!(
            "failed to update repo last_pulled_at | repo id: {} | {e:#}",
            repo.id
          );
      }
    }

    update_update(update.clone()).await?;
    Ok(update)
  }
}

#[async_trait]
impl Resolve<PullRepo, User> for State {
  #[instrument(name = "PullRepo", skip(self, user))]
  async fn resolve(
    &self,
    PullRepo { repo }: PullRepo,
    user: User,
  ) -> anyhow::Result<Update> {
    let repo = Repo::get_resource_check_permissions(
      &repo,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    // get the action state for the repo (or insert default).
    let action_state =
      action_states().repo.get_or_insert_default(&repo.id).await;

    // This will set action state back to default when dropped.
    // Will also check to ensure repo not already busy before updating.
    let _action_guard =
      action_state.update(|state| state.pulling = true)?;

    if repo.config.server_id.is_empty() {
      return Err(anyhow!("repo has no server attached"));
    }

    let server = Server::get_resource(&repo.config.server_id).await?;

    let periphery = periphery_client(&server)?;

    let start_ts = monitor_timestamp();

    let mut update = Update {
      operation: Operation::PullRepo,
      target: ResourceTarget::Repo(repo.id.clone()),
      start_ts,
      status: UpdateStatus::InProgress,
      operator: user.id.clone(),
      success: true,
      ..Default::default()
    };

    update.id = add_update(update.clone()).await?;

    let logs = match periphery
      .request(api::git::PullRepo {
        name: repo.name,
        branch: optional_string(&repo.config.branch),
        on_pull: repo.config.on_pull.into_option(),
      })
      .await
    {
      Ok(logs) => logs,
      Err(e) => {
        vec![Log::error("pull repo", serialize_error_pretty(&e))]
      }
    };

    update.logs.extend(logs);

    update.finalize();

    if update.success {
      let res = db_client().await
          .repos
          .update_one(
            doc! { "_id": ObjectId::from_str(&repo.id)? },
            doc! { "$set": { "info.last_pulled_at": monitor_timestamp() } },
            None,
          )
          .await;
      if let Err(e) = res {
        warn!(
            "failed to update repo last_pulled_at | repo id: {} | {e:#}",
            repo.id
          );
      }
    }

    update_update(update.clone()).await?;
    Ok(update)
  }
}
