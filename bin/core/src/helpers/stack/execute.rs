use anyhow::anyhow;
use monitor_client::{
  api::execute::*,
  entities::{
    permission::PermissionLevel,
    stack::{Stack, StackActionState},
    update::Update,
    user::User,
  },
};
use periphery_client::{api::compose::*, PeripheryClient};

use crate::{
  helpers::{periphery_client, update::update_update},
  monitor::update_cache_for_server,
  state::action_states,
};

use super::get_stack_and_server;

pub trait ExecuteCompose {
  type Extras;
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    service: Option<String>,
    extras: Self::Extras,
  ) -> anyhow::Result<ComposeExecutionResponse>;
}

pub async fn execute_compose<T: ExecuteCompose>(
  stack: &str,
  service: Option<String>,
  user: &User,
  set_in_progress: impl Fn(&mut StackActionState),
  mut update: Update,
  extras: T::Extras,
) -> anyhow::Result<Update> {
  let (stack, server) =
    get_stack_and_server(stack, user, PermissionLevel::Execute, true)
      .await?;

  // get the action state for the stack (or insert default).
  let action_state =
    action_states().stack.get_or_insert_default(&stack.id).await;

  // Will check to ensure stack not already busy before updating, and return Err if so.
  // The returned guard will set the action state back to default when dropped.
  let _action_guard = action_state.update(set_in_progress)?;

  let periphery = periphery_client(&server)?;

  let ComposeExecutionResponse { file_missing, log } =
    T::execute(periphery, stack, service, extras).await?;
  if let Some(log) = log {
    update.logs.push(log);
  }
  if file_missing {
    return Err(anyhow!("Compose file is missing on Periphery. Redeploy the stack to fix."));
  }

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
    service: Option<String>,
    _: Self::Extras,
  ) -> anyhow::Result<ComposeExecutionResponse> {
    let service = service
      .map(|service| format!(" {service}"))
      .unwrap_or_default();
    periphery
      .request(ComposeExecution {
        name: stack.name,
        run_directory: stack.config.run_directory,
        file_path: stack.config.file_path,
        command: format!("start{service}"),
      })
      .await
  }
}

impl ExecuteCompose for RestartStack {
  type Extras = ();
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    service: Option<String>,
    _: Self::Extras,
  ) -> anyhow::Result<ComposeExecutionResponse> {
    let service = service
      .map(|service| format!(" {service}"))
      .unwrap_or_default();
    periphery
      .request(ComposeExecution {
        name: stack.name,
        run_directory: stack.config.run_directory,
        file_path: stack.config.file_path,
        command: format!("restart{service}"),
      })
      .await
  }
}

impl ExecuteCompose for PauseStack {
  type Extras = ();
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    service: Option<String>,
    _: Self::Extras,
  ) -> anyhow::Result<ComposeExecutionResponse> {
    let service = service
      .map(|service| format!(" {service}"))
      .unwrap_or_default();
    periphery
      .request(ComposeExecution {
        name: stack.name,
        run_directory: stack.config.run_directory,
        file_path: stack.config.file_path,
        command: format!("pause{service}"),
      })
      .await
  }
}

impl ExecuteCompose for UnpauseStack {
  type Extras = ();
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    service: Option<String>,
    _: Self::Extras,
  ) -> anyhow::Result<ComposeExecutionResponse> {
    let service = service
      .map(|service| format!(" {service}"))
      .unwrap_or_default();
    periphery
      .request(ComposeExecution {
        name: stack.name,
        run_directory: stack.config.run_directory,
        file_path: stack.config.file_path,
        command: format!("unpause{service}"),
      })
      .await
  }
}

impl ExecuteCompose for StopStack {
  type Extras = Option<i32>;
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    service: Option<String>,
    timeout: Self::Extras,
  ) -> anyhow::Result<ComposeExecutionResponse> {
    let service = service
      .map(|service| format!(" {service}"))
      .unwrap_or_default();
    let maybe_timeout = maybe_timeout(timeout);
    periphery
      .request(ComposeExecution {
        name: stack.name,
        run_directory: stack.config.run_directory,
        file_path: stack.config.file_path,
        command: format!("stop{maybe_timeout}{service}"),
      })
      .await
  }
}

impl ExecuteCompose for DestroyStack {
  type Extras = (Option<i32>, bool);
  async fn execute(
    periphery: PeripheryClient,
    stack: Stack,
    service: Option<String>,
    (timeout, remove_orphans): Self::Extras,
  ) -> anyhow::Result<ComposeExecutionResponse> {
    let service = service
      .map(|service| format!(" {service}"))
      .unwrap_or_default();
    let maybe_timeout = maybe_timeout(timeout);
    let maybe_remove_orphans = if remove_orphans {
      " --remove-orphans"
    } else {
      ""
    };
    periphery
      .request(ComposeExecution {
        name: stack.name,
        run_directory: stack.config.run_directory,
        file_path: stack.config.file_path,
        command: format!(
          "down{maybe_timeout}{maybe_remove_orphans}{service}"
        ),
      })
      .await
  }
}

fn maybe_timeout(timeout: Option<i32>) -> String {
  if let Some(timeout) = timeout {
    format!(" --timeout {timeout}")
  } else {
    String::new()
  }
}
