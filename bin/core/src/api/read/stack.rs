use std::{collections::HashSet, path::PathBuf};

use anyhow::Context;
use monitor_client::{
  api::read::*,
  entities::{
    config::core::CoreConfig,
    deployment::ContainerSummary,
    permission::PermissionLevel,
    stack::{Stack, StackActionState, StackListItem, StackState},
    to_monitor_name,
    user::User,
  },
};
use periphery_client::api::compose::{
  GetComposeServiceLog, GetComposeServiceLogSearch,
};
use resolver_api::{Resolve, ResolveToString};

use crate::{
  config::core_config,
  helpers::{periphery_client, stack::get_stack_and_server},
  resource,
  state::{action_states, github_client, stack_status_cache, State},
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

impl ResolveToString<GetStackJson, User> for State {
  async fn resolve_to_string(
    &self,
    GetStackJson { stack }: GetStackJson,
    user: User,
  ) -> anyhow::Result<String> {
    let stack = resource::get_check_permissions::<Stack>(
      &stack,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let deployed_json =
      stack.info.deployed_json.as_deref().unwrap_or("null");
    let deployed_error =
      stack.info.deployed_json_error.as_deref().unwrap_or("null");
    let latest_json =
      stack.info.latest_json.as_deref().unwrap_or("null");
    let latest_error =
      stack.info.latest_json_error.as_deref().unwrap_or("null");
    let res = format!(
      "{{\"deployed_json\":{deployed_json},\"deployed_error\":{deployed_error},\"latest_json\":{latest_json},\"latest_error\":{latest_error}}}",
    );
    Ok(res)
  }
}

impl Resolve<GetStackContainers, User> for State {
  async fn resolve(
    &self,
    GetStackContainers { stack }: GetStackContainers,
    user: User,
  ) -> anyhow::Result<Vec<ContainerSummary>> {
    let stack = resource::get_check_permissions::<Stack>(
      &stack,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let containers = stack_status_cache()
      .get(&stack.id)
      .await
      .unwrap_or_default()
      .curr
      .containers
      .clone();
    Ok(containers)
  }
}

impl Resolve<GetStackServiceLog, User> for State {
  async fn resolve(
    &self,
    GetStackServiceLog {
      stack,
      service,
      tail,
    }: GetStackServiceLog,
    user: User,
  ) -> anyhow::Result<GetStackServiceLogResponse> {
    let (stack, server) =
      get_stack_and_server(&stack, &user, PermissionLevel::Read)
        .await?;
    let periphery = periphery_client(&server)?;
    let run_directory = to_monitor_name(&stack.name)
      .parse::<PathBuf>()
      .context("stack name is not valid path")?
      .join(&stack.config.run_directory);
    periphery
      .request(GetComposeServiceLog {
        run_directory,
        file_path: stack.config.file_path,
        service,
        tail,
      })
      .await
      .context("failed to get stack service log from periphery")
  }
}

impl Resolve<SearchStackServiceLog, User> for State {
  async fn resolve(
    &self,
    SearchStackServiceLog {
      stack,
      service,
      terms,
      combinator,
      invert,
    }: SearchStackServiceLog,
    user: User,
  ) -> anyhow::Result<SearchStackServiceLogResponse> {
    let (stack, server) =
      get_stack_and_server(&stack, &user, PermissionLevel::Read)
        .await?;
    let periphery = periphery_client(&server)?;
    let run_directory = to_monitor_name(&stack.name)
      .parse::<PathBuf>()
      .context("stack name is not valid path")?
      .join(&stack.config.run_directory);
    periphery
      .request(GetComposeServiceLogSearch {
        run_directory,
        file_path: stack.config.file_path,
        service,
        terms,
        combinator,
        invert,
      })
      .await
      .context("failed to get stack service log from periphery")
  }
}

impl Resolve<ListCommonStackExtraArgs, User> for State {
  async fn resolve(
    &self,
    ListCommonStackExtraArgs { query }: ListCommonStackExtraArgs,
    user: User,
  ) -> anyhow::Result<ListCommonStackExtraArgsResponse> {
    let stacks = resource::list_full_for_user::<Stack>(query, &user)
      .await
      .context("failed to get resources matching query")?;

    // first collect with guaranteed uniqueness
    let mut res = HashSet::<String>::new();

    for stack in stacks {
      for extra_arg in stack.config.extra_args {
        res.insert(extra_arg);
      }
    }

    let mut res = res.into_iter().collect::<Vec<_>>();
    res.sort();
    Ok(res)
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

    let cache = stack_status_cache();

    for stack in stacks {
      res.total += 1;
      match cache.get(&stack.id).await.unwrap_or_default().curr.state
      {
        StackState::Healthy => res.healthy += 1,
        StackState::Unhealthy => res.unhealthy += 1,
        StackState::Down => res.down += 1,
        StackState::Unknown => res.unknown += 1,
      }
    }

    Ok(res)
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
