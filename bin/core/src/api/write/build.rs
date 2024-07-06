use anyhow::{anyhow, Context};
use monitor_client::{
  api::write::*,
  entities::{
    build::{Build, PartialBuildConfig},
    config::core::CoreConfig,
    permission::PermissionLevel,
    user::User,
    NoData,
  },
};
use octorust::types::{
  ReposCreateWebhookRequest, ReposCreateWebhookRequestConfig,
};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  resource,
  state::{github_client, State},
};

impl Resolve<CreateBuild, User> for State {
  #[instrument(name = "CreateBuild", skip(self, user))]
  async fn resolve(
    &self,
    CreateBuild { name, config }: CreateBuild,
    user: User,
  ) -> anyhow::Result<Build> {
    resource::create::<Build>(&name, config, &user).await
  }
}

impl Resolve<CopyBuild, User> for State {
  #[instrument(name = "CopyBuild", skip(self, user))]
  async fn resolve(
    &self,
    CopyBuild { name, id }: CopyBuild,
    user: User,
  ) -> anyhow::Result<Build> {
    let Build { config, .. } =
      resource::get_check_permissions::<Build>(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;
    resource::create::<Build>(&name, config.into(), &user).await
  }
}

impl Resolve<DeleteBuild, User> for State {
  #[instrument(name = "DeleteBuild", skip(self, user))]
  async fn resolve(
    &self,
    DeleteBuild { id }: DeleteBuild,
    user: User,
  ) -> anyhow::Result<Build> {
    resource::delete::<Build>(&id, &user).await
  }
}

impl Resolve<UpdateBuild, User> for State {
  #[instrument(name = "UpdateBuild", skip(self, user))]
  async fn resolve(
    &self,
    UpdateBuild { id, config }: UpdateBuild,
    user: User,
  ) -> anyhow::Result<Build> {
    resource::update::<Build>(&id, config, &user).await
  }
}

impl Resolve<CreateBuildWebhook, User> for State {
  #[instrument(name = "CreateBuildWebhook", skip(self, user))]
  async fn resolve(
    &self,
    CreateBuildWebhook { build }: CreateBuildWebhook,
    user: User,
  ) -> anyhow::Result<CreateBuildWebhookResponse> {
    let Some(github) = github_client() else {
      return Err(anyhow!(
        "github_webhook_app is not configured in core config toml"
      ));
    };

    let build = resource::get_check_permissions::<Build>(
      &build,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    if build.config.repo.is_empty() {
      return Err(anyhow!(
        "No repo configured, can't create webhook"
      ));
    }

    let mut split = build.config.repo.split('/');
    let owner = split.next().context("Build repo has no owner")?;

    let CoreConfig {
      host,
      github_webhook_base_url,
      github_webhook_app,
      github_webhook_secret,
      ..
    } = core_config();

    if !github_webhook_app.owners.iter().any(|o| o == owner) {
      return Err(anyhow!(
        "Cannot manage repo webhooks under owner {owner}"
      ));
    }

    let repo =
      split.next().context("Build repo has no repo after the /")?;

    let github_repos = github.repos();

    // First make sure the webhook isn't already created (inactive ones are ignored)
    let webhooks = github_repos
      .list_all_webhooks(owner, repo)
      .await
      .context("failed to list all webhooks on repo")?
      .body;

    let host = github_webhook_base_url.as_ref().unwrap_or(host);
    let url = format!("{host}/listener/github/build/{}", build.id);

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
        secret: github_webhook_secret.to_string(),
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

    if !build.config.webhook_enabled {
      self
        .resolve(
          UpdateBuild {
            id: build.id,
            config: PartialBuildConfig {
              webhook_enabled: Some(true),
              ..Default::default()
            },
          },
          user,
        )
        .await
        .context("failed to update build to enable webhook")?;
    }

    Ok(NoData {})
  }
}

impl Resolve<DeleteBuildWebhook, User> for State {
  #[instrument(name = "DeleteBuildWebhook", skip(self, user))]
  async fn resolve(
    &self,
    DeleteBuildWebhook { build }: DeleteBuildWebhook,
    user: User,
  ) -> anyhow::Result<DeleteBuildWebhookResponse> {
    let Some(github) = github_client() else {
      return Err(anyhow!(
        "github_webhook_app is not configured in core config toml"
      ));
    };

    let build = resource::get_check_permissions::<Build>(
      &build,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    if build.config.repo.is_empty() {
      return Err(anyhow!(
        "No repo configured, can't delete webhook"
      ));
    }

    let mut split = build.config.repo.split('/');
    let owner = split.next().context("Build repo has no owner")?;

    let CoreConfig {
      host,
      github_webhook_base_url,
      github_webhook_app,
      ..
    } = core_config();

    if !github_webhook_app.owners.iter().any(|o| o == owner) {
      return Err(anyhow!(
        "Cannot manage repo webhooks under owner {owner}"
      ));
    }

    let repo =
      split.next().context("Build repo has no repo after the /")?;

    let github_repos = github.repos();

    let webhooks = github_repos
      .list_all_webhooks(owner, repo)
      .await
      .context("failed to list all webhooks on repo")?
      .body;

    let host = github_webhook_base_url.as_ref().unwrap_or(host);
    let url = format!("{host}/listener/github/build/{}", build.id);

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
