use std::str::FromStr;

use anyhow::anyhow;
use async_trait::async_trait;
use monitor_client::{
  api::execute::*,
  entities::{
    monitor_timestamp, optional_string,
    repo::Repo,
    server::Server,
    update::{Log, ResourceTarget, Update, UpdateStatus},
    user::User,
    Operation, PermissionLevel,
  },
};
use mungos::mongodb::bson::{doc, oid::ObjectId};
use periphery_client::api;
use resolver_api::Resolve;

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
  async fn resolve(
    &self,
    CloneRepo { id }: CloneRepo,
    user: User,
  ) -> anyhow::Result<Update> {
    let repo: Repo = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Execute,
      )
      .await?;

    if repo.config.server_id.is_empty() {
      return Err(anyhow!("repo has no server attached"));
    }

    let server: Server =
      self.get_resource(&repo.config.server_id).await?;

    let periphery = periphery_client(&server)?;

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
          vec![Log::error("clone repo", format!("{e:#?}"))]
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
            "failed to update repo last_pulled_at | repo id: {} | {e:#?}",
            repo.id
          );
        }
      }

      update_update(update.clone()).await?;
      Ok(update)
    };

    if action_states().repo.busy(&id).await {
      return Err(anyhow!("repo busy"));
    }

    action_states()
      .repo
      .update_entry(&id, |entry| {
        entry.cloning = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .repo
      .update_entry(id, |entry| {
        entry.cloning = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<PullRepo, User> for State {
  async fn resolve(
    &self,
    PullRepo { id }: PullRepo,
    user: User,
  ) -> anyhow::Result<Update> {
    let repo: Repo = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    if repo.config.server_id.is_empty() {
      return Err(anyhow!("repo has no server attached"));
    }

    let server: Server =
      self.get_resource(&repo.config.server_id).await?;

    let periphery = periphery_client(&server)?;

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
          vec![Log::error("pull repo", format!("{e:#?}"))]
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
            "failed to update repo last_pulled_at | repo id: {} | {e:#?}",
            repo.id
          );
        }
      }

      update_update(update.clone()).await?;
      Ok(update)
    };

    if action_states().repo.busy(&id).await {
      return Err(anyhow!("repo busy"));
    }

    action_states()
      .repo
      .update_entry(id.clone(), |entry| {
        entry.pulling = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .repo
      .update_entry(id, |entry| {
        entry.pulling = false;
      })
      .await;

    res
  }
}
