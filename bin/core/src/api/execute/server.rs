use anyhow::{anyhow, Context};
use formatting::format_serror;
use monitor_client::{
  api::execute::*,
  entities::{
    all_logs_success, monitor_timestamp,
    permission::PermissionLevel,
    server::{Server, ServerState},
    update::{Log, Update, UpdateStatus},
    user::User,
  },
};
use periphery_client::api;
use resolver_api::Resolve;

use crate::{
  helpers::{
    periphery_client, query::get_server_with_status,
    update::update_update,
  },
  resource,
  state::{action_states, State},
};

impl Resolve<StopAllContainers, (User, Update)> for State {
  #[instrument(name = "StopAllContainers", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StopAllContainers { server }: StopAllContainers,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, status) = get_server_with_status(&server).await?;
    if status != ServerState::Ok {
      return Err(anyhow!(
        "cannot send action when server is unreachable or disabled"
      ));
    }

    // get the action state for the server (or insert default).
    let action_state = action_states()
      .server
      .get_or_insert_default(&server.id)
      .await;

    // Will check to ensure server not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard = action_state
      .update(|state| state.stopping_containers = true)?;

    let logs = periphery_client(&server)?
      .request(api::container::StopAllContainers {})
      .await
      .context("failed to stop all container on host")?;

    update.logs.extend(logs);

    if all_logs_success(&update.logs) {
      update.push_simple_log("stop all containers", String::from("All containers have successfully been stopped on the host."));
    }

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<PruneContainers, (User, Update)> for State {
  #[instrument(name = "PruneContainers", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PruneContainers { server }: PruneContainers,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // get the action state for the server (or insert default).
    let action_state = action_states()
      .server
      .get_or_insert_default(&server.id)
      .await;

    // Will check to ensure server not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.pruning_containers = true)?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::PruneContainers {})
      .await
      .context(format!(
        "failed to prune containers on server {}",
        server.name
      )) {
      Ok(log) => log,
      Err(e) => Log::error(
        "prune containers",
        format_serror(
          &e.context("failed to prune containers").into(),
        ),
      ),
    };

    update.success = log.success;
    update.status = UpdateStatus::Complete;
    update.end_ts = Some(monitor_timestamp());
    update.logs.push(log);

    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<PruneNetworks, (User, Update)> for State {
  #[instrument(name = "PruneNetworks", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PruneNetworks { server }: PruneNetworks,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // get the action state for the server (or insert default).
    let action_state = action_states()
      .server
      .get_or_insert_default(&server.id)
      .await;

    // Will check to ensure server not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.pruning_networks = true)?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::network::PruneNetworks {})
      .await
      .context(format!(
        "failed to prune networks on server {}",
        server.name
      )) {
      Ok(log) => log,
      Err(e) => Log::error(
        "prune networks",
        format_serror(&e.context("failed to prune networks").into()),
      ),
    };

    update.success = log.success;
    update.status = UpdateStatus::Complete;
    update.end_ts = Some(monitor_timestamp());
    update.logs.push(log);

    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<PruneImages, (User, Update)> for State {
  #[instrument(name = "PruneImages", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PruneImages { server }: PruneImages,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // get the action state for the server (or insert default).
    let action_state = action_states()
      .server
      .get_or_insert_default(&server.id)
      .await;

    // Will check to ensure server not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.pruning_images = true)?;

    let periphery = periphery_client(&server)?;

    let log =
      match periphery.request(api::build::PruneImages {}).await {
        Ok(log) => log,
        Err(e) => Log::error(
          "prune images",
          format!(
            "failed to prune images on server {} | {e:#?}",
            server.name
          ),
        ),
      };

    update.logs.push(log);

    update.finalize();

    update_update(update.clone()).await?;

    Ok(update)
  }
}
