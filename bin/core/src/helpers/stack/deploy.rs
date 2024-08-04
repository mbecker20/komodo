use formatting::format_serror;
use monitor_client::entities::{
  permission::PermissionLevel, update::Update, user::User,
};
use periphery_client::api::compose::{ComposeUp, ComposeUpResponse};

use crate::{
  helpers::{
    interpolate_variables_secrets_into_environment, periphery_client,
    update::update_update,
  },
  monitor::update_cache_for_server,
  state::action_states,
};

use super::{get_stack_and_server, refresh_stack_info};

pub async fn deploy_stack(
  stack: &str,
  service: Option<String>,
  user: User,
  mut update: Update,
) -> anyhow::Result<Update> {
  let (mut stack, server) =
    get_stack_and_server(stack, &user, PermissionLevel::Execute)
      .await?;

  // get the action state for the stack (or insert default).
  let action_state =
    action_states().stack.get_or_insert_default(&stack.id).await;

  // Will check to ensure stack not already busy before updating, and return Err if so.
  // The returned guard will set the action state back to default when dropped.
  let _action_guard =
    action_state.update(|state| state.deploying = true)?;

  let git_token = crate::helpers::git_token(
    &stack.config.git_provider,
    &stack.config.git_account,
  );

  let registry_token = crate::helpers::registry_token(
    &stack.config.registry_provider,
    &stack.config.registry_account,
  );

  if !stack.config.skip_secret_interp {
    interpolate_variables_secrets_into_environment(
      &mut stack.config.environment,
      &mut update,
    )
    .await?;
  }

  let ComposeUpResponse {
    logs,
    deployed: is_deploy,
    file_contents,
    file_missing,
    remote_error,
    commit_hash,
    commit_message,
  } = periphery_client(&server)?
    .request(ComposeUp {
      stack: stack.clone(),
      service,
      git_token,
      registry_token,
    })
    .await?;

  update.logs.extend(logs);

  // This will be weird with single service deploys. Come back to it.
  if let Err(e) = refresh_stack_info(
    &stack,
    is_deploy,
    file_missing,
    file_contents,
    remote_error,
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
