use anyhow::{anyhow, Context};
use formatting::format_serror;
use monitor_client::{
  api::execute::{DeployStack, DestroyStack},
  entities::{
    permission::PermissionLevel, server::Server, stack::Stack,
    update::Update, user::User,
  },
};
use periphery_client::api::compose::{ComposeUp, ComposeUpResponse};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::{
    periphery_client, stack::refresh_stack_info,
    update::update_update,
  },
  monitor::update_cache_for_server,
  resource,
  state::State,
};

impl Resolve<DeployStack, (User, Update)> for State {
  #[instrument(name = "DeployStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DeployStack { stack, stop_time }: DeployStack,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let mut stack = resource::get_check_permissions::<Stack>(
      &stack,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    if stack.config.server_id.is_empty() {
      return Err(anyhow!("Stack has no server configured"));
    }

    let server = resource::get::<Server>(&stack.config.server_id)
      .await
      .context("Stack has invalid server set")?;

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
    DestroyStack { stack, stop_time }: DestroyStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    todo!()
  }
}
