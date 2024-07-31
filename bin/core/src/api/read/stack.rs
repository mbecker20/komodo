use anyhow::Context;
use monitor_client::{
  api::read::*,
  entities::{
    config::core::CoreConfig,
    permission::PermissionLevel,
    stack::{Stack, StackActionState, StackListItem, StackState},
    user::User,
  },
};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  resource,
  state::{action_states, github_client, State},
};

impl Resolve<GetStack, User> for State {
  async fn resolve(
    &self,
    GetStack { stack }: GetStack,
    user: User,
  ) -> anyhow::Result<Stack> {
    resource::get_check_permissions::<Stack>(
      &stack,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

impl Resolve<ListStacks, User> for State {
  async fn resolve(
    &self,
    ListStacks { query }: ListStacks,
    user: User,
  ) -> anyhow::Result<Vec<StackListItem>> {
    resource::list_for_user::<Stack>(query, &user).await
  }
}

impl Resolve<ListFullStacks, User> for State {
  async fn resolve(
    &self,
    ListFullStacks { query }: ListFullStacks,
    user: User,
  ) -> anyhow::Result<ListFullStacksResponse> {
    resource::list_full_for_user::<Stack>(query, &user).await
  }
}

impl Resolve<GetStackActionState, User> for State {
  async fn resolve(
    &self,
    GetStackActionState { stack }: GetStackActionState,
    user: User,
  ) -> anyhow::Result<StackActionState> {
    let stack = resource::get_check_permissions::<Stack>(
      &stack,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .stack
      .get(&stack.id)
      .await
      .unwrap_or_default()
      .get()?;
    Ok(action_state)
  }
}

impl Resolve<GetStacksSummary, User> for State {
  async fn resolve(
    &self,
    GetStacksSummary {}: GetStacksSummary,
    user: User,
  ) -> anyhow::Result<GetStacksSummaryResponse> {
    let stacks = resource::list_full_for_user::<Stack>(
      Default::default(),
      &user,
    )
    .await
    .context("failed to get stacks from db")?;

    let mut res = GetStacksSummaryResponse::default();

    // let cache = resource_sync_state_cache();
    // let action_states = action_states();

    // for stack in stacks {
    //   res.total += 1;

    //   match (
    //     cache.get(&stack.id).await.unwrap_or_default(),
    //     action_states
    //       .stack
    //       .get(&stack.id)
    //       .await
    //       .unwrap_or_default()
    //       .get()?,
    //   ) {
    //     (_, action_states) if action_states.syncing => {
    //       res.syncing += 1;
    //     }
    //     (StackState::Up, _) => res.ok += 1,
    //     (StackState::Failed, _) => res.failed += 1,
    //     (StackState::Unknown, _) => res.unknown += 1,
    //     // will never come off the cache in the building state, since that comes from action states
    //     (StackState::Syncing, _) => {
    //       unreachable!()
    //     }
    //     (StackState::Pending, _) => {
    //       unreachable!()
    //     }
    //   }
    // }

    // Ok(res)

    todo!()
  }
}

impl Resolve<GetStackWebhooksEnabled, User> for State {
  async fn resolve(
    &self,
    GetStackWebhooksEnabled { stack }: GetStackWebhooksEnabled,
    user: User,
  ) -> anyhow::Result<GetStackWebhooksEnabledResponse> {
    let Some(github) = github_client() else {
      return Ok(GetStackWebhooksEnabledResponse {
        managed: false,
        refresh_enabled: false,
        deploy_enabled: false,
      });
    };

    let stack = resource::get_check_permissions::<Stack>(
      &stack,
      &user,
      PermissionLevel::Read,
    )
    .await?;

    if stack.config.git_provider != "github.com"
      || stack.config.repo.is_empty()
    {
      return Ok(GetStackWebhooksEnabledResponse {
        managed: false,
        refresh_enabled: false,
        deploy_enabled: false,
      });
    }

    let mut split = stack.config.repo.split('/');
    let owner = split.next().context("Sync repo has no owner")?;

    let Some(github) = github.get(owner) else {
      return Ok(GetStackWebhooksEnabledResponse {
        managed: false,
        refresh_enabled: false,
        deploy_enabled: false,
      });
    };

    let repo_name =
      split.next().context("Repo repo has no repo after the /")?;

    let github_repos = github.repos();

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
    let refresh_url =
      format!("{host}/listener/github/stack/{}/refresh", stack.id);
    let deploy_url =
      format!("{host}/listener/github/stack/{}/deploy", stack.id);

    let mut refresh_enabled = false;
    let mut deploy_enabled = false;

    for webhook in webhooks {
      if webhook.active && webhook.config.url == refresh_url {
        refresh_enabled = true
      }
      if webhook.active && webhook.config.url == deploy_url {
        deploy_enabled = true
      }
    }

    Ok(GetStackWebhooksEnabledResponse {
      managed: true,
      refresh_enabled,
      deploy_enabled,
    })
  }
}
