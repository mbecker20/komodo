use monitor_client::{
  api::execute::*,
  entities::{
    stack::{Stack, StackActionState},
    update::Update,
    user::User,
  },
};
use periphery_client::{api::compose::*, PeripheryClient};

use crate::{
  helpers::{git_token, periphery_client, update::update_update},
  monitor::update_cache_for_server,
  state::action_states,
};

use super::get_stack_and_server;

pub trait ExecuteCompose {
  type Extras;
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    extras: Self::Extras,
  ) -> anyhow::Result<ComposeResponse>;
}

pub async fn execute_compose<T: ExecuteCompose>(
  stack: &str,
  user: &User,
  set_in_progress: impl Fn(&mut StackActionState),
  mut update: Update,
  extras: T::Extras,
) -> anyhow::Result<Update> {
  let (stack, server) = get_stack_and_server(stack, user).await?;

  // get the action state for the stack (or insert default).
  let action_state =
    action_states().stack.get_or_insert_default(&stack.id).await;

  // Will check to ensure stack not already busy before updating, and return Err if so.
  // The returned guard will set the action state back to default when dropped.
  let _action_guard = action_state.update(set_in_progress)?;

  let git_token =
    git_token(&stack.config.git_provider, &stack.config.git_account);

  let periphery = periphery_client(&server)?;

  let res = T::execute(periphery, stack, git_token, extras).await?;

  update.logs.extend(res.logs);

  // Ensure cached stack state up to date by updating server cache
  update_cache_for_server(&server).await;

  update.finalize();
  update_update(update.clone()).await?;

  Ok(update)
}

impl ExecuteCompose for StartStack {
  type Extras = ();
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    _: Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery.request(ComposeStart { stack, git_token }).await
  }
}

impl ExecuteCompose for RestartStack {
  type Extras = ();
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    _: Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery.request(ComposeRestart { stack, git_token }).await
  }
}

impl ExecuteCompose for PauseStack {
  type Extras = ();
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    _: Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery.request(ComposePause { stack, git_token }).await
  }
}

impl ExecuteCompose for UnpauseStack {
  type Extras = ();
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    _: Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery.request(ComposeUnpause { stack, git_token }).await
  }
}

impl ExecuteCompose for StopStack {
  type Extras = Option<i32>;
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    timeout: Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery
      .request(ComposeStop {
        stack,
        git_token,
        timeout,
      })
      .await
  }
}

impl ExecuteCompose for DestroyStack {
  type Extras = (Option<i32>, bool);
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    (timeout, remove_orphans): Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery
      .request(ComposeDown {
        stack,
        git_token,
        timeout,
        remove_orphans,
      })
      .await
  }
}

impl ExecuteCompose for StartStackService {
  type Extras = String;
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    service: Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery
      .request(ComposeServiceStart { stack, service, git_token })
      .await
  }
}

impl ExecuteCompose for RestartStackService {
  type Extras = String;
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    service: Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery.request(ComposeServiceRestart { stack, service, git_token }).await
  }
}

impl ExecuteCompose for PauseStackService {
  type Extras = String;
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    service: Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery.request(ComposeServicePause { stack, service, git_token }).await
  }
}

impl ExecuteCompose for UnpauseStackService {
  type Extras = String;
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    service: Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery.request(ComposeServiceUnpause { stack, service, git_token }).await
  }
}

impl ExecuteCompose for StopStackService {
  type Extras = (String, Option<i32>);
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    (service, timeout): Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery
      .request(ComposeServiceStop {
        stack,
        service,
        git_token,
        timeout,
      })
      .await
  }
}

impl ExecuteCompose for DestroyStackService {
  type Extras = (String, Option<i32>, bool);
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    git_token: Option<String>,
    (service, timeout, remove_orphans): Self::Extras,
  ) -> anyhow::Result<ComposeResponse> {
    periphery
      .request(ComposeServiceDown {
        stack,
        service,
        git_token,
        timeout,
        remove_orphans,
      })
      .await
  }
}
