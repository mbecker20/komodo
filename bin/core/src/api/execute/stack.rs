use formatting::format_serror;
use monitor_client::{
  api::execute::*,
  entities::{
    permission::PermissionLevel, update::Update, user::User,
  },
};
use periphery_client::api::compose::{ComposeUp, ComposeUpResponse};
use resolver_api::Resolve;

use crate::{
  helpers::{
    interpolate_variables_secrets_into_environment, periphery_client,
    stack::{
      execute::execute_compose, get_stack_and_server,
      refresh_stack_info,
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
    DeployStack {
      stack,
      stop_time,
      service,
    }: DeployStack,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (mut stack, server) =
      get_stack_and_server(&stack, &user, PermissionLevel::Execute)
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
      deployed,
      file_missing,
      file_contents,
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
      deployed,
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
}

impl Resolve<StartStack, (User, Update)> for State {
  #[instrument(name = "StartStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StartStack { stack, service }: StartStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    let no_service = service.is_none();
    execute_compose::<StartStack>(
      &stack,
      service,
      &user,
      |state| {
        if no_service {
          state.starting = true
        }
      },
      update,
      (),
    )
    .await
  }
}

impl Resolve<RestartStack, (User, Update)> for State {
  #[instrument(name = "RestartStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RestartStack { stack, service }: RestartStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    let no_service = service.is_none();
    execute_compose::<RestartStack>(
      &stack,
      service,
      &user,
      |state| {
        if no_service {
          state.restarting = true;
        }
      },
      update,
      (),
    )
    .await
  }
}

impl Resolve<PauseStack, (User, Update)> for State {
  #[instrument(name = "PauseStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PauseStack { stack, service }: PauseStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    let no_service = service.is_none();
    execute_compose::<PauseStack>(
      &stack,
      service,
      &user,
      |state| {
        if no_service {
          state.pausing = true
        }
      },
      update,
      (),
    )
    .await
  }
}

impl Resolve<UnpauseStack, (User, Update)> for State {
  #[instrument(name = "UnpauseStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    UnpauseStack { stack, service }: UnpauseStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    let no_service = service.is_none();
    execute_compose::<UnpauseStack>(
      &stack,
      service,
      &user,
      |state| {
        if no_service {
          state.unpausing = true
        }
      },
      update,
      (),
    )
    .await
  }
}

impl Resolve<StopStack, (User, Update)> for State {
  #[instrument(name = "StopStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StopStack {
      stack,
      stop_time,
      service,
    }: StopStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    let no_service = service.is_none();
    execute_compose::<StopStack>(
      &stack,
      service,
      &user,
      |state| {
        if no_service {
          state.stopping = true
        }
      },
      update,
      stop_time,
    )
    .await
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
      service,
    }: DestroyStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    let no_service = service.is_none();
    execute_compose::<DestroyStack>(
      &stack,
      service,
      &user,
      |state| {
        if no_service {
          state.destroying = true
        }
      },
      update,
      (stop_time, remove_orphans),
    )
    .await
  }
}
