use anyhow::Context;
use monitor_client::{
  api::read::*,
  entities::{
    config::core::CoreConfig,
    permission::PermissionLevel,
    repo::{Repo, RepoActionState, RepoListItem, RepoState},
    user::User,
  },
};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  resource,
  state::{action_states, github_client, repo_state_cache, State},
};

impl Resolve<GetRepo, User> for State {
  async fn resolve(
    &self,
    GetRepo { repo }: GetRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    resource::get_check_permissions::<Repo>(
      &repo,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

impl Resolve<ListRepos, User> for State {
  async fn resolve(
    &self,
    ListRepos { query }: ListRepos,
    user: User,
  ) -> anyhow::Result<Vec<RepoListItem>> {
    resource::list_for_user::<Repo>(query, &user).await
  }
}

impl Resolve<ListFullRepos, User> for State {
  async fn resolve(
    &self,
    ListFullRepos { query }: ListFullRepos,
    user: User,
  ) -> anyhow::Result<ListFullReposResponse> {
    resource::list_full_for_user::<Repo>(query, &user).await
  }
}

impl Resolve<GetRepoActionState, User> for State {
  async fn resolve(
    &self,
    GetRepoActionState { repo }: GetRepoActionState,
    user: User,
  ) -> anyhow::Result<RepoActionState> {
    let repo = resource::get_check_permissions::<Repo>(
      &repo,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .repo
      .get(&repo.id)
      .await
      .unwrap_or_default()
      .get()?;
    Ok(action_state)
  }
}

impl Resolve<GetReposSummary, User> for State {
  async fn resolve(
    &self,
    GetReposSummary {}: GetReposSummary,
    user: User,
  ) -> anyhow::Result<GetReposSummaryResponse> {
    let repos =
      resource::list_full_for_user::<Repo>(Default::default(), &user)
        .await
        .context("failed to get repos from db")?;

    let mut res = GetReposSummaryResponse::default();

    let cache = repo_state_cache();
    let action_states = action_states();

    for repo in repos {
      res.total += 1;

      match (
        cache.get(&repo.id).await.unwrap_or_default(),
        action_states
          .repo
          .get(&repo.id)
          .await
          .unwrap_or_default()
          .get()?,
      ) {
        (_, action_states) if action_states.cloning => {
          res.cloning += 1;
        }
        (_, action_states) if action_states.pulling => {
          res.pulling += 1;
        }
        (RepoState::Ok, _) => res.ok += 1,
        (RepoState::Failed, _) => res.failed += 1,
        (RepoState::Unknown, _) => res.unknown += 1,
        // will never come off the cache in the building state, since that comes from action states
        (RepoState::Cloning, _) | (RepoState::Pulling, _) => {
          unreachable!()
        }
      }
    }

    Ok(res)
  }
}

impl Resolve<GetRepoWebhooksEnabled, User> for State {
  async fn resolve(
    &self,
    GetRepoWebhooksEnabled { repo }: GetRepoWebhooksEnabled,
    user: User,
  ) -> anyhow::Result<GetRepoWebhooksEnabledResponse> {
    let Some(github) = github_client() else {
      return Ok(GetRepoWebhooksEnabledResponse {
        managed: false,
        clone_enabled: false,
        pull_enabled: false,
      });
    };

    let repo = resource::get_check_permissions::<Repo>(
      &repo,
      &user,
      PermissionLevel::Read,
    )
    .await?;

    if repo.config.repo.is_empty() {
      return Ok(GetRepoWebhooksEnabledResponse {
        managed: false,
        clone_enabled: false,
        pull_enabled: false,
      });
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
      return Ok(GetRepoWebhooksEnabledResponse {
        managed: false,
        clone_enabled: false,
        pull_enabled: false,
      });
    }

    let repo_name =
      split.next().context("Repo repo has no repo after the /")?;

    let github_repos = github.repos();
    let webhooks = github_repos
      .list_all_webhooks(owner, repo_name)
      .await
      .context("failed to list all webhooks on repo")?
      .body;

    let host = github_webhook_base_url.as_ref().unwrap_or(host);
    let clone_url =
      format!("{host}/listener/github/repo/{}/clone", repo.id);
    let pull_url =
      format!("{host}/listener/github/repo/{}/pull", repo.id);

    let mut clone_enabled = false;
    let mut pull_enabled = false;

    for webhook in webhooks {
      if webhook.active && webhook.config.url == clone_url {
        clone_enabled = true
      }
      if webhook.active && webhook.config.url == pull_url {
        pull_enabled = true
      }
    }

    Ok(GetRepoWebhooksEnabledResponse {
      managed: true,
      clone_enabled,
      pull_enabled,
    })
  }
}
