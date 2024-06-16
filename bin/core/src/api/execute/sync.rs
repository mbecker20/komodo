use anyhow::{anyhow, Context};
use mongo_indexed::doc;
use monitor_client::{
  api::{execute::RunSync, write::RefreshResourceSyncPending},
  entities::{
    self,
    alerter::Alerter,
    build::Build,
    builder::Builder,
    monitor_timestamp,
    permission::PermissionLevel,
    procedure::Procedure,
    repo::Repo,
    server::Server,
    server_template::ServerTemplate,
    update::{Log, Update},
    user::{sync_user, User},
  },
};
use mungos::{by_id::update_one_by_id, mongodb::bson::to_document};
use resolver_api::Resolve;
use serror::serialize_error_pretty;

use crate::{
  helpers::{
    formatting::{colored, Color},
    query::get_id_to_tags,
    sync::{
      deployment,
      resource::{
        get_updates_for_execution, AllResourcesById, ResourceSync,
      },
    },
    update::update_update,
  },
  resource::{self, refresh_resource_sync_state_cache},
  state::{db_client, State},
};

impl Resolve<RunSync, (User, Update)> for State {
  #[instrument(name = "RunSync", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RunSync { sync }: RunSync,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let sync = resource::get_check_permissions::<
      entities::sync::ResourceSync,
    >(&sync, &user, PermissionLevel::Execute)
    .await?;

    if sync.config.repo.is_empty() {
      return Err(anyhow!("resource sync repo not configured"));
    }

    let (res, logs, hash, message) =
      crate::helpers::sync::remote::get_remote_resources(&sync)
        .await
        .context("failed to get remote resources")?;

    update.logs.extend(logs);
    update_update(update.clone()).await?;

    let resources = res?;

    let all_resources = AllResourcesById::load().await?;
    let id_to_tags = get_id_to_tags(None).await?;

    let (servers_to_create, servers_to_update, servers_to_delete) =
      get_updates_for_execution::<Server>(
        resources.servers,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await?;
    let (
      deployments_to_create,
      deployments_to_update,
      deployments_to_delete,
    ) = deployment::get_updates_for_execution(
      resources.deployments,
      sync.config.delete,
      &all_resources,
      &id_to_tags,
    )
    .await?;
    let (builds_to_create, builds_to_update, builds_to_delete) =
      get_updates_for_execution::<Build>(
        resources.builds,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await?;
    let (repos_to_create, repos_to_update, repos_to_delete) =
      get_updates_for_execution::<Repo>(
        resources.repos,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await?;
    let (
      procedures_to_create,
      procedures_to_update,
      procedures_to_delete,
    ) = get_updates_for_execution::<Procedure>(
      resources.procedures,
      sync.config.delete,
      &all_resources,
      &id_to_tags,
    )
    .await?;
    let (builders_to_create, builders_to_update, builders_to_delete) =
      get_updates_for_execution::<Builder>(
        resources.builders,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await?;
    let (alerters_to_create, alerters_to_update, alerters_to_delete) =
      get_updates_for_execution::<Alerter>(
        resources.alerters,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await?;
    let (
      server_templates_to_create,
      server_templates_to_update,
      server_templates_to_delete,
    ) = get_updates_for_execution::<ServerTemplate>(
      resources.server_templates,
      sync.config.delete,
      &all_resources,
      &id_to_tags,
    )
    .await?;
    let (
      resource_syncs_to_create,
      resource_syncs_to_update,
      resource_syncs_to_delete,
    ) = get_updates_for_execution::<entities::sync::ResourceSync>(
      resources.resource_syncs,
      sync.config.delete,
      &all_resources,
      &id_to_tags,
    )
    .await?;
    let (
      variables_to_create,
      variables_to_update,
      variables_to_delete,
    ) = crate::helpers::sync::variables::get_updates_for_execution(
      resources.variables,
      sync.config.delete,
    )
    .await?;
    let (
      user_groups_to_create,
      user_groups_to_update,
      user_groups_to_delete,
    ) = crate::helpers::sync::user_groups::get_updates_for_execution(
      resources.user_groups,
      sync.config.delete,
      &all_resources,
    )
    .await?;

    if resource_syncs_to_create.is_empty()
      && resource_syncs_to_update.is_empty()
      && resource_syncs_to_delete.is_empty()
      && server_templates_to_create.is_empty()
      && server_templates_to_update.is_empty()
      && server_templates_to_delete.is_empty()
      && servers_to_create.is_empty()
      && servers_to_update.is_empty()
      && servers_to_delete.is_empty()
      && deployments_to_create.is_empty()
      && deployments_to_update.is_empty()
      && deployments_to_delete.is_empty()
      && builds_to_create.is_empty()
      && builds_to_update.is_empty()
      && builds_to_delete.is_empty()
      && builders_to_create.is_empty()
      && builders_to_update.is_empty()
      && builders_to_delete.is_empty()
      && alerters_to_create.is_empty()
      && alerters_to_update.is_empty()
      && alerters_to_delete.is_empty()
      && repos_to_create.is_empty()
      && repos_to_update.is_empty()
      && repos_to_delete.is_empty()
      && procedures_to_create.is_empty()
      && procedures_to_update.is_empty()
      && procedures_to_delete.is_empty()
      && user_groups_to_create.is_empty()
      && user_groups_to_update.is_empty()
      && user_groups_to_delete.is_empty()
      && variables_to_create.is_empty()
      && variables_to_update.is_empty()
      && variables_to_delete.is_empty()
    {
      update.push_simple_log(
        "No Changes",
        format!(
          "{}. exiting.",
          colored("nothing to do", Color::Green)
        ),
      );
      update.finalize();
      update_update(update.clone()).await?;
      return Ok(update);
    }

    // =================

    // No deps
    maybe_extend(
      &mut update.logs,
      crate::helpers::sync::variables::run_updates(
        variables_to_create,
        variables_to_update,
        variables_to_delete,
      )
      .await,
    );
    maybe_extend(
      &mut update.logs,
      crate::helpers::sync::user_groups::run_updates(
        user_groups_to_create,
        user_groups_to_update,
        user_groups_to_delete,
      )
      .await,
    );
    maybe_extend(
      &mut update.logs,
      entities::sync::ResourceSync::run_updates(
        resource_syncs_to_create,
        resource_syncs_to_update,
        resource_syncs_to_delete,
      )
      .await,
    );
    maybe_extend(
      &mut update.logs,
      ServerTemplate::run_updates(
        server_templates_to_create,
        server_templates_to_update,
        server_templates_to_delete,
      )
      .await,
    );
    maybe_extend(
      &mut update.logs,
      Server::run_updates(
        servers_to_create,
        servers_to_update,
        servers_to_delete,
      )
      .await,
    );
    maybe_extend(
      &mut update.logs,
      Alerter::run_updates(
        alerters_to_create,
        alerters_to_update,
        alerters_to_delete,
      )
      .await,
    );

    // Dependent on server
    maybe_extend(
      &mut update.logs,
      Builder::run_updates(
        builders_to_create,
        builders_to_update,
        builders_to_delete,
      )
      .await,
    );
    maybe_extend(
      &mut update.logs,
      Repo::run_updates(
        repos_to_create,
        repos_to_update,
        repos_to_delete,
      )
      .await,
    );

    // Dependant on builder
    maybe_extend(
      &mut update.logs,
      Build::run_updates(
        builds_to_create,
        builds_to_update,
        builds_to_delete,
      )
      .await,
    );

    // Dependant on server / build
    if let Some(res) = deployment::run_updates(
      deployments_to_create,
      deployments_to_update,
      deployments_to_delete,
    )
    .await
    {
      update.logs.extend(res);
    }

    // Dependant on everything
    maybe_extend(
      &mut update.logs,
      Procedure::run_updates(
        procedures_to_create,
        procedures_to_update,
        procedures_to_delete,
      )
      .await,
    );

    let db = db_client().await;

    if let Err(e) = update_one_by_id(
      &db.resource_syncs,
      &sync.id,
      doc! {
        "$set": {
          "info.last_sync_ts": monitor_timestamp(),
          "info.last_sync_hash": hash,
          "info.last_sync_message": message,
        }
      },
      None,
    )
    .await
    {
      warn!(
        "failed to update resource sync {} info after sync | {e:#}",
        sync.name
      )
    }

    if let Err(e) = State
      .resolve(
        RefreshResourceSyncPending { sync: sync.id },
        sync_user().to_owned(),
      )
      .await
    {
      warn!("failed to refresh sync {} after run | {e:#}", sync.name);
      update.push_error_log(
        "refresh sync",
        format!(
          "failed to refresh sync pending after run | {}",
          serialize_error_pretty(&e)
        ),
      );
    }

    update.finalize();

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
      refresh_resource_sync_state_cache().await;
    }
    update_update(update.clone()).await?;

    Ok(update)
  }
}

fn maybe_extend(logs: &mut Vec<Log>, log: Option<Log>) {
  if let Some(log) = log {
    logs.push(log);
  }
}
