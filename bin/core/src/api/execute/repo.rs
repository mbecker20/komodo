use anyhow::anyhow;
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
use mungos::{
  by_id::update_one_by_id,
  mongodb::bson::{doc, to_document},
};
use periphery_client::api;
use resolver_api::Resolve;
use serror::serialize_error_pretty;

use crate::{
  config::core_config,
  helpers::{
    periphery_client,
    update::{add_update, update_update},
  },
  resource::{self, refresh_repo_state_cache},
  state::{action_states, db_client, State},
};

impl Resolve<CloneRepo, User> for State {
  #[instrument(name = "CloneRepo", skip(self, user))]
  async fn resolve(
    &self,
    CloneRepo { repo }: CloneRepo,
    user: User,
  ) -> anyhow::Result<Update> {
    let repo = resource::get_check_permissions::<Repo>(
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

    let server =
      resource::get::<Server>(&repo.config.server_id).await?;

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

    let github_token = core_config()
      .github_accounts
      .get(&repo.config.github_account)
      .cloned();

    let logs = match periphery
      .request(api::git::CloneRepo {
        args: (&repo).into(),
        github_token,
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
      update_last_pulled_time(&repo.name).await;
    }

    handle_update_return(update).await
  }
}

impl Resolve<PullRepo, User> for State {
  #[instrument(name = "PullRepo", skip(self, user))]
  async fn resolve(
    &self,
    PullRepo { repo }: PullRepo,
    user: User,
  ) -> anyhow::Result<Update> {
    let repo = resource::get_check_permissions::<Repo>(
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
      action_state.update(|state| state.pulling = true)?;

    if repo.config.server_id.is_empty() {
      return Err(anyhow!("repo has no server attached"));
    }

    let server =
      resource::get::<Server>(&repo.config.server_id).await?;

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
        name: repo.name.clone(),
        branch: optional_string(&repo.config.branch),
        commit: optional_string(&repo.config.commit),
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
      update_last_pulled_time(&repo.name).await;
    }

    handle_update_return(update).await
  }
}

#[instrument(skip_all, fields(update_id = update.id))]
async fn handle_update_return(
  update: Update,
) -> anyhow::Result<Update> {
  // Need to manually update the update before cache refresh,
  // and before broadcast with add_update.
  // The Err case of to_document should be unreachable,
  // but will fail to update cache in that case.
  if let Ok(update_doc) = to_document(&update) {
    let _ = update_one_by_id(
      &db_client().await.updates,
      &update.id,
      mungos::update::Update::Set(update_doc),
      None,
    )
    .await;
    refresh_repo_state_cache().await;
  }
  update_update(update.clone()).await?;
  Ok(update)
}

#[instrument]
async fn update_last_pulled_time(repo_name: &str) {
  let res = db_client()
    .await
    .repos
    .update_one(
      doc! { "name": repo_name },
      doc! { "$set": { "info.last_pulled_at": monitor_timestamp() } },
      None,
    )
    .await;
  if let Err(e) = res {
    warn!(
      "failed to update repo last_pulled_at | repo: {repo_name} | {e:#}",
    );
  }
}
