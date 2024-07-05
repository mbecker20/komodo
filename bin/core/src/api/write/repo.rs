use anyhow::{anyhow, Context};
use monitor_client::{
  api::write::*,
  entities::{
    config::core::CoreConfig, permission::PermissionLevel,
    repo::Repo, user::User, NoData,
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

impl Resolve<CreateRepo, User> for State {
  #[instrument(name = "CreateRepo", skip(self, user))]
  async fn resolve(
    &self,
    CreateRepo { name, config }: CreateRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    resource::create::<Repo>(&name, config, &user).await
  }
}

impl Resolve<CopyRepo, User> for State {
  #[instrument(name = "CopyRepo", skip(self, user))]
  async fn resolve(
    &self,
    CopyRepo { name, id }: CopyRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    let Repo { config, .. } =
      resource::get_check_permissions::<Repo>(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;
    resource::create::<Repo>(&name, config.into(), &user).await
  }
}

impl Resolve<DeleteRepo, User> for State {
  #[instrument(name = "DeleteRepo", skip(self, user))]
  async fn resolve(
    &self,
    DeleteRepo { id }: DeleteRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    resource::delete::<Repo>(&id, &user).await
  }
}

impl Resolve<UpdateRepo, User> for State {
  #[instrument(name = "UpdateRepo", skip(self, user))]
  async fn resolve(
    &self,
    UpdateRepo { id, config }: UpdateRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    resource::update::<Repo>(&id, config, &user).await
  }
}

impl Resolve<CreateRepoWebhook, User> for State {
  #[instrument(name = "CreateRepoWebhook", skip(self, user))]
  async fn resolve(
    &self,
    CreateRepoWebhook { repo, action }: CreateRepoWebhook,
    user: User,
  ) -> anyhow::Result<CreateRepoWebhookResponse> {
    let Some(github) = github_client() else {
      return Err(anyhow!(
        "github_webhook_app is not configured in core config toml"
      ));
    };

    let repo = resource::get_check_permissions::<Repo>(
      &repo,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    if repo.config.repo.is_empty() {
      return Err(anyhow!(
        "No repo configured, can't create webhook"
      ));
    }

    let mut split = repo.config.repo.split('/');
    let owner = split.next().context("Repo repo has no owner")?;

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

    let repo_name =
      split.next().context("Repo repo has no repo after the /")?;

    let github_repos = github.repos();

    // First make sure the webhook isn't already created (inactive ones are ignored)
    let webhooks = github_repos
      .list_all_webhooks(owner, repo_name)
      .await
      .context("failed to list all webhooks on repo")?
      .body;

    let host = github_webhook_base_url.as_ref().unwrap_or(host);
    let url = match action {
      RepoWebhookAction::Clone => {
        format!("{host}/listener/github/repo/{}/clone", repo.id)
      }
      RepoWebhookAction::Pull => {
        format!("{host}/listener/github/repo/{}/pull", repo.id)
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
        secret: core_config().github_webhook_secret.to_string(),
        content_type: String::from("json"),
        insecure_ssl: None,
        digest: Default::default(),
        token: Default::default(),
      }),
      events: vec![String::from("push")],
      name: String::from("web"),
    };
    github_repos
      .create_webhook(owner, repo_name, &request)
      .await
      .context("failed to create webhook")?;

    Ok(NoData {})
  }
}

impl Resolve<DeleteRepoWebhook, User> for State {
  #[instrument(name = "DeleteRepoWebhook", skip(self, user))]
  async fn resolve(
    &self,
    DeleteRepoWebhook { repo, action }: DeleteRepoWebhook,
    user: User,
  ) -> anyhow::Result<DeleteRepoWebhookResponse> {
    let Some(github) = github_client() else {
      return Err(anyhow!(
        "github_webhook_app is not configured in core config toml"
      ));
    };

    let repo = resource::get_check_permissions::<Repo>(
      &repo,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    if repo.config.repo.is_empty() {
      return Err(anyhow!(
        "No repo configured, can't create webhook"
      ));
    }

    let mut split = repo.config.repo.split('/');
    let owner = split.next().context("Repo repo has no owner")?;

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

    let repo_name =
      split.next().context("Repo repo has no repo after the /")?;

    let github_repos = github.repos();

    // First make sure the webhook isn't already created (inactive ones are ignored)
    let webhooks = github_repos
      .list_all_webhooks(owner, repo_name)
      .await
      .context("failed to list all webhooks on repo")?
      .body;

    let host = github_webhook_base_url.as_ref().unwrap_or(host);
    let url = match action {
      RepoWebhookAction::Clone => {
        format!("{host}/listener/github/repo/{}/clone", repo.id)
      }
      RepoWebhookAction::Pull => {
        format!("{host}/listener/github/repo/{}/pull", repo.id)
      }
    };

    for webhook in webhooks {
      if webhook.active && webhook.config.url == url {
        github_repos
          .delete_webhook(owner, repo_name, webhook.id)
          .await
          .context("failed to delete webhook")?;
        return Ok(NoData {});
      }
    }

    Err(anyhow!("Didn't find any webhook to delete"))
  }
}
