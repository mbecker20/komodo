use anyhow::{anyhow, Context};
use async_timing_util::{wait_until_timelength, Timelength};
use formatting::format_serror;
use monitor_client::{
  api::write::RefreshStackCache,
  entities::{
    permission::PermissionLevel,
    server::{Server, ServerState},
    stack::{Stack, StackInfo},
    update::Update,
    user::{stack_user, User},
  },
};
use mungos::{
  find::find_collect,
  mongodb::bson::{doc, to_document},
};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  resource,
  state::{db_client, State},
};

use super::query::get_server_with_status;

pub mod deploy;
pub mod execute;
pub mod json;
pub mod remote;
pub mod services;

pub fn spawn_stack_refresh_loop() {
  let interval: Timelength = core_config()
    .stack_poll_interval
    .try_into()
    .expect("Invalid stack poll interval");
  tokio::spawn(async move {
    let db = db_client().await;
    let user = stack_user();
    loop {
      wait_until_timelength(interval, 3000).await;
      let Ok(stacks) =
        find_collect(&db.stacks, None, None).await.inspect_err(|e| {
          warn!(
            "failed to get stacks from db in refresh task | {e:#}"
          )
        })
      else {
        continue;
      };
      for stack in stacks {
        State
          .resolve(
            RefreshStackCache { stack: stack.id },
            user.clone(),
          )
          .await
          .inspect_err(|e| {
            warn!("failed to refresh stack cache in refresh task | stack: {} | {e:#}", stack.name)
          })
          .ok();
      }
    }
  });
}

pub async fn refresh_stack_info(
  stack: &Stack,
  is_deploy: bool,
  file_missing: bool,
  file_contents: Option<String>,
  remote_error: Option<String>,
  hash: Option<String>,
  message: Option<String>,
  update: Option<&mut Update>,
) -> anyhow::Result<()> {
  let (new_services, json, json_error) = if let Some(contents) =
    &file_contents
  {
    let (json, json_error) = json::get_config_json(contents).await;
    match services::extract_services(contents) {
      Ok(services) => (services, json, json_error),
      Err(e) => {
        if let Some(update) = update {
          update.push_error_log(
            "extract services",
            format_serror(&e.context("Failed to extract stack services. Things probably won't work correctly").into())
          );
        }
        (Vec::new(), json, json_error)
      }
    }
  } else {
    (Vec::new(), None, None)
  };

  let (
    services,
    deployed_contents,
    deployed_hash,
    deployed_message,
    deployed_json,
    deployed_json_error,
  ) = if is_deploy {
    (
      new_services,
      file_contents.clone(),
      hash.clone(),
      message.clone(),
      json.clone(),
      json_error.clone(),
    )
  } else {
    (
      stack.info.services.clone(),
      stack.info.deployed_contents.clone(),
      stack.info.deployed_hash.clone(),
      stack.info.deployed_message.clone(),
      stack.info.deployed_json.clone(),
      stack.info.deployed_json_error.clone(),
    )
  };

  let info = StackInfo {
    file_missing,
    deployed_contents,
    deployed_hash,
    deployed_message,
    deployed_json,
    deployed_json_error,
    services,
    latest_json: json,
    latest_json_error: json_error,
    remote_contents: file_contents.and_then(|contents| {
      // Only store remote contents here (not defined in `file_contents`)
      stack.config.file_contents.is_empty().then_some(contents)
    }),
    remote_error,
    latest_hash: hash,
    latest_message: message,
  };

  let info = to_document(&info)
    .context("failed to serialize stack info to bson")?;

  db_client()
    .await
    .stacks
    .update_one(
      doc! { "name": &stack.name },
      doc! { "$set": { "info": info } },
    )
    .await?;

  Ok(())
}

pub async fn get_stack_and_server(
  stack: &str,
  user: &User,
  permission_level: PermissionLevel,
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
  if status != ServerState::Ok {
    return Err(anyhow!(
      "cannot send action when server is unreachable or disabled"
    ));
  }

  Ok((stack, server))
}
