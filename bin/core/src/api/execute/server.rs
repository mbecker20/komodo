use anyhow::{anyhow, Context};
use async_trait::async_trait;
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
    add_update, make_update, periphery_client,
    resource::StateResource, update_update,
  },
  state::{action_states, State},
};

#[async_trait]
impl Resolve<PruneDockerContainers, User> for State {
  async fn resolve(
    &self,
    PruneDockerContainers { server }: PruneDockerContainers,
    user: User,
  ) -> anyhow::Result<Update> {
    let server = Server::get_resource_check_permissions(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    if action_states().server.busy(&server.id).await {
      return Err(anyhow!("server busy"));
    }

    let periphery = periphery_client(&server)?;

    let inner = || async {
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
    };

    action_states()
      .server
      .update_entry(server.id.to_string(), |entry| {
        entry.pruning_containers = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .server
      .update_entry(server.id, |entry| {
        entry.pruning_containers = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<PruneDockerNetworks, User> for State {
  async fn resolve(
    &self,
    PruneDockerNetworks { server }: PruneDockerNetworks,
    user: User,
  ) -> anyhow::Result<Update> {
    let server = Server::get_resource_check_permissions(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    if action_states().server.busy(&server.id).await {
      return Err(anyhow!("server busy"));
    }

    let periphery = periphery_client(&server)?;

    let inner = || async {
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
    };

    action_states()
      .server
      .update_entry(server.id.clone(), |entry| {
        entry.pruning_networks = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .server
      .update_entry(server.id, |entry| {
        entry.pruning_networks = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<PruneDockerImages, User> for State {
  async fn resolve(
    &self,
    PruneDockerImages { server }: PruneDockerImages,
    user: User,
  ) -> anyhow::Result<Update> {
    let server = Server::get_resource_check_permissions(
      &server,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    if action_states().server.busy(&server.id).await {
      return Err(anyhow!("server busy"));
    }

    let periphery = periphery_client(&server)?;

    let inner = || async {
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
    };

    action_states()
      .server
      .update_entry(&server.id, |entry| {
        entry.pruning_images = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .server
      .update_entry(server.id, |entry| {
        entry.pruning_images = false;
      })
      .await;

    res
  }
}
