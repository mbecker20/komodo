use formatting::format_serror;
use monitor_client::entities::{update::Update, user::User};
use periphery_client::api::compose::{
  ComposeServiceUp, ComposeUp, ComposeUpResponse,
};

use crate::{
  config::core_config,
  helpers::{
    interpolate_variables_secrets_into_environment, periphery_client,
    update::update_update,
  },
  monitor::update_cache_for_server,
  state::action_states,
};

use super::{get_stack_and_server, refresh_stack_info};

pub async fn deploy_stack_maybe_service(
  stack: &str,
  user: User,
  mut update: Update,
  service: Option<String>,
) -> anyhow::Result<Update> {
  let (mut stack, server) =
    get_stack_and_server(stack, &user).await?;

  // get the action state for the stack (or insert default).
  let action_state =
    action_states().stack.get_or_insert_default(&stack.id).await;

  // Will check to ensure stack not already busy before updating, and return Err if so.
  // The returned guard will set the action state back to default when dropped.
  let _action_guard =
    action_state.update(|state| state.deploying = true)?;

  let core_config = core_config();

  let git_token = core_config
    .git_providers
    .iter()
    .find(|provider| provider.domain == stack.config.git_provider)
    .and_then(|provider| {
      stack.config.git_https = provider.https;
      provider
        .accounts
        .iter()
        .find(|account| account.username == stack.config.git_account)
        .map(|account| account.token.clone())
    });

  let registry_token = core_config
    .docker_registries
    .iter()
    .find(|provider| {
      provider.domain == stack.config.registry_provider
    })
    .and_then(|provider| {
      provider
        .accounts
        .iter()
        .find(|account| {
          account.username == stack.config.registry_account
        })
        .map(|account| account.token.clone())
    });

  if !stack.config.skip_secret_interp {
    interpolate_variables_secrets_into_environment(
      &mut stack.config.environment,
      &mut update,
    )
    .await?;
  }

  let periphery = periphery_client(&server)?;

  let ComposeUpResponse {
    logs,
    deployed,
    file_contents,
    commit_hash,
    commit_message,
  } = match service {
    Some(service) => {
      periphery
        .request(ComposeServiceUp {
          stack: stack.clone(),
          git_token,
          registry_token,
          service,
        })
        .await?
    }
    None => {
      periphery_client(&server)?
        .request(ComposeUp {
          stack: stack.clone(),
          git_token,
          registry_token,
        })
        .await?
    }
  };

  update.logs.extend(logs);

  if let Err(e) = refresh_stack_info(
    &stack,
    deployed,
    file_contents,
    commit_hash,
    commit_message,
    Some(&mut update),
  )
  .await
  {
    update.push_error_log(
      "refresh stack info",
      format_serror(
        &e.context("failed to refresh stack info on db").into(),
      ),
    )
  }

  // Ensure cached stack state up to date by updating server cache
  update_cache_for_server(&server).await;

  update.finalize();
  update_update(update.clone()).await?;

  Ok(update)
}
