use anyhow::{anyhow, Context};
use monitor_client::{
  api::write::*,
  entities::{
    self,
    alert::{Alert, AlertData},
    alerter::Alerter,
    build::Build,
    builder::Builder,
    monitor_timestamp,
    permission::PermissionLevel,
    procedure::Procedure,
    repo::Repo,
    server::{stats::SeverityLevel, Server},
    server_template::ServerTemplate,
    sync::{
      PendingSyncUpdates, PendingSyncUpdatesData,
      PendingSyncUpdatesDataErr, PendingSyncUpdatesDataOk,
      ResourceSync,
    },
    update::ResourceTarget,
    user::User,
  },
};
use mungos::{
  by_id::update_one_by_id,
  mongodb::bson::{doc, to_document},
};
use resolver_api::Resolve;
use serror::serialize_error_pretty;

use crate::{
  helpers::{
    query::get_id_to_tags,
    sync::{
      deployment,
      resource::{get_updates_for_view, AllResourcesById},
    },
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

    if sync.config.repo.is_empty() {
      return Err(anyhow!("resource sync repo not configured"));
    }

    let res = async {
      let (res, _, hash, message) =
        crate::helpers::sync::remote::get_remote_resources(&sync)
          .await
          .context("failed to get remote resources")?;
      let resources = res?;

      let all_resources = AllResourcesById::load().await?;
      let id_to_tags = get_id_to_tags(None).await?;

      let data = PendingSyncUpdatesDataOk {
        server_updates: get_updates_for_view::<Server>(
          resources.servers,
          sync.config.delete,
          &all_resources,
          &id_to_tags,
        )
        .await
        .context("failed to get server updates")?,
        deployment_updates: deployment::get_updates_for_view(
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
        variable_updates:
          crate::helpers::sync::variables::get_updates_for_view(
            resources.variables,
            sync.config.delete,
          )
          .await
          .context("failed to get variable updates")?,
        user_group_updates:
          crate::helpers::sync::user_groups::get_updates_for_view(
            resources.user_groups,
            sync.config.delete,
            &all_resources,
          )
          .await
          .context("failed to get user group updates")?,
      };
      anyhow::Ok((hash, message, data))
    }
    .await;

    let (pending, has_updates) = match res {
      Ok((hash, message, data)) => {
        let has_updates = !data.no_updates();
        (
          PendingSyncUpdates {
            hash: Some(hash),
            message: Some(message),
            data: PendingSyncUpdatesData::Ok(data),
          },
          has_updates,
        )
      }
      Err(e) => (
        PendingSyncUpdates {
          hash: None,
          message: None,
          data: PendingSyncUpdatesData::Err(
            PendingSyncUpdatesDataErr {
              message: serialize_error_pretty(&e),
            },
          ),
        },
        false,
      ),
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

    // check to update alert
    let id = sync.id.clone();
    let name = sync.name.clone();
    tokio::task::spawn(async move {
      let db = db_client().await;
      let Some(existing) = db_client()
        .await
        .alerts
        .find_one(
          doc! {
            "resolved": false,
            "target.type": "ResourceSync",
            "target.id": &id,
          },
          None,
        )
        .await
        .context("failed to query db for alert")
        .inspect_err(|e| warn!("{e:#}"))
        .ok()
      else {
        return;
      };
      match (existing, has_updates) {
        // OPEN A NEW ALERT
        (None, true) => {
          let alert = Alert {
            id: Default::default(),
            ts: monitor_timestamp(),
            resolved: false,
            level: SeverityLevel::Ok,
            target: ResourceTarget::ResourceSync(id.clone()),
            data: AlertData::ResourceSyncPendingUpdates { id, name },
            resolved_ts: None,
          };
          db.alerts
            .insert_one(&alert, None)
            .await
            .context("failed to open existing pending resource sync updates alert")
            .inspect_err(|e| warn!("{e:#}"))
            .ok();
        }
        // CLOSE ALERT
        (Some(existing), false) => {
          update_one_by_id(
            &db.alerts,
            &existing.id,
            doc! {
              "$set": {
                "resolved": true,
                "resolved_ts": monitor_timestamp()
              }
            },
            None,
          )
          .await
          .context("failed to close existing pending resource sync updates alert")
          .inspect_err(|e| warn!("{e:#}"))
          .ok();
        }
        // NOTHING TO DO
        _ => {}
      }
    });

    crate::resource::get::<ResourceSync>(&sync.id).await
  }
}
