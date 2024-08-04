use anyhow::{anyhow, Context};
use formatting::format_serror;
use monitor_client::{
  api::execute::*,
  entities::{
    permission::PermissionLevel, stack::StackInfo, update::Update,
    user::User,
  },
};
use mungos::mongodb::bson::{doc, to_document};
use periphery_client::api::compose::*;
use resolver_api::Resolve;

use crate::{
  helpers::{
    interpolate_variables_secrets_into_environment, periphery_client,
    stack::{
      execute::{execute_compose, maybe_timeout},
      get_stack_and_server,
      json::get_config_json,
      services::extract_services,
    },
    update::update_update,
  },
  monitor::update_cache_for_server,
  state::{action_states, db_client, State},
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
    let (mut stack, server) = get_stack_and_server(
      &stack,
      &user,
      PermissionLevel::Execute,
      true,
    )
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

    let update_info = async {
      let (services, json, json_error) = if let Some(contents) =
        &file_contents
      {
        let (json, json_error) = get_config_json(contents).await;
        match extract_services(
          contents,
          &stack.config.run_directory,
          &stack.config.file_path,
        ) {
          Ok(services) => (services, json, json_error),
          Err(e) => {
            update.push_error_log(
            "extract services",
            format_serror(&e.context("Failed to extract stack services. Things probably won't work correctly").into())
          );
            (Vec::new(), json, json_error)
          }
        }
      } else {
        // maybe better to do something else here for services.
        (stack.info.services.clone(), None, None)
      };

      let (
        services,
        deployed_contents,
        deployed_hash,
        deployed_message,
        deployed_json,
        deployed_json_error,
      ) = if deployed {
        (
          services,
          file_contents.clone(),
          commit_hash.clone(),
          commit_message.clone(),
          json.clone(),
          json_error.clone(),
        )
      } else {
        (
          stack.info.services,
          stack.info.deployed_contents,
          stack.info.deployed_hash,
          stack.info.deployed_message,
          stack.info.deployed_json,
          stack.info.deployed_json_error,
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
        latest_hash: commit_hash,
        latest_message: commit_message,
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
        .await
        .context("failed to update stack info on db")?;
      anyhow::Ok(())
    };

    // This will be weird with single service deploys. Come back to it.
    if let Err(e) = update_info.await {
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
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (stack, server) = get_stack_and_server(
      &stack,
      &user,
      PermissionLevel::Execute,
      true,
    )
    .await?;

    // get the action state for the stack (or insert default).
    let action_state =
      action_states().stack.get_or_insert_default(&stack.id).await;

    // Will check to ensure stack not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard = action_state.update(|state| {
      if service.is_none() {
        state.destroying = true
      }
    })?;

    let periphery = periphery_client(&server)?;

    let service = service
      .map(|service| format!(" {service}"))
      .unwrap_or_default();
    let maybe_timeout = maybe_timeout(stop_time);
    let maybe_remove_orphans = if remove_orphans {
      " --remove-orphans"
    } else {
      ""
    };

    let ComposeExecutionResponse { file_missing, log } = periphery
      .request(ComposeExecution {
        name: stack.name,
        run_directory: stack.config.run_directory.clone(),
        file_path: stack.config.file_path.clone(),
        command: format!(
          "down{maybe_timeout}{maybe_remove_orphans}{service}"
        ),
      })
      .await
      .context(
        "failed to bring destroy stack with docker compose down",
      )?;

    if file_missing {
      let services = if stack.info.services.is_empty() {
        let file_contents = if stack.config.file_contents.is_empty() {
          stack.info.remote_contents
        } else {
          Some(stack.config.file_contents)
        };
        let Some(file_contents) = file_contents else {
          return Err(
            anyhow!("The compose file on the host is missing")
              .context("Stack cached services are empty, and cannot get file contents, so do not know which containers to destroy")
          );
        };
        extract_services(
          &file_contents,
          &stack.config.run_directory,
          &stack.config.file_path,
        )
        .context("failed to extract services")?
      } else {
        stack.info.services
      };
      let logs = periphery
        .request(ComposeDestroy { services })
        .await
        .context(
          "failed to destroy stack with docker container rm",
        )?;
      update.logs.extend(logs);
    } else if let Some(log) = log {
      update.logs.push(log);
    }

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}
