use anyhow::{anyhow, Context};
use formatting::format_serror;
use monitor_client::{
  api::execute::{DeployStack, DestroyStack},
  entities::{
    permission::PermissionLevel,
    server::{Server, ServerState},
    stack::Stack,
    update::Update,
    user::User,
  },
};
use periphery_client::api::compose::{
  ComposeDown, ComposeServiceUp, ComposeUp, ComposeUpResponse,
};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::{
    interpolate_variables_secrets_into_environment, periphery_client,
    stack::{
      deploy::deploy_stack_maybe_service, get_stack_and_server,
      refresh_stack_info, remote::get_remote_compose_file,
    },
    update::update_update,
  },
  monitor::update_cache_for_server,
  state::{action_states, State},
};

impl Resolve<DeployStack, (User, Update)> for State {
  #[instrument(name = "DeployStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DeployStack { stack, stop_time }: DeployStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    deploy_stack_maybe_service(&stack, user, update, None).await
  }
}

impl Resolve<DestroyStack, (User, Update)> for State {
  #[instrument(name = "DestroyStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DestroyStack {
      stack,
      remove_orphans,
      stop_time,
    }: DestroyStack,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (stack, server) = get_stack_and_server(&stack, &user).await?;

    // get the action state for the stack (or insert default).
    let action_state =
      action_states().stack.get_or_insert_default(&stack.id).await;

    // Will check to ensure stack not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.destroying = true)?;

    let file = if let Some(file) = stack.info.deployed_contents {
      file
    } else if stack.config.file_contents.is_empty() {
      let (res, logs, _, _) =
        get_remote_compose_file(&stack)
          .await
          .context("failed to get remote compose file")?;

      update.logs.extend(logs);
      update_update(update.clone()).await?;

      res.context("failed to read remote compose file")?
    } else {
      stack.config.file_contents
    };

    let logs = periphery_client(&server)?
      .request(ComposeDown {
        file,
        remove_orphans,
        timeout: stop_time,
      })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}
