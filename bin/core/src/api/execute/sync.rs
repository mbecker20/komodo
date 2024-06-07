use anyhow::Context;
use mongo_indexed::doc;
use monitor_client::{
  api::execute::RunSync,
  entities::{
    self,
    alerter::Alerter,
    build::Build,
    builder::Builder,
    deployment::Deployment,
    monitor_timestamp,
    permission::PermissionLevel,
    procedure::Procedure,
    repo::Repo,
    server::Server,
    server_template::ServerTemplate,
    update::{Log, Update},
    user::User,
  },
};
use mungos::by_id::update_one_by_id;
use resolver_api::Resolve;

use crate::{
  helpers::{
    query::get_id_to_tags,
    sync::resource::{
      get_updates_for_execution, AllResourcesById, ResourceSync,
    },
    update::update_update,
  },
  resource,
  state::{db_client, State},
};

impl Resolve<RunSync, (User, Update)> for State {
  async fn resolve(
    &self,
    RunSync { sync }: RunSync,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let sync = resource::get_check_permissions::<
      entities::sync::ResourceSync,
    >(&sync, &user, PermissionLevel::Execute)
    .await?;

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
    ) = get_updates_for_execution::<Deployment>(
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
      resources.syncs,
      sync.config.delete,
      &all_resources,
      &id_to_tags,
    )
    .await?;

    // =================

    // No deps
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
    maybe_extend(
      &mut update.logs,
      Deployment::run_updates(
        deployments_to_create,
        deployments_to_update,
        deployments_to_delete,
      )
      .await,
    );

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

    if let Err(e) = update_one_by_id(
      &db_client().await.resource_syncs,
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

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

fn maybe_extend(logs: &mut Vec<Log>, log: Option<Log>) {
  if let Some(log) = log {
    logs.push(log);
  }
}
