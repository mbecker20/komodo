use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Context};
use formatting::format_serror;
use komodo_client::{
  api::{read::ExportAllResourcesToToml, write::*},
  entities::{
    self,
    action::Action,
    alert::{Alert, AlertData, SeverityLevel},
    alerter::Alerter,
    all_logs_success,
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
      PartialResourceSyncConfig, ResourceSync, ResourceSyncInfo,
    },
    to_komodo_name,
    update::{Log, Update},
    user::sync_user,
    CloneArgs, NoData, Operation, ResourceTarget,
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
use tokio::fs;

use crate::{
  alert::send_alerts,
  api::read::ReadArgs,
  config::core_config,
  helpers::{
    query::get_id_to_tags,
    update::{add_update, make_update, update_update},
  },
  resource::{self, refresh_resource_sync_state_cache},
  state::{db_client, github_client},
  sync::{
    deploy::SyncDeployParams, remote::RemoteResources,
    view::push_updates_for_view, AllResourcesById,
  },
};

use super::WriteArgs;

impl Resolve<WriteArgs> for CreateResourceSync {
  #[instrument(name = "CreateResourceSync", skip(user))]
  async fn resolve(
    self,
    WriteArgs { user }: &WriteArgs,
  ) -> serror::Result<ResourceSync> {
    Ok(
      resource::create::<ResourceSync>(
        &self.name,
        self.config,
        &user,
      )
      .await?,
    )
  }
}

impl Resolve<WriteArgs> for CopyResourceSync {
  #[instrument(name = "CopyResourceSync", skip(user))]
  async fn resolve(
    self,
    WriteArgs { user }: &WriteArgs,
  ) -> serror::Result<ResourceSync> {
    let ResourceSync { config, .. } =
      resource::get_check_permissions::<ResourceSync>(
        &self.id,
        &user,
        PermissionLevel::Write,
      )
      .await?;
    Ok(
      resource::create::<ResourceSync>(
        &self.name,
        config.into(),
        user,
      )
      .await?,
    )
  }
}

impl Resolve<WriteArgs> for DeleteResourceSync {
  #[instrument(name = "DeleteResourceSync", skip(args))]
  async fn resolve(
    self,
    args: &WriteArgs,
  ) -> serror::Result<ResourceSync> {
    Ok(resource::delete::<ResourceSync>(&self.id, args).await?)
  }
}

impl Resolve<WriteArgs> for UpdateResourceSync {
  #[instrument(name = "UpdateResourceSync", skip(user))]
  async fn resolve(
    self,
    WriteArgs { user }: &WriteArgs,
  ) -> serror::Result<ResourceSync> {
    Ok(
      resource::update::<ResourceSync>(&self.id, self.config, user)
        .await?,
    )
  }
}

impl Resolve<WriteArgs> for RenameResourceSync {
  #[instrument(name = "RenameResourceSync", skip(user))]
  async fn resolve(
    self,
    WriteArgs { user }: &WriteArgs,
  ) -> serror::Result<Update> {
    Ok(
      resource::rename::<ResourceSync>(&self.id, &self.name, user)
        .await?,
    )
  }
}

impl Resolve<WriteArgs> for WriteSyncFileContents {
  async fn resolve(self, args: &WriteArgs) -> serror::Result<Update> {
    let WriteSyncFileContents {
      sync,
      resource_path,
      file_path,
      contents,
    } = self;
    let WriteArgs { user } = args;

    let sync = resource::get_check_permissions::<ResourceSync>(
      &sync,
      user,
      PermissionLevel::Write,
    )
    .await?;

    if !sync.config.files_on_host && sync.config.repo.is_empty() {
      return Err(
        anyhow!(
        "This method is only for files on host, or repo based syncs."
      )
        .into(),
      );
    }

    let mut update =
      make_update(&sync, Operation::WriteSyncContents, &user);

    update.push_simple_log("File contents", &contents);

    let root = if sync.config.files_on_host {
      core_config()
        .sync_directory
        .join(to_komodo_name(&sync.name))
    } else {
      let clone_args: CloneArgs = (&sync).into();
      clone_args.unique_path(&core_config().repo_directory)?
    };
    let file_path =
      file_path.parse::<PathBuf>().context("Invalid file path")?;
    let resource_path = resource_path
      .parse::<PathBuf>()
      .context("Invalid resource path")?;
    let full_path = root.join(&resource_path).join(&file_path);

    if let Some(parent) = full_path.parent() {
      let _ = fs::create_dir_all(parent).await;
    }

    if let Err(e) =
      fs::write(&full_path, &contents).await.with_context(|| {
        format!("Failed to write file contents to {full_path:?}")
      })
    {
      update.push_error_log("Write file", format_serror(&e.into()));
    } else {
      update.push_simple_log(
        "Write file",
        format!("File written to {full_path:?}"),
      );
    };

    if !all_logs_success(&update.logs) {
      update.finalize();
      update.id = add_update(update.clone()).await?;

      return Ok(update);
    }

    if sync.config.files_on_host {
      if let Err(e) = (RefreshResourceSyncPending { sync: sync.name })
        .resolve(args)
        .await
      {
        update.push_error_log(
          "Refresh failed",
          format_serror(&e.error.into()),
        );
      }

      update.finalize();
      update.id = add_update(update.clone()).await?;

      return Ok(update);
    }

    let commit_res = git::commit_file(
      &format!("{}: Commit Resource File", user.username),
      &root,
      &resource_path.join(&file_path),
    )
    .await;

    update.logs.extend(commit_res.logs);

    if let Err(e) = (RefreshResourceSyncPending { sync: sync.name })
      .resolve(args)
      .await
    {
      update.push_error_log(
        "Refresh failed",
        format_serror(&e.error.into()),
      );
    }

    update.finalize();
    update.id = add_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<WriteArgs> for CommitSync {
  #[instrument(name = "CommitSync", skip(args))]
  async fn resolve(self, args: &WriteArgs) -> serror::Result<Update> {
    let WriteArgs { user } = args;

    let sync = resource::get_check_permissions::<
      entities::sync::ResourceSync,
    >(&self.sync, &user, PermissionLevel::Write)
    .await?;

    let file_contents_empty = sync.config.file_contents_empty();

    let fresh_sync = !sync.config.files_on_host
      && sync.config.repo.is_empty()
      && file_contents_empty;

    if !sync.config.managed && !fresh_sync {
      return Err(
        anyhow!("Cannot commit to sync. Enabled 'managed' mode.")
          .into(),
      );
    }

    // Get this here so it can fail before update created.
    let resource_path =
      if sync.config.files_on_host || !sync.config.repo.is_empty() {
        let resource_path = sync
          .config
          .resource_path
          .first()
          .context("Sync does not have resource path configured.")?
          .parse::<PathBuf>()
          .context("Invalid resource path")?;

        if resource_path
          .extension()
          .context("Resource path missing '.toml' extension")?
          != "toml"
        {
          return Err(
            anyhow!("Resource path missing '.toml' extension").into(),
          );
        }
        Some(resource_path)
      } else {
        None
      };

    let res = ExportAllResourcesToToml {
      tags: sync.config.match_tags.clone(),
    }
    .resolve(&ReadArgs {
      user: sync_user().to_owned(),
    })
    .await?;

    let mut update = make_update(&sync, Operation::CommitSync, &user);
    update.id = add_update(update.clone()).await?;

    update.logs.push(Log::simple("Resources", res.toml.clone()));

    if sync.config.files_on_host {
      let Some(resource_path) = resource_path else {
        // Resource path checked above for files_on_host mode.
        unreachable!()
      };
      let file_path = core_config()
        .sync_directory
        .join(to_komodo_name(&sync.name))
        .join(&resource_path);
      if let Some(parent) = file_path.parent() {
        let _ = tokio::fs::create_dir_all(&parent).await;
      };
      if let Err(e) = tokio::fs::write(&file_path, &res.toml)
        .await
        .with_context(|| {
          format!("Failed to write resource file to {file_path:?}",)
        })
      {
        update.push_error_log(
          "Write resource file",
          format_serror(&e.into()),
        );
        update.finalize();
        add_update(update.clone()).await?;
        return Ok(update);
      } else {
        update.push_simple_log(
          "Write contents",
          format!("File contents written to {file_path:?}"),
        );
      }
    } else if !sync.config.repo.is_empty() {
      let Some(resource_path) = resource_path else {
        // Resource path checked above for repo mode.
        unreachable!()
      };
      // GIT REPO
      let args: CloneArgs = (&sync).into();
      let root = args.unique_path(&core_config().repo_directory)?;
      match git::write_commit_file(
        "Commit Sync",
        &root,
        &resource_path,
        &res.toml,
      )
      .await
      {
        Ok(res) => update.logs.extend(res.logs),
        Err(e) => {
          update.push_error_log(
            "Write resource file",
            format_serror(&e.into()),
          );
          update.finalize();
          add_update(update.clone()).await?;
          return Ok(update);
        }
      }
      // ===========
      // UI DEFINED
    } else if let Err(e) = db_client()
      .resource_syncs
      .update_one(
        doc! { "name": &sync.name },
        doc! { "$set": { "config.file_contents": res.toml } },
      )
      .await
      .context("failed to update file_contents on db")
    {
      update.push_error_log(
        "Write resource to database",
        format_serror(&e.into()),
      );
      update.finalize();
      add_update(update.clone()).await?;
      return Ok(update);
    }

    if let Err(e) = (RefreshResourceSyncPending { sync: sync.name })
      .resolve(args)
      .await
    {
      update.push_error_log(
        "Refresh sync pending",
        format_serror(&e.error.into()),
      );
    };

    update.finalize();

    // Need to manually update the update before cache refresh,
    // and before broadcast with add_update.
    // The Err case of to_document should be unreachable,
    // but will fail to update cache in that case.
    if let Ok(update_doc) = to_document(&update) {
      let _ = update_one_by_id(
        &db_client().updates,
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

impl Resolve<WriteArgs> for RefreshResourceSyncPending {
  #[instrument(
    name = "RefreshResourceSyncPending",
    level = "debug",
    skip(user)
  )]
  async fn resolve(
    self,
    WriteArgs { user }: &WriteArgs,
  ) -> serror::Result<ResourceSync> {
    // Even though this is a write request, this doesn't change any config. Anyone that can execute the
    // sync should be able to do this.
    let mut sync = resource::get_check_permissions::<
      entities::sync::ResourceSync,
    >(&self.sync, user, PermissionLevel::Execute)
    .await?;

    if !sync.config.managed
      && !sync.config.files_on_host
      && sync.config.file_contents.is_empty()
      && sync.config.repo.is_empty()
    {
      // Sync not configured, nothing to refresh
      return Ok(sync);
    }

    let res = async {
      let RemoteResources {
        resources,
        files,
        file_errors,
        hash,
        message,
        ..
      } = crate::sync::remote::get_remote_resources(&sync)
        .await
        .context("failed to get remote resources")?;

      sync.info.remote_contents = files;
      sync.info.remote_errors = file_errors;
      sync.info.pending_hash = hash;
      sync.info.pending_message = message;

      if !sync.info.remote_errors.is_empty() {
        return Err(
          anyhow!(
            "Remote resources have errors. Cannot compute diffs."
          )
          .into(),
        );
      }

      let resources = resources?;

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
        crate::sync::deploy::get_updates_for_view(SyncDeployParams {
          deployments: &resources.deployments,
          deployment_map: &deployments_by_name,
          stacks: &resources.stacks,
          stack_map: &stacks_by_name,
          all_resources: &all_resources,
        })
        .await;

      let delete = sync.config.managed || sync.config.delete;

      let mut diffs = Vec::new();

      {
        push_updates_for_view::<Server>(
          resources.servers,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
        push_updates_for_view::<Stack>(
          resources.stacks,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
        push_updates_for_view::<Deployment>(
          resources.deployments,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
        push_updates_for_view::<Build>(
          resources.builds,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
        push_updates_for_view::<Repo>(
          resources.repos,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
        push_updates_for_view::<Procedure>(
          resources.procedures,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
        push_updates_for_view::<Action>(
          resources.actions,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
        push_updates_for_view::<Builder>(
          resources.builders,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
        push_updates_for_view::<Alerter>(
          resources.alerters,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
        push_updates_for_view::<ServerTemplate>(
          resources.server_templates,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
        push_updates_for_view::<ResourceSync>(
          resources.resource_syncs,
          delete,
          &all_resources,
          None,
          None,
          &id_to_tags,
          &sync.config.match_tags,
          &mut diffs,
        )
        .await?;
      }

      let variable_updates = if sync.config.match_tags.is_empty() {
        crate::sync::variables::get_updates_for_view(
          &resources.variables,
          delete,
        )
        .await?
      } else {
        Default::default()
      };

      let user_group_updates = if sync.config.match_tags.is_empty() {
        crate::sync::user_groups::get_updates_for_view(
          resources.user_groups,
          delete,
          &all_resources,
        )
        .await?
      } else {
        Default::default()
      };

      anyhow::Ok((
        diffs,
        deploy_updates,
        variable_updates,
        user_group_updates,
      ))
    }
    .await;

    let (
      resource_updates,
      deploy_updates,
      variable_updates,
      user_group_updates,
      pending_error,
    ) = match res {
      Ok(res) => (res.0, res.1, res.2, res.3, None),
      Err(e) => (
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Some(format_serror(&e.into())),
      ),
    };

    let has_updates = !resource_updates.is_empty()
      || !deploy_updates.to_deploy == 0
      || !variable_updates.is_empty()
      || !user_group_updates.is_empty();

    let info = ResourceSyncInfo {
      last_sync_ts: sync.info.last_sync_ts,
      last_sync_hash: sync.info.last_sync_hash,
      last_sync_message: sync.info.last_sync_message,
      remote_contents: sync.info.remote_contents,
      remote_errors: sync.info.remote_errors,
      pending_hash: sync.info.pending_hash,
      pending_message: sync.info.pending_message,
      pending_deploy: deploy_updates,
      resource_updates,
      variable_updates,
      user_group_updates,
      pending_error,
    };

    let info = to_document(&info)
      .context("failed to serialize pending to document")?;

    update_one_by_id(
      &db_client().resource_syncs,
      &sync.id,
      doc! { "$set": { "info": info } },
      None,
    )
    .await?;

    // check to update alert
    let id = sync.id.clone();
    let name = sync.name.clone();
    tokio::task::spawn(async move {
      let db = db_client();
      let Some(existing) = db_client()
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

    Ok(crate::resource::get::<ResourceSync>(&sync.id).await?)
  }
}

impl Resolve<WriteArgs> for CreateSyncWebhook {
  #[instrument(name = "CreateSyncWebhook", skip(args))]
  async fn resolve(
    self,
    args: &WriteArgs,
  ) -> serror::Result<CreateSyncWebhookResponse> {
    let WriteArgs { user } = args;
    let Some(github) = github_client() else {
      return Err(
        anyhow!(
          "github_webhook_app is not configured in core config toml"
        )
        .into(),
      );
    };

    let sync = resource::get_check_permissions::<ResourceSync>(
      &self.sync,
      user,
      PermissionLevel::Write,
    )
    .await?;

    if sync.config.repo.is_empty() {
      return Err(
        anyhow!("No repo configured, can't create webhook").into(),
      );
    }

    let mut split = sync.config.repo.split('/');
    let owner = split.next().context("Sync repo has no owner")?;

    let Some(github) = github.get(owner) else {
      return Err(
        anyhow!("Cannot manage repo webhooks under owner {owner}")
          .into(),
      );
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

    let host = if webhook_base_url.is_empty() {
      host
    } else {
      webhook_base_url
    };
    let url = match self.action {
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
      UpdateResourceSync {
        id: sync.id,
        config: PartialResourceSyncConfig {
          webhook_enabled: Some(true),
          ..Default::default()
        },
      }
      .resolve(args)
      .await
      .map_err(|e| e.error)
      .context("failed to update sync to enable webhook")?;
    }

    Ok(NoData {})
  }
}

impl Resolve<WriteArgs> for DeleteSyncWebhook {
  #[instrument(name = "DeleteSyncWebhook", skip(user))]
  async fn resolve(
    self,
    WriteArgs { user }: &WriteArgs,
  ) -> serror::Result<DeleteSyncWebhookResponse> {
    let Some(github) = github_client() else {
      return Err(
        anyhow!(
          "github_webhook_app is not configured in core config toml"
        )
        .into(),
      );
    };

    let sync = resource::get_check_permissions::<ResourceSync>(
      &self.sync,
      user,
      PermissionLevel::Write,
    )
    .await?;

    if sync.config.git_provider != "github.com" {
      return Err(
        anyhow!("Can only manage github.com repo webhooks").into(),
      );
    }

    if sync.config.repo.is_empty() {
      return Err(
        anyhow!("No repo configured, can't create webhook").into(),
      );
    }

    let mut split = sync.config.repo.split('/');
    let owner = split.next().context("Sync repo has no owner")?;

    let Some(github) = github.get(owner) else {
      return Err(
        anyhow!("Cannot manage repo webhooks under owner {owner}")
          .into(),
      );
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

    let host = if webhook_base_url.is_empty() {
      host
    } else {
      webhook_base_url
    };
    let url = match self.action {
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
