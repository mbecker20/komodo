use std::collections::HashSet;

use anyhow::Context;
use formatting::format_serror;
use komodo_client::{
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
    interpolate::{
      add_interp_update_log,
      interpolate_variables_secrets_into_extra_args,
      interpolate_variables_secrets_into_string,
      interpolate_variables_secrets_into_system_command,
    },
    periphery_client,
    query::get_variables_and_secrets,
    update::update_update,
  },
  monitor::update_cache_for_server,
  stack::{
    execute::execute_compose, get_stack_and_server,
    services::extract_services_into_res,
  },
  state::{action_states, db_client, State},
};

impl Resolve<DeployStack, (User, Update)> for State {
  #[instrument(name = "DeployStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DeployStack { stack, stop_time }: DeployStack,
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

    update_update(update.clone()).await?;

    let git_token = crate::helpers::git_token(
      &stack.config.git_provider,
      &stack.config.git_account,
      |https| stack.config.git_https = https,
    ).await.with_context(
      || format!("Failed to get git token in call to db. Stopping run. | {} | {}", stack.config.git_provider, stack.config.git_account),
    )?;

    let registry_token = crate::helpers::registry_token(
      &stack.config.registry_provider,
      &stack.config.registry_account,
    ).await.with_context(
      || format!("Failed to get registry token in call to db. Stopping run. | {} | {}", stack.config.registry_provider, stack.config.registry_account),
    )?;

    // interpolate variables / secrets, returning the sanitizing replacers to send to
    // periphery so it may sanitize the final command for safe logging (avoids exposing secret values)
    let secret_replacers = if !stack.config.skip_secret_interp {
      let vars_and_secrets = get_variables_and_secrets().await?;

      let mut global_replacers = HashSet::new();
      let mut secret_replacers = HashSet::new();

      interpolate_variables_secrets_into_string(
        &vars_and_secrets,
        &mut stack.config.environment,
        &mut global_replacers,
        &mut secret_replacers,
      )?;

      interpolate_variables_secrets_into_extra_args(
        &vars_and_secrets,
        &mut stack.config.extra_args,
        &mut global_replacers,
        &mut secret_replacers,
      )?;

      interpolate_variables_secrets_into_extra_args(
        &vars_and_secrets,
        &mut stack.config.build_extra_args,
        &mut global_replacers,
        &mut secret_replacers,
      )?;

      interpolate_variables_secrets_into_system_command(
        &vars_and_secrets,
        &mut stack.config.pre_deploy,
        &mut global_replacers,
        &mut secret_replacers,
      )?;

      add_interp_update_log(
        &mut update,
        &global_replacers,
        &secret_replacers,
      );

      secret_replacers
    } else {
      Default::default()
    };

    let ComposeUpResponse {
      logs,
      deployed,
      file_contents,
      missing_files,
      remote_errors,
      commit_hash,
      commit_message,
    } = periphery_client(&server)?
      .request(ComposeUp {
        stack: stack.clone(),
        service: None,
        git_token,
        registry_token,
        replacers: secret_replacers.into_iter().collect(),
      })
      .await?;

    update.logs.extend(logs);

    let update_info = async {
      let latest_services = if !file_contents.is_empty() {
        let mut services = Vec::new();
        for contents in &file_contents {
          if let Err(e) = extract_services_into_res(
            &stack.project_name(true),
            &contents.contents,
            &mut services,
          ) {
            update.push_error_log(
              "extract services",
              format_serror(&e.context(format!("Failed to extract stack services for compose file path {}. Things probably won't work correctly", contents.path)).into())
            );
          }
        }
        services
      } else {
        // maybe better to do something else here for services.
        stack.info.latest_services.clone()
      };

      // This ensures to get the latest project name,
      // as it may have changed since the last deploy.
      let project_name = stack.project_name(true);

      let (
        deployed_services,
        deployed_contents,
        deployed_hash,
        deployed_message,
      ) = if deployed {
        (
          Some(latest_services.clone()),
          Some(file_contents.clone()),
          commit_hash.clone(),
          commit_message.clone(),
        )
      } else {
        (
          stack.info.deployed_services,
          stack.info.deployed_contents,
          stack.info.deployed_hash,
          stack.info.deployed_message,
        )
      };

      let info = StackInfo {
        missing_files,
        deployed_project_name: project_name.into(),
        deployed_services,
        deployed_contents,
        deployed_hash,
        deployed_message,
        latest_services,
        remote_contents: stack
          .config
          .file_contents
          .is_empty()
          .then_some(file_contents),
        remote_errors: stack
          .config
          .file_contents
          .is_empty()
          .then_some(remote_errors),
        latest_hash: commit_hash,
        latest_message: commit_message,
      };

      let info = to_document(&info)
        .context("failed to serialize stack info to bson")?;

      db_client()
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
    execute_compose::<StartStack>(
      &stack,
      service,
      &user,
      |state| state.starting = true,
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
    execute_compose::<RestartStack>(
      &stack,
      service,
      &user,
      |state| {
        state.restarting = true;
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
    execute_compose::<PauseStack>(
      &stack,
      service,
      &user,
      |state| state.pausing = true,
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
    execute_compose::<UnpauseStack>(
      &stack,
      service,
      &user,
      |state| state.unpausing = true,
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
    execute_compose::<StopStack>(
      &stack,
      service,
      &user,
      |state| state.stopping = true,
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
    }: DestroyStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<DestroyStack>(
      &stack,
      None,
      &user,
      |state| state.destroying = true,
      update,
      (stop_time, remove_orphans),
    )
    .await
  }
}
