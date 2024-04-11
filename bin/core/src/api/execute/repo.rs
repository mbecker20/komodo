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
  db::db_client,
  helpers::{
    add_update, periphery_client, resource::StateResource,
    update_update,
  },
  state::{action_states, State},
};

#[async_trait]
impl Resolve<CloneRepo, User> for State {
  #[instrument(name = "CloneRepo", skip(self))]
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

    if repo.config.server_id.is_empty() {
      return Err(anyhow!("repo has no server attached"));
    }

    let server = Server::get_resource(&repo.config.server_id).await?;

    let periphery = periphery_client(&server)?;

    let repo_id = repo.id.clone();

    let inner = || async move {
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
    };

    if action_states().repo.busy(&repo_id).await {
      return Err(anyhow!("repo busy"));
    }

    action_states()
      .repo
      .update_entry(&repo_id, |entry| {
        entry.cloning = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .repo
      .update_entry(repo_id, |entry| {
        entry.cloning = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<PullRepo, User> for State {
  #[instrument(name = "PullRepo", skip(self))]
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

    if repo.config.server_id.is_empty() {
      return Err(anyhow!("repo has no server attached"));
    }

    let server = Server::get_resource(&repo.config.server_id).await?;

    let periphery = periphery_client(&server)?;

    let repo_id = repo.id.clone();

    let inner = || async move {
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
    };

    if action_states().repo.busy(&repo_id).await {
      return Err(anyhow!("repo busy"));
    }

    action_states()
      .repo
      .update_entry(&repo_id, |entry| {
        entry.pulling = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .repo
      .update_entry(repo_id, |entry| {
        entry.pulling = false;
      })
      .await;

    res
  }
}
