use anyhow::{anyhow, Context};
use formatting::format_serror;
use monitor_client::{
  api::write::*,
  entities::{
    config::core::CoreConfig,
    permission::PermissionLevel,
    stack::{PartialStackConfig, Stack, StackInfo},
    user::User,
    NoData,
  },
};
use mungos::mongodb::bson::{doc, to_document};
use octorust::types::{
  ReposCreateWebhookRequest, ReposCreateWebhookRequestConfig,
};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::stack::{
    json::get_config_json, remote::get_remote_compose_file,
  },
  resource,
  state::{db_client, github_client, State},
};

impl Resolve<CreateStack, User> for State {
  #[instrument(name = "CreateStack", skip(self, user))]
  async fn resolve(
    &self,
    CreateStack { name, config }: CreateStack,
    user: User,
  ) -> anyhow::Result<Stack> {
    resource::create::<Stack>(&name, config, &user).await
  }
}

impl Resolve<CopyStack, User> for State {
  #[instrument(name = "CopyStack", skip(self, user))]
  async fn resolve(
    &self,
    CopyStack { name, id }: CopyStack,
    user: User,
  ) -> anyhow::Result<Stack> {
    let Stack { config, .. } =
      resource::get_check_permissions::<Stack>(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;
    resource::create::<Stack>(&name, config.into(), &user).await
  }
}

impl Resolve<DeleteStack, User> for State {
  #[instrument(name = "DeleteStack", skip(self, user))]
  async fn resolve(
    &self,
    DeleteStack { id }: DeleteStack,
    user: User,
  ) -> anyhow::Result<Stack> {
    resource::delete::<Stack>(&id, &user).await
  }
}

impl Resolve<UpdateStack, User> for State {
  #[instrument(name = "UpdateStack", skip(self, user))]
  async fn resolve(
    &self,
    UpdateStack { id, config }: UpdateStack,
    user: User,
  ) -> anyhow::Result<Stack> {
    resource::update::<Stack>(&id, config, &user).await
  }
}

impl Resolve<RefreshStackCache, User> for State {
  #[instrument(name = "RefreshStackCache", skip(self, user))]
  async fn resolve(
    &self,
    RefreshStackCache { stack }: RefreshStackCache,
    user: User,
  ) -> anyhow::Result<NoData> {
    // Even though this is a write request, this doesn't change any config. Anyone that can execute the
    // stack should be able to do this.
    let stack = resource::get_check_permissions::<Stack>(
      &stack,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    let info = if stack.config.file_contents.is_empty() {
      let (res, _, latest_hash, latest_message) =
        get_remote_compose_file(&stack)
          .await
          .context("failed to clone remote compose file")?;
      match res {
        Ok(remote_contents) => {
          let (json, json_error) =
            get_config_json(&remote_contents).await;
          StackInfo {
            json,
            json_error,
            remote_contents: Some(remote_contents),
            remote_error: false,
            latest_hash,
            latest_message,
            ..stack.info
          }
        }
        Err(e) => {
          let remote_contents = format_serror(
            &e.context("failed to read remote compose file").into(),
          );
          StackInfo {
            json: String::new(),
            json_error: true,
            remote_contents: Some(remote_contents),
            remote_error: true,
            latest_hash,
            latest_message,
            ..stack.info
          }
        }
      }
    } else {
      let (json, json_error) =
        get_config_json(&stack.config.file_contents).await;
      StackInfo {
        json,
        json_error,
        ..stack.info
      }
    };

    let info = to_document(&info)
      .context("failed to serialize stack info to bson")?;

    db_client()
      .await
      .stacks
      .update_one(
        doc! { "name": &stack.name },
        doc! { "$set": { "info": info } },
      )
      .await
      .context("failed to update stack info on db")?;

    Ok(NoData {})
  }
}

impl Resolve<CreateStackWebhook, User> for State {
  #[instrument(name = "CreateStackWebhook", skip(self, user))]
  async fn resolve(
    &self,
    CreateStackWebhook { stack, action }: CreateStackWebhook,
    user: User,
  ) -> anyhow::Result<CreateStackWebhookResponse> {
    let Some(github) = github_client() else {
      return Err(anyhow!(
        "github_webhook_app is not configured in core config toml"
      ));
    };

    let stack = resource::get_check_permissions::<Stack>(
      &stack,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    if stack.config.repo.is_empty() {
      return Err(anyhow!(
        "No repo configured, can't create webhook"
      ));
    }

    let mut split = stack.config.repo.split('/');
    let owner = split.next().context("Stack repo has no owner")?;

    let Some(github) = github.get(owner) else {
      return Err(anyhow!(
        "Cannot manage repo webhooks under owner {owner}"
      ));
    };

    let repo =
      split.next().context("Stack repo has no repo after the /")?;

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

    let host = webhook_base_url.as_ref().unwrap_or(host);
    let url = match action {
      StackWebhookAction::Refresh => {
        format!("{host}/listener/github/stack/{}/refresh", stack.id)
      }
      StackWebhookAction::Deploy => {
        format!("{host}/listener/github/stack/{}/deploy", stack.id)
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

    if !stack.config.webhook_enabled {
      self
        .resolve(
          UpdateStack {
            id: stack.id,
            config: PartialStackConfig {
              webhook_enabled: Some(true),
              ..Default::default()
            },
          },
          user,
        )
        .await
        .context("failed to update stack to enable webhook")?;
    }

    Ok(NoData {})
  }
}

impl Resolve<DeleteStackWebhook, User> for State {
  #[instrument(name = "DeleteStackWebhook", skip(self, user))]
  async fn resolve(
    &self,
    DeleteStackWebhook { stack, action }: DeleteStackWebhook,
    user: User,
  ) -> anyhow::Result<DeleteStackWebhookResponse> {
    let Some(github) = github_client() else {
      return Err(anyhow!(
        "github_webhook_app is not configured in core config toml"
      ));
    };

    let stack = resource::get_check_permissions::<Stack>(
      &stack,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    if stack.config.git_provider != "github.com" {
      return Err(anyhow!(
        "Can only manage github.com repo webhooks"
      ));
    }

    if stack.config.repo.is_empty() {
      return Err(anyhow!(
        "No repo configured, can't create webhook"
      ));
    }

    let mut split = stack.config.repo.split('/');
    let owner = split.next().context("Stack repo has no owner")?;

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
      StackWebhookAction::Refresh => {
        format!("{host}/listener/github/stack/{}/refresh", stack.id)
      }
      StackWebhookAction::Deploy => {
        format!("{host}/listener/github/stack/{}/deploy", stack.id)
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
