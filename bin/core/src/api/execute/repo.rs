use std::{future::IntoFuture, time::Duration};

use anyhow::{anyhow, Context};
use formatting::format_serror;
use monitor_client::{
  api::execute::*,
  entities::{
    alert::{Alert, AlertData},
    builder::{Builder, BuilderConfig},
    monitor_timestamp, optional_string,
    permission::PermissionLevel,
    repo::Repo,
    server::{stats::SeverityLevel, Server},
    update::{Log, Update},
    user::User,
  },
};
use mungos::{
  by_id::update_one_by_id,
  mongodb::{
    bson::{doc, to_document},
    options::FindOneOptions,
  },
};
use periphery_client::api::{self, git::RepoActionResponseV1_13};
use resolver_api::Resolve;
use tokio_util::sync::CancellationToken;

use crate::{
  helpers::{
    alert::send_alerts,
    builder::{cleanup_builder_instance, get_builder_periphery},
    channel::repo_cancel_channel,
    git_token, periphery_client,
    update::update_update,
  },
  resource::{self, refresh_repo_state_cache},
  state::{action_states, db_client, State},
};

use super::ExecuteRequest;

impl Resolve<CloneRepo, (User, Update)> for State {
  #[instrument(name = "CloneRepo", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    CloneRepo { repo }: CloneRepo,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let mut repo = resource::get_check_permissions::<Repo>(
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

    update_update(update.clone()).await?;

    let git_token = git_token(
      &repo.config.git_provider,
      &repo.config.git_account,
      |https| repo.config.git_https = https,
    )
    .await
    .with_context(
      || format!("Failed to get git token in call to db. This is a database error, not a token exisitence error. Stopping run. | {} | {}", repo.config.git_provider, repo.config.git_account),
    )?;

    if repo.config.server_id.is_empty() {
      return Err(anyhow!("repo has no server attached"));
    }

    let server =
      resource::get::<Server>(&repo.config.server_id).await?;

    let periphery = periphery_client(&server)?;

    let logs = match periphery
      .request(api::git::CloneRepo {
        args: (&repo).into(),
        git_token,
        environment: repo.config.environment,
        env_file_path: repo.config.env_file_path,
        skip_secret_interp: repo.config.skip_secret_interp,
      })
      .await
    {
      Ok(res) => {
        let res: RepoActionResponseV1_13 = res.into();
        res.logs
      }
      Err(e) => {
        vec![Log::error(
          "clone repo",
          format_serror(&e.context("failed to clone repo").into()),
        )]
      }
    };

    update.logs.extend(logs);
    update.finalize();

    if update.success {
      update_last_pulled_time(&repo.name).await;
    }

    handle_server_update_return(update).await
  }
}

impl Resolve<PullRepo, (User, Update)> for State {
  #[instrument(name = "PullRepo", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PullRepo { repo }: PullRepo,
    (user, mut update): (User, Update),
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

    let logs = match periphery
      .request(api::git::PullRepo {
        name: repo.name.clone(),
        branch: optional_string(&repo.config.branch),
        commit: optional_string(&repo.config.commit),
        on_pull: repo.config.on_pull.into_option(),
        environment: repo.config.environment,
        env_file_path: repo.config.env_file_path,
        skip_secret_interp: repo.config.skip_secret_interp,
      })
      .await
    {
      Ok(res) => {
        let res: RepoActionResponseV1_13 = res.into();
        update.commit_hash = res.commit_hash.unwrap_or_default();
        res.logs
      }
      Err(e) => {
        vec![Log::error(
          "pull repo",
          format_serror(&e.context("failed to pull repo").into()),
        )]
      }
    };

    update.logs.extend(logs);

    update.finalize();

    if update.success {
      update_last_pulled_time(&repo.name).await;
    }

    handle_server_update_return(update).await
  }
}

#[instrument(skip_all, fields(update_id = update.id))]
async fn handle_server_update_return(
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
    )
    .await;
  if let Err(e) = res {
    warn!(
      "failed to update repo last_pulled_at | repo: {repo_name} | {e:#}",
    );
  }
}

impl Resolve<BuildRepo, (User, Update)> for State {
  #[instrument(name = "BuildRepo", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    BuildRepo { repo }: BuildRepo,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let mut repo = resource::get_check_permissions::<Repo>(
      &repo,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    if repo.config.builder_id.is_empty() {
      return Err(anyhow!("Must attach builder to BuildRepo"));
    }

    let git_token = git_token(
      &repo.config.git_provider,
      &repo.config.git_account,
      |https| repo.config.git_https = https,
    )
    .await
    .with_context(
      || format!("Failed to get git token in call to db. This is a database error, not a token exisitence error. Stopping run. | {} | {}", repo.config.git_provider, repo.config.git_account),
    )?;

    // get the action state for the repo (or insert default).
    let action_state =
      action_states().repo.get_or_insert_default(&repo.id).await;

    // This will set action state back to default when dropped.
    // Will also check to ensure repo not already busy before updating.
    let _action_guard =
      action_state.update(|state| state.building = true)?;

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    let mut cancel_recv =
      repo_cancel_channel().receiver.resubscribe();
    let repo_id = repo.id.clone();

    let builder =
      resource::get::<Builder>(&repo.config.builder_id).await?;

    let is_server_builder =
      matches!(&builder.config, BuilderConfig::Server(_));

    tokio::spawn(async move {
      let poll = async {
        loop {
          let (incoming_repo_id, mut update) = tokio::select! {
            _ = cancel_clone.cancelled() => return Ok(()),
            id = cancel_recv.recv() => id?
          };
          if incoming_repo_id == repo_id {
            if is_server_builder {
              update.push_error_log("Cancel acknowledged", "Repo Build cancellation is not possible on server builders at this time. Use an AWS builder to enable this feature.");
            } else {
              update.push_simple_log("Cancel acknowledged", "The repo build cancellation has been queued, it may still take some time.");
            }
            update.finalize();
            let id = update.id.clone();
            if let Err(e) = update_update(update).await {
              warn!("failed to modify Update {id} on db | {e:#}");
            }
            if !is_server_builder {
              cancel_clone.cancel();
            }
            return Ok(());
          }
        }
        #[allow(unreachable_code)]
        anyhow::Ok(())
      };
      tokio::select! {
        _ = cancel_clone.cancelled() => {}
        _ = poll => {}
      }
    });

    // GET BUILDER PERIPHERY

    let (periphery, cleanup_data) = match get_builder_periphery(
      repo.name.clone(),
      None,
      builder,
      &mut update,
    )
    .await
    {
      Ok(builder) => builder,
      Err(e) => {
        warn!("failed to get builder for repo {} | {e:#}", repo.name);
        update.logs.push(Log::error(
          "get builder",
          format_serror(&e.context("failed to get builder").into()),
        ));
        return handle_builder_early_return(
          update, repo.id, repo.name, false,
        )
        .await;
      }
    };

    // CLONE REPO

    let res = tokio::select! {
      res = periphery
        .request(api::git::CloneRepo {
          args: (&repo).into(),
          git_token,
          environment: Default::default(),
          env_file_path: Default::default(),
          skip_secret_interp: Default::default(),
        }) => res,
      _ = cancel.cancelled() => {
        debug!("build cancelled during clone, cleaning up builder");
        update.push_error_log("build cancelled", String::from("user cancelled build during repo clone"));
        cleanup_builder_instance(periphery, cleanup_data, &mut update)
          .await;
        info!("builder cleaned up");
        return handle_builder_early_return(update, repo.id, repo.name, true).await
      },
    };

    let commit_message = match res {
      Ok(res) => {
        debug!("finished repo clone");
        let res: RepoActionResponseV1_13 = res.into();
        update.logs.extend(res.logs);
        update.commit_hash = res.commit_hash.unwrap_or_default();
        res.commit_message.unwrap_or_default()
      }
      Err(e) => {
        update.push_error_log(
          "clone repo",
          format_serror(&e.context("failed to clone repo").into()),
        );
        Default::default()
      }
    };

    update.finalize();

    let db = db_client().await;

    if update.success {
      let _ = db
        .repos
        .update_one(
          doc! { "name": &repo.name },
          doc! { "$set": {
            "info.last_built_at": monitor_timestamp(),
            "info.built_hash": &update.commit_hash,
            "info.built_message": commit_message
          }},
        )
        .await;
    }

    // stop the cancel listening task from going forever
    cancel.cancel();

    cleanup_builder_instance(periphery, cleanup_data, &mut update)
      .await;

    // Need to manually update the update before cache refresh,
    // and before broadcast with add_update.
    // The Err case of to_document should be unreachable,
    // but will fail to update cache in that case.
    if let Ok(update_doc) = to_document(&update) {
      let _ = update_one_by_id(
        &db.updates,
        &update.id,
        mungos::update::Update::Set(update_doc),
        None,
      )
      .await;
      refresh_repo_state_cache().await;
    }

    update_update(update.clone()).await?;

    if !update.success {
      warn!("repo build unsuccessful, alerting...");
      let target = update.target.clone();
      tokio::spawn(async move {
        let alert = Alert {
          id: Default::default(),
          target,
          ts: monitor_timestamp(),
          resolved_ts: Some(monitor_timestamp()),
          resolved: true,
          level: SeverityLevel::Warning,
          data: AlertData::RepoBuildFailed {
            id: repo.id,
            name: repo.name,
          },
        };
        send_alerts(&[alert]).await
      });
    }

    Ok(update)
  }
}

#[instrument(skip(update))]
async fn handle_builder_early_return(
  mut update: Update,
  repo_id: String,
  repo_name: String,
  is_cancel: bool,
) -> anyhow::Result<Update> {
  update.finalize();
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
  if !update.success && !is_cancel {
    warn!("repo build unsuccessful, alerting...");
    let target = update.target.clone();
    tokio::spawn(async move {
      let alert = Alert {
        id: Default::default(),
        target,
        ts: monitor_timestamp(),
        resolved_ts: Some(monitor_timestamp()),
        resolved: true,
        level: SeverityLevel::Warning,
        data: AlertData::RepoBuildFailed {
          id: repo_id,
          name: repo_name,
        },
      };
      send_alerts(&[alert]).await
    });
  }
  Ok(update)
}

#[instrument(skip_all)]
pub async fn validate_cancel_repo_build(
  request: &ExecuteRequest,
) -> anyhow::Result<()> {
  if let ExecuteRequest::CancelRepoBuild(req) = request {
    let repo = resource::get::<Repo>(&req.repo).await?;

    let db = db_client().await;

    let (latest_build, latest_cancel) = tokio::try_join!(
      db.updates
        .find_one(doc! {
          "operation": "BuildRepo",
          "target.id": &repo.id,
        },)
        .with_options(
          FindOneOptions::builder()
            .sort(doc! { "start_ts": -1 })
            .build()
        )
        .into_future(),
      db.updates
        .find_one(doc! {
          "operation": "CancelRepoBuild",
          "target.id": &repo.id,
        },)
        .with_options(
          FindOneOptions::builder()
            .sort(doc! { "start_ts": -1 })
            .build()
        )
        .into_future()
    )?;

    match (latest_build, latest_cancel) {
      (Some(build), Some(cancel)) => {
        if cancel.start_ts > build.start_ts {
          return Err(anyhow!(
            "Repo build has already been cancelled"
          ));
        }
      }
      (None, _) => return Err(anyhow!("No repo build in progress")),
      _ => {}
    };
  }
  Ok(())
}

impl Resolve<CancelRepoBuild, (User, Update)> for State {
  #[instrument(name = "CancelRepoBuild", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    CancelRepoBuild { repo }: CancelRepoBuild,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let repo = resource::get_check_permissions::<Repo>(
      &repo,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // make sure the build is building
    if !action_states()
      .repo
      .get(&repo.id)
      .await
      .and_then(|s| s.get().ok().map(|s| s.building))
      .unwrap_or_default()
    {
      return Err(anyhow!("Repo is not building."));
    }

    update.push_simple_log(
      "cancel triggered",
      "the repo build cancel has been triggered",
    );
    update_update(update.clone()).await?;

    repo_cancel_channel()
      .sender
      .lock()
      .await
      .send((repo.id, update.clone()))?;

    // Make sure cancel is set to complete after some time in case
    // no reciever is there to do it. Prevents update stuck in InProgress.
    let update_id = update.id.clone();
    tokio::spawn(async move {
      tokio::time::sleep(Duration::from_secs(60)).await;
      if let Err(e) = update_one_by_id(
        &db_client().await.updates,
        &update_id,
        doc! { "$set": { "status": "Complete" } },
        None,
      )
      .await
      {
        warn!("failed to set CancelRepoBuild Update status Complete after timeout | {e:#}")
      }
    });

    Ok(update)
  }
}
