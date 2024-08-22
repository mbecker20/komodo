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
    periphery_client, query::get_server_with_state,
    update::update_update,
  },
  monitor::update_cache_for_server,
  resource,
  state::{action_states, State},
};

impl Resolve<StartContainer, (User, Update)> for State {
  #[instrument(name = "StartContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StartContainer { server, container }: StartContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // // get the action state for the server (or insert default).
    // let action_state = action_states()
    //   .server
    //   .get_or_insert_default(&server.id)
    //   .await;

    // // Will check to ensure deployment not already busy before updating, and return Err if so.
    // // The returned guard will set the action state back to default when dropped.
    // let _action_guard =
    //   action_state.update(|state| state.starting = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::StartContainer { name: container })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "start container",
        format_serror(&e.context("failed to start container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<RestartContainer, (User, Update)> for State {
  #[instrument(name = "RestartContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RestartContainer { server, container }: RestartContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // // get the action state for the deployment (or insert default).
    // let action_state = action_states()
    //   .deployment
    //   .get_or_insert_default(&deployment.id)
    //   .await;

    // // Will check to ensure deployment not already busy before updating, and return Err if so.
    // // The returned guard will set the action state back to default when dropped.
    // let _action_guard =
    //   action_state.update(|state| state.restarting = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::RestartContainer { name: container })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "restart container",
        format_serror(
          &e.context("failed to restart container").into(),
        ),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<PauseContainer, (User, Update)> for State {
  #[instrument(name = "PauseContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PauseContainer { server, container }: PauseContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // // get the action state for the deployment (or insert default).
    // let action_state = action_states()
    //   .deployment
    //   .get_or_insert_default(&deployment.id)
    //   .await;

    // // Will check to ensure deployment not already busy before updating, and return Err if so.
    // // The returned guard will set the action state back to default when dropped.
    // let _action_guard =
    //   action_state.update(|state| state.pausing = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::PauseContainer { name: container })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "pause container",
        format_serror(&e.context("failed to pause container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<UnpauseContainer, (User, Update)> for State {
  #[instrument(name = "UnpauseContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    UnpauseContainer { server, container }: UnpauseContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // // get the action state for the deployment (or insert default).
    // let action_state = action_states()
    //   .deployment
    //   .get_or_insert_default(&deployment.id)
    //   .await;

    // // Will check to ensure deployment not already busy before updating, and return Err if so.
    // // The returned guard will set the action state back to default when dropped.
    // let _action_guard =
    //   action_state.update(|state| state.unpausing = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::UnpauseContainer { name: container })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "unpause container",
        format_serror(
          &e.context("failed to unpause container").into(),
        ),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<StopContainer, (User, Update)> for State {
  #[instrument(name = "StopContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StopContainer {
      server,
      container,
      signal,
      time,
    }: StopContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // // get the action state for the deployment (or insert default).
    // let action_state = action_states()
    //   .deployment
    //   .get_or_insert_default(&deployment.id)
    //   .await;

    // // Will check to ensure deployment not already busy before updating, and return Err if so.
    // // The returned guard will set the action state back to default when dropped.
    // let _action_guard =
    //   action_state.update(|state| state.stopping = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::StopContainer {
        name: container,
        signal,
        time,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "stop container",
        format_serror(&e.context("failed to stop container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<DestroyContainer, (User, Update)> for State {
  #[instrument(name = "DestroyContainer", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DestroyContainer {
      server,
      container,
      signal,
      time,
    }: DestroyContainer,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let server = resource::get_check_permissions::<Server>(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // // get the action state for the deployment (or insert default).
    // let action_state = action_states()
    //   .deployment
    //   .get_or_insert_default(&deployment.id)
    //   .await;

    // // Will check to ensure deployment not already busy before updating, and return Err if so.
    // // The returned guard will set the action state back to default when dropped.
    // let _action_guard =
    //   action_state.update(|state| state.destroying = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::RemoveContainer {
        name: container,
        signal,
        time,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "stop container",
        format_serror(&e.context("failed to stop container").into()),
      ),
    };

    update.logs.push(log);
    update.finalize();
    update_cache_for_server(&server).await;
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<StopAllContainers, (User, Update)> for State {
  #[instrument(name = "StopAllContainers", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StopAllContainers { server }: StopAllContainers,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, status) = get_server_with_state(&server).await?;
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

    update_update(update.clone()).await?;

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

    update_update(update.clone()).await?;

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

    update_update(update.clone()).await?;

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

    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log =
      match periphery.request(api::image::PruneImages {}).await {
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

impl Resolve<PruneVolumes, (User, Update)> for State {
  #[instrument(name = "PruneVolumes", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PruneVolumes { server }: PruneVolumes,
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
      action_state.update(|state| state.pruning_volumes = true)?;

    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log =
      match periphery.request(api::volume::PruneVolumes {}).await {
        Ok(log) => log,
        Err(e) => Log::error(
          "prune volumes",
          format!(
            "failed to prune volumes on server {} | {e:#?}",
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

impl Resolve<PruneSystem, (User, Update)> for State {
  #[instrument(name = "PruneSystem", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PruneSystem { server }: PruneSystem,
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
      action_state.update(|state| state.pruning_system = true)?;

    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery.request(api::PruneSystem {}).await {
      Ok(log) => log,
      Err(e) => Log::error(
        "prune system",
        format!(
          "failed to docket system prune on server {} | {e:#?}",
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
