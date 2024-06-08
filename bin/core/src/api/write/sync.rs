use anyhow::Context;
use monitor_client::{
  api::write::*,
  entities::{
    self,
    alerter::Alerter,
    build::Build,
    builder::Builder,
    deployment::Deployment,
    permission::PermissionLevel,
    procedure::Procedure,
    repo::Repo,
    server::Server,
    server_template::ServerTemplate,
    sync::{PendingUpdates, ResourceSync},
    user::User,
  },
};
use mungos::{
  by_id::update_one_by_id,
  mongodb::bson::{doc, to_document},
};
use resolver_api::Resolve;

use crate::{
  helpers::{
    query::get_id_to_tags,
    sync::resource::{get_updates_for_view, AllResourcesById},
  },
  resource,
  state::{db_client, State},
};

impl Resolve<CreateResourceSync, User> for State {
  #[instrument(name = "CreateResourceSync", skip(self, user))]
  async fn resolve(
    &self,
    CreateResourceSync { name, config }: CreateResourceSync,
    user: User,
  ) -> anyhow::Result<ResourceSync> {
    resource::create::<ResourceSync>(&name, config, &user).await
  }
}

impl Resolve<CopyResourceSync, User> for State {
  #[instrument(name = "CopyResourceSync", skip(self, user))]
  async fn resolve(
    &self,
    CopyResourceSync { name, id }: CopyResourceSync,
    user: User,
  ) -> anyhow::Result<ResourceSync> {
    let ResourceSync { config, .. } =
      resource::get_check_permissions::<ResourceSync>(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;
    resource::create::<ResourceSync>(&name, config.into(), &user)
      .await
  }
}

impl Resolve<DeleteResourceSync, User> for State {
  #[instrument(name = "DeleteResourceSync", skip(self, user))]
  async fn resolve(
    &self,
    DeleteResourceSync { id }: DeleteResourceSync,
    user: User,
  ) -> anyhow::Result<ResourceSync> {
    resource::delete::<ResourceSync>(&id, &user).await
  }
}

impl Resolve<UpdateResourceSync, User> for State {
  #[instrument(name = "UpdateResourceSync", skip(self, user))]
  async fn resolve(
    &self,
    UpdateResourceSync { id, config }: UpdateResourceSync,
    user: User,
  ) -> anyhow::Result<ResourceSync> {
    resource::update::<ResourceSync>(&id, config, &user).await
  }
}

impl Resolve<RefreshResourceSyncPending, User> for State {
  async fn resolve(
    &self,
    RefreshResourceSyncPending { sync }: RefreshResourceSyncPending,
    user: User,
  ) -> anyhow::Result<ResourceSync> {
    // Even though this is a write request, this doesn't change any config. Anyone that can execute the
    // sync should be able to do this.
    let sync = resource::get_check_permissions::<
      entities::sync::ResourceSync,
    >(&sync, &user, PermissionLevel::Execute)
    .await?;

    let (res, _, hash, message) =
      crate::helpers::sync::remote::get_remote_resources(&sync)
        .await
        .context("failed to get remote resources")?;
    let resources = res?;

    let all_resources = AllResourcesById::load().await?;
    let id_to_tags = get_id_to_tags(None).await?;

    let pending = PendingUpdates {
      hash,
      message,
      server_updates: get_updates_for_view::<Server>(
        resources.servers,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await
      .context("failed to get server updates")?,
      deployment_updates: get_updates_for_view::<Deployment>(
        resources.deployments,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await
      .context("failed to get deployment updates")?,
      build_updates: get_updates_for_view::<Build>(
        resources.builds,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await
      .context("failed to get build updates")?,
      repo_updates: get_updates_for_view::<Repo>(
        resources.repos,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await
      .context("failed to get repo updates")?,
      procedure_updates: get_updates_for_view::<Procedure>(
        resources.procedures,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await
      .context("failed to get procedure updates")?,
      alerter_updates: get_updates_for_view::<Alerter>(
        resources.alerters,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await
      .context("failed to get alerter updates")?,
      builder_updates: get_updates_for_view::<Builder>(
        resources.builders,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await
      .context("failed to get builder updates")?,
      server_template_updates:
        get_updates_for_view::<ServerTemplate>(
          resources.server_templates,
          sync.config.delete,
          &all_resources,
          &id_to_tags,
        )
        .await
        .context("failed to get server template updates")?,
      resource_sync_updates: get_updates_for_view::<
        entities::sync::ResourceSync,
      >(
        resources.resource_syncs,
        sync.config.delete,
        &all_resources,
        &id_to_tags,
      )
      .await
      .context("failed to get resource sync updates")?,
    };

    let pending = to_document(&pending)
      .context("failed to serialize pending to document")?;

    update_one_by_id(
      &db_client().await.resource_syncs,
      &sync.id,
      doc! { "$set": { "info.pending": pending } },
      None,
    )
    .await?;

    crate::resource::get::<ResourceSync>(&sync.id).await
  }
}
