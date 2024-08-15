use anyhow::{anyhow, Context};
use async_timing_util::{wait_until_timelength, Timelength};
use monitor_client::{
  api::write::RefreshStackCache,
  entities::{
    permission::PermissionLevel,
    server::{Server, ServerState},
    stack::Stack,
    user::{stack_user, User},
  },
};
use mungos::find::find_collect;
use regex::Regex;
use resolver_api::Resolve;

use crate::{
  config::core_config,
  resource,
  state::{db_client, State},
};

use super::query::get_server_with_status;

pub mod execute;
pub mod remote;
pub mod services;

pub fn spawn_stack_refresh_loop() {
  let interval: Timelength = core_config()
    .stack_poll_interval
    .try_into()
    .expect("Invalid stack poll interval");
  tokio::spawn(async move {
    refresh_stacks().await;
    loop {
      wait_until_timelength(interval, 3000).await;
      refresh_stacks().await;
    }
  });
}

async fn refresh_stacks() {
  let Ok(stacks) =
    find_collect(&db_client().await.stacks, None, None)
      .await
      .inspect_err(|e| {
        warn!("failed to get stacks from db in refresh task | {e:#}")
      })
  else {
    return;
  };
  for stack in stacks {
    State
      .resolve(
        RefreshStackCache { stack: stack.id },
        stack_user().clone(),
      )
      .await
      .inspect_err(|e| {
        warn!("failed to refresh stack cache in refresh task | stack: {} | {e:#}", stack.name)
      })
      .ok();
  }
}

pub async fn get_stack_and_server(
  stack: &str,
  user: &User,
  permission_level: PermissionLevel,
  block_if_server_unreachable: bool,
) -> anyhow::Result<(Stack, Server)> {
  let stack = resource::get_check_permissions::<Stack>(
    stack,
    user,
    permission_level,
  )
  .await?;

  if stack.config.server_id.is_empty() {
    return Err(anyhow!("Stack has no server configured"));
  }

  let (server, status) =
    get_server_with_status(&stack.config.server_id).await?;
  if block_if_server_unreachable && status != ServerState::Ok {
    return Err(anyhow!(
      "cannot send action when server is unreachable or disabled"
    ));
  }

  Ok((stack, server))
}

pub fn compose_container_match_regex(
  container_name: &str,
) -> anyhow::Result<Regex> {
  let regex = format!("^{container_name}-?[0-9]*$");
  Regex::new(&regex).with_context(|| {
    format!("failed to construct valid regex from {regex}")
  })
}
