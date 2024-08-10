use anyhow::{anyhow, Context};
use mongo_indexed::doc;
use monitor_client::{
  api::write::*,
  entities::{
    config::core::CoreConfig,
    permission::PermissionLevel,
    repo::{PartialRepoConfig, Repo, RepoInfo},
    user::User,
    CloneArgs, NoData,
  },
};
use mungos::mongodb::bson::to_document;
use octorust::types::{
  ReposCreateWebhookRequest, ReposCreateWebhookRequestConfig,
};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::{git_token, random_string},
  resource,
  state::{db_client, github_client, State},
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

impl Resolve<RefreshRepoCache, User> for State {
  #[instrument(name = "RefreshRepoCache", skip(self, user))]
  async fn resolve(
    &self,
    RefreshRepoCache { repo }: RefreshRepoCache,
    user: User,
  ) -> anyhow::Result<NoData> {
    // Even though this is a write request, this doesn't change any config. Anyone that can execute the
    // repo should be able to do this.
    let repo = resource::get_check_permissions::<Repo>(
      &repo,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    let config = core_config();

    let repo_dir = config.repo_directory.join(random_string(10));
    let mut clone_args: CloneArgs = (&repo).into();
    // No reason to to the commands here.
    clone_args.on_clone = None;
    clone_args.on_pull = None;
    clone_args.destination = Some(repo_dir.display().to_string());

    let access_token = match (&clone_args.account, &clone_args.provider)
    {
      (None, _) => None,
      (Some(_), None) => {
        return Err(anyhow!(
          "Account is configured, but provider is empty"
        ))
      }
      (Some(username), Some(provider)) => {
        git_token(provider, username, |https| {
          clone_args.https = https
        })
        .await
        .with_context(
          || format!("Failed to get git token in call to db. Stopping run. | {provider} | {username}"),
        )?
      }
    };

    let (_, latest_hash, latest_message, _) = git::clone(
      clone_args,
      &config.repo_directory,
      access_token,
      &[],
      "",
      None,
    )
    .await
    .context("failed to clone repo (the resource) repo")?;

    let info = RepoInfo {
      last_pulled_at: repo.info.last_pulled_at,
      last_built_at: repo.info.last_built_at,
      built_hash: repo.info.built_hash,
      built_message: repo.info.built_message,
      latest_hash,
      latest_message,
    };

    let info = to_document(&info)
      .context("failed to serialize repo info to bson")?;

    db_client()
      .await
      .repos
      .update_one(
        doc! { "name": &repo.name },
        doc! { "$set": { "info": info } },
      )
      .await
      .context("failed to update repo info on db")?;

    if repo_dir.exists() {
      if let Err(e) = std::fs::remove_dir_all(&repo_dir) {
        warn!(
          "failed to remove repo (resource) cache update repo directory | {e:?}"
        )
      }
    }

    Ok(NoData {})
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

    let Some(github) = github.get(owner) else {
      return Err(anyhow!(
        "Cannot manage repo webhooks under owner {owner}"
      ));
    };

    let repo_name =
      split.next().context("Repo repo has no repo after the /")?;

    let github_repos = github.repos();

    // First make sure the webhook isn't already created (inactive ones are ignored)
    let webhooks = github_repos
      .list_all_webhooks(owner, repo_name)
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
      .create_webhook(owner, repo_name, &request)
      .await
      .context("failed to create webhook")?;

    if !repo.config.webhook_enabled {
      self
        .resolve(
          UpdateRepo {
            id: repo.id,
            config: PartialRepoConfig {
              webhook_enabled: Some(true),
              ..Default::default()
            },
          },
          user,
        )
        .await
        .context("failed to update repo to enable webhook")?;
    }

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

    if repo.config.git_provider != "github.com" {
      return Err(anyhow!(
        "Can only manage github.com repo webhooks"
      ));
    }

    if repo.config.repo.is_empty() {
      return Err(anyhow!(
        "No repo configured, can't create webhook"
      ));
    }

    let mut split = repo.config.repo.split('/');
    let owner = split.next().context("Repo repo has no owner")?;

    let Some(github) = github.get(owner) else {
      return Err(anyhow!(
        "Cannot manage repo webhooks under owner {owner}"
      ));
    };

    let repo_name =
      split.next().context("Repo repo has no repo after the /")?;

    let github_repos = github.repos();

    // First make sure the webhook isn't already created (inactive ones are ignored)
    let webhooks = github_repos
      .list_all_webhooks(owner, repo_name)
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

    // No webhook to delete, all good
    Ok(NoData {})
  }
}
