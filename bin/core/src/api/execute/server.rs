use anyhow::Context;
use monitor_client::{
  api::execute::*,
  entities::{
    monitor_timestamp,
    permission::PermissionLevel,
    server::Server,
    update::{Log, Update, UpdateStatus},
    user::User,
    Operation,
  },
};
use periphery_client::api;
use resolver_api::Resolve;
use serror::serialize_error_pretty;

use crate::{
  helpers::{
    periphery_client,
    update::{add_update, make_update, update_update},
  },
  resource,
  state::{action_states, State},
};

impl Resolve<PruneDockerContainers, User> for State {
  #[instrument(name = "PruneDockerContainers", skip(self, user))]
  async fn resolve(
    &self,
    PruneDockerContainers { server }: PruneDockerContainers,
    user: User,
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

    let mut update =
      make_update(&server, Operation::PruneContainersServer, &user);
    update.in_progress();
    update.id = add_update(update.clone()).await?;

    let log = match periphery
      .request(api::container::PruneContainers {})
      .await
      .context(format!(
        "failed to prune containers on server {}",
        server.name
      )) {
      Ok(log) => log,
      Err(e) => {
        Log::error("prune containers", serialize_error_pretty(&e))
      }
    };

    update.success = log.success;
    update.status = UpdateStatus::Complete;
    update.end_ts = Some(monitor_timestamp());
    update.logs.push(log);

    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<PruneDockerNetworks, User> for State {
  #[instrument(name = "PruneDockerNetworks", skip(self, user))]
  async fn resolve(
    &self,
    PruneDockerNetworks { server }: PruneDockerNetworks,
    user: User,
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

    let mut update =
      make_update(&server, Operation::PruneNetworksServer, &user);
    update.in_progress();
    update.id = add_update(update.clone()).await?;

    let log = match periphery
      .request(api::network::PruneNetworks {})
      .await
      .context(format!(
        "failed to prune networks on server {}",
        server.name
      )) {
      Ok(log) => log,
      Err(e) => {
        Log::error("prune networks", serialize_error_pretty(&e))
      }
    };

    update.success = log.success;
    update.status = UpdateStatus::Complete;
    update.end_ts = Some(monitor_timestamp());
    update.logs.push(log);

    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<PruneDockerImages, User> for State {
  #[instrument(name = "PruneDockerImages", skip(self, user))]
  async fn resolve(
    &self,
    PruneDockerImages { server }: PruneDockerImages,
    user: User,
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

    let mut update =
      make_update(&server, Operation::PruneImagesServer, &user);
    update.in_progress();
    update.id = add_update(update.clone()).await?;

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
