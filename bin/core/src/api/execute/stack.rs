use anyhow::{anyhow, Context};
use formatting::format_serror;
use monitor_client::{
  api::execute::{DeployStack, DestroyStack},
  entities::{
    permission::PermissionLevel, server::Server, stack::Stack,
    update::Update, user::User,
  },
};
use periphery_client::api::compose::{
  ComposeDown, ComposeUp, ComposeUpResponse,
};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::{
    periphery_client,
    stack::{refresh_stack_info, remote::get_remote_compose_file},
    update::update_update,
  },
  monitor::update_cache_for_server,
  resource,
  state::State,
};

async fn get_stack_and_server(
  stack: &str,
  user: &User,
) -> anyhow::Result<(Stack, Server)> {
  let stack = resource::get_check_permissions::<Stack>(
    stack,
    user,
    PermissionLevel::Execute,
  )
  .await?;

  if stack.config.server_id.is_empty() {
    return Err(anyhow!("Stack has no server configured"));
  }

  let server = resource::get::<Server>(&stack.config.server_id)
    .await
    .context("Stack has invalid server set")?;

  Ok((stack, server))
}

impl Resolve<DeployStack, (User, Update)> for State {
  #[instrument(name = "DeployStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DeployStack { stack, stop_time }: DeployStack,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (mut stack, server) =
      get_stack_and_server(&stack, &user).await?;

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
          .find(|account| {
            account.username == stack.config.git_account
          })
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

    let ComposeUpResponse {
      logs,
      file_contents,
      commit_hash,
      commit_message,
    } = periphery_client(&server)?
      .request(ComposeUp {
        stack: stack.clone(),
        git_token,
        registry_token,
      })
      .await?;

    update.logs.extend(logs);

    if let Err(e) = refresh_stack_info(
      &stack,
      true,
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
