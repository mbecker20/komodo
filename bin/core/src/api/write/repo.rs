use anyhow::{anyhow, Context};
use formatting::format_serror;
use git::GitRes;
use komodo_client::{
  api::write::*,
  entities::{
    config::core::CoreConfig,
    komodo_timestamp,
    permission::PermissionLevel,
    repo::{PartialRepoConfig, Repo, RepoInfo},
    server::Server,
    to_komodo_name,
    update::{Log, Update},
    user::User,
    CloneArgs, NoData, Operation,
  },
};
use mongo_indexed::doc;
use mungos::{by_id::update_one_by_id, mongodb::bson::to_document};
use octorust::types::{
  ReposCreateWebhookRequest, ReposCreateWebhookRequestConfig,
};
use periphery_client::api;
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::{
    git_token, periphery_client,
    update::{add_update, make_update},
  },
  resource,
  state::{action_states, db_client, github_client, State},
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

impl Resolve<RenameRepo, User> for State {
  #[instrument(name = "RenameRepo", skip(self, user))]
  async fn resolve(
    &self,
    RenameRepo { id, name }: RenameRepo,
    user: User,
  ) -> anyhow::Result<Update> {
    let repo = resource::get_check_permissions::<Repo>(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    if repo.config.server_id.is_empty()
      || !repo.config.path.is_empty()
    {
      return resource::rename::<Repo>(&repo.id, &name, &user).await;
    }

    // get the action state for the repo (or insert default).
    let action_state =
      action_states().repo.get_or_insert_default(&repo.id).await;

    // Will check to ensure repo not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.renaming = true)?;

    let name = to_komodo_name(&name);

    let mut update = make_update(&repo, Operation::RenameRepo, &user);

    update_one_by_id(
      &db_client().repos,
      &repo.id,
      mungos::update::Update::Set(
        doc! { "name": &name, "updated_at": komodo_timestamp() },
      ),
      None,
    )
    .await
    .context("Failed to update Repo name on db")?;

    let server =
      resource::get::<Server>(&repo.config.server_id).await?;

    let log = match periphery_client(&server)?
      .request(api::git::RenameRepo {
        curr_name: to_komodo_name(&repo.name),
        new_name: name.clone(),
      })
      .await
      .context("Failed to rename Repo directory on Server")
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "Rename Repo directory failure",
        format_serror(&e.into()),
      ),
    };

    update.logs.push(log);

    update.push_simple_log(
      "Rename Repo",
      format!("Renamed Repo from {} to {}", repo.name, name),
    );
    update.finalize();
    update.id = add_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<RefreshRepoCache, User> for State {
  #[instrument(
    name = "RefreshRepoCache",
    level = "debug",
    skip(self, user)
  )]
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

    if repo.config.git_provider.is_empty()
      || repo.config.repo.is_empty()
    {
      // Nothing to do
      return Ok(NoData {});
    }

    let mut clone_args: CloneArgs = (&repo).into();
    let repo_path =
      clone_args.unique_path(&core_config().repo_directory)?;
    clone_args.destination = Some(repo_path.display().to_string());
    // Don't want to run these on core.
    clone_args.on_clone = None;
    clone_args.on_pull = None;

    let access_token = if let Some(username) = &clone_args.account {
      git_token(&clone_args.provider, username, |https| {
          clone_args.https = https
        })
        .await
        .with_context(
          || format!("Failed to get git token in call to db. Stopping run. | {} | {username}", clone_args.provider),
        )?
    } else {
      None
    };

    let GitRes { hash, message, .. } = git::pull_or_clone(
      clone_args,
      &core_config().repo_directory,
      access_token,
      &[],
      "",
      None,
      &[],
    )
    .await
    .with_context(|| {
      format!("Failed to update repo at {repo_path:?}")
    })?;

    let info = RepoInfo {
      last_pulled_at: repo.info.last_pulled_at,
      last_built_at: repo.info.last_built_at,
      built_hash: repo.info.built_hash,
      built_message: repo.info.built_message,
      latest_hash: hash,
      latest_message: message,
    };

    let info = to_document(&info)
      .context("failed to serialize repo info to bson")?;

    db_client()
      .repos
      .update_one(
        doc! { "name": &repo.name },
        doc! { "$set": { "info": info } },
      )
      .await
      .context("failed to update repo info on db")?;

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

    let webhook_secret = if repo.config.webhook_secret.is_empty() {
      webhook_secret
    } else {
      &repo.config.webhook_secret
    };

    let host = if webhook_base_url.is_empty() {
      host
    } else {
      webhook_base_url
    };
    let url = match action {
      RepoWebhookAction::Clone => {
        format!("{host}/listener/github/repo/{}/clone", repo.id)
      }
      RepoWebhookAction::Pull => {
        format!("{host}/listener/github/repo/{}/pull", repo.id)
      }
      RepoWebhookAction::Build => {
        format!("{host}/listener/github/repo/{}/build", repo.id)
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

    let host = if webhook_base_url.is_empty() {
      host
    } else {
      webhook_base_url
    };
    let url = match action {
      RepoWebhookAction::Clone => {
        format!("{host}/listener/github/repo/{}/clone", repo.id)
      }
      RepoWebhookAction::Pull => {
        format!("{host}/listener/github/repo/{}/pull", repo.id)
      }
      RepoWebhookAction::Build => {
        format!("{host}/listener/github/repo/{}/build", repo.id)
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
