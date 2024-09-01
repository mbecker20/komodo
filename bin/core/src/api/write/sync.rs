use std::collections::HashMap;

use anyhow::{anyhow, Context};
use formatting::format_serror;
use komodo_client::{
  api::write::*,
  entities::{
    self,
    alert::{Alert, AlertData, SeverityLevel},
    alerter::Alerter,
    build::Build,
    builder::Builder,
    config::core::CoreConfig,
    deployment::Deployment,
    komodo_timestamp,
    permission::PermissionLevel,
    procedure::Procedure,
    repo::Repo,
    server::Server,
    server_template::ServerTemplate,
    stack::Stack,
    sync::{
      PartialResourceSyncConfig, PendingSyncUpdates,
      PendingSyncUpdatesData, PendingSyncUpdatesDataErr,
      PendingSyncUpdatesDataOk, ResourceSync,
    },
    ResourceTarget,
    user::User,
    NoData,
  },
};
use mungos::{
  by_id::update_one_by_id,
  mongodb::bson::{doc, to_document},
};
use octorust::types::{
  ReposCreateWebhookRequest, ReposCreateWebhookRequestConfig,
};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::{
    alert::send_alerts,
    query::get_id_to_tags,
    sync::{
      deploy::SyncDeployParams,
      resource::{get_updates_for_view, AllResourcesById},
    },
  },
  resource,
  state::{db_client, github_client, State},
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
  #[instrument(
    name = "RefreshResourceSyncPending",
    level = "debug",
    skip(self, user)
  )]
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

      let id_to_tags = get_id_to_tags(None).await?;
      let all_resources = AllResourcesById::load().await?;

      let deployments_by_name = all_resources
        .deployments
        .values()
        .map(|deployment| {
          (deployment.name.clone(), deployment.clone())
        })
        .collect::<HashMap<_, _>>();
      let stacks_by_name = all_resources
        .stacks
        .values()
        .map(|stack| (stack.name.clone(), stack.clone()))
        .collect::<HashMap<_, _>>();

      let deploy_updates =
        crate::helpers::sync::deploy::get_updates_for_view(
          SyncDeployParams {
            deployments: &resources.deployments,
            deployment_map: &deployments_by_name,
            stacks: &resources.stacks,
            stack_map: &stacks_by_name,
            all_resources: &all_resources,
          },
        )
        .await;

      let data = PendingSyncUpdatesDataOk {
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
        stack_updates: get_updates_for_view::<Stack>(
          resources.stacks,
          sync.config.delete,
          &all_resources,
          &id_to_tags,
        )
        .await
        .context("failed to get stack updates")?,
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
        deploy_updates,
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
              message: format_serror(&e.into()),
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
        .find_one(doc! {
          "resolved": false,
          "target.type": "ResourceSync",
          "target.id": &id,
        })
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
            ts: komodo_timestamp(),
            resolved: false,
            level: SeverityLevel::Ok,
            target: ResourceTarget::ResourceSync(id.clone()),
            data: AlertData::ResourceSyncPendingUpdates { id, name },
            resolved_ts: None,
          };
          db.alerts
            .insert_one(&alert)
            .await
            .context("failed to open existing pending resource sync updates alert")
            .inspect_err(|e| warn!("{e:#}"))
            .ok();
          send_alerts(&[alert]).await;
        }
        // CLOSE ALERT
        (Some(existing), false) => {
          update_one_by_id(
            &db.alerts,
            &existing.id,
            doc! {
              "$set": {
                "resolved": true,
                "resolved_ts": komodo_timestamp()
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

impl Resolve<CreateSyncWebhook, User> for State {
  #[instrument(name = "CreateSyncWebhook", skip(self, user))]
  async fn resolve(
    &self,
    CreateSyncWebhook { sync, action }: CreateSyncWebhook,
    user: User,
  ) -> anyhow::Result<CreateSyncWebhookResponse> {
    let Some(github) = github_client() else {
      return Err(anyhow!(
        "github_webhook_app is not configured in core config toml"
      ));
    };

    let sync = resource::get_check_permissions::<ResourceSync>(
      &sync,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    if sync.config.repo.is_empty() {
      return Err(anyhow!(
        "No repo configured, can't create webhook"
      ));
    }

    let mut split = sync.config.repo.split('/');
    let owner = split.next().context("Sync repo has no owner")?;

    let Some(github) = github.get(owner) else {
      return Err(anyhow!(
        "Cannot manage repo webhooks under owner {owner}"
      ));
    };

    let repo =
      split.next().context("Repo repo has no repo after the /")?;

    let github_repos = github.repos();

    // First make sure the webhook isn't already created (inactive ones are ignored)
    let webhooks = github_repos
      .list_all_webhooks(owner, repo)
      .await
      .context("failed to list all webhooks on repo")?
      .body;

    let CoreConfig {
      host,
      webhook_base_url,
      webhook_secret,
      ..
    } = core_config();

    let webhook_secret = if sync.config.webhook_secret.is_empty() {
      webhook_secret
    } else {
      &sync.config.webhook_secret
    };

    let host = webhook_base_url.as_ref().unwrap_or(host);
    let url = match action {
      SyncWebhookAction::Refresh => {
        format!("{host}/listener/github/sync/{}/refresh", sync.id)
      }
      SyncWebhookAction::Sync => {
        format!("{host}/listener/github/sync/{}/sync", sync.id)
      }
    };

    for webhook in webhooks {
      if webhook.active && webhook.config.url == url {
        return Ok(NoData {});
      }
    }

    // Now good to create the webhook
    let request = ReposCreateWebhookRequest {
      active: Some(true),
      config: Some(ReposCreateWebhookRequestConfig {
        url,
        secret: webhook_secret.to_string(),
        content_type: String::from("json"),
        insecure_ssl: None,
        digest: Default::default(),
        token: Default::default(),
      }),
      events: vec![String::from("push")],
      name: String::from("web"),
    };
    github_repos
      .create_webhook(owner, repo, &request)
      .await
      .context("failed to create webhook")?;

    if !sync.config.webhook_enabled {
      self
        .resolve(
          UpdateResourceSync {
            id: sync.id,
            config: PartialResourceSyncConfig {
              webhook_enabled: Some(true),
              ..Default::default()
            },
          },
          user,
        )
        .await
        .context("failed to update sync to enable webhook")?;
    }

    Ok(NoData {})
  }
}

impl Resolve<DeleteSyncWebhook, User> for State {
  #[instrument(name = "DeleteSyncWebhook", skip(self, user))]
  async fn resolve(
    &self,
    DeleteSyncWebhook { sync, action }: DeleteSyncWebhook,
    user: User,
  ) -> anyhow::Result<DeleteSyncWebhookResponse> {
    let Some(github) = github_client() else {
      return Err(anyhow!(
        "github_webhook_app is not configured in core config toml"
      ));
    };

    let sync = resource::get_check_permissions::<ResourceSync>(
      &sync,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    if sync.config.git_provider != "github.com" {
      return Err(anyhow!(
        "Can only manage github.com repo webhooks"
      ));
    }

    if sync.config.repo.is_empty() {
      return Err(anyhow!(
        "No repo configured, can't create webhook"
      ));
    }

    let mut split = sync.config.repo.split('/');
    let owner = split.next().context("Sync repo has no owner")?;

    let Some(github) = github.get(owner) else {
      return Err(anyhow!(
        "Cannot manage repo webhooks under owner {owner}"
      ));
    };

    let repo =
      split.next().context("Sync repo has no repo after the /")?;

    let github_repos = github.repos();

    // First make sure the webhook isn't already created (inactive ones are ignored)
    let webhooks = github_repos
      .list_all_webhooks(owner, repo)
      .await
      .context("failed to list all webhooks on repo")?
      .body;

    let CoreConfig {
      host,
      webhook_base_url,
      ..
    } = core_config();

    let host = webhook_base_url.as_ref().unwrap_or(host);
    let url = match action {
      SyncWebhookAction::Refresh => {
        format!("{host}/listener/github/sync/{}/refresh", sync.id)
      }
      SyncWebhookAction::Sync => {
        format!("{host}/listener/github/sync/{}/sync", sync.id)
      }
    };

    for webhook in webhooks {
      if webhook.active && webhook.config.url == url {
        github_repos
          .delete_webhook(owner, repo, webhook.id)
          .await
          .context("failed to delete webhook")?;
        return Ok(NoData {});
      }
    }

    // No webhook to delete, all good
    Ok(NoData {})
  }
}
