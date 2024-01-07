use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::execute::*,
  entities::{
    monitor_timestamp,
    server::Server,
    update::{Log, ResourceTarget, Update, UpdateStatus},
    Operation, PermissionLevel,
  },
};
use periphery_client::requests;
use resolver_api::Resolve;

use crate::{
  auth::RequestUser, helpers::resource::StateResource, state::State,
};

#[async_trait]
impl Resolve<PruneDockerContainers, RequestUser> for State {
  async fn resolve(
    &self,
    PruneDockerContainers { server_id }: PruneDockerContainers,
    user: RequestUser,
  ) -> anyhow::Result<Update> {
    if self.action_states.server.busy(&server_id).await {
      return Err(anyhow!("server busy"));
    }

    let server: Server = self
      .get_resource_check_permissions(
        &server_id,
        &user,
        PermissionLevel::Execute,
      )
      .await?;

    let periphery = self.periphery_client(&server)?;

    let inner = || async {
      let start_ts = monitor_timestamp();
      let mut update = Update {
        target: ResourceTarget::Server(server_id),
        operation: Operation::PruneContainersServer,
        start_ts,
        status: UpdateStatus::InProgress,
        success: true,
        operator: user.id.clone(),
        ..Default::default()
      };
      update.id = self.add_update(update.clone()).await?;

      let log = match periphery
        .request(requests::PruneNetworks {})
        .await
        .context(format!(
          "failed to prune containers on server {}",
          server.name
        )) {
        Ok(log) => log,
        Err(e) => Log::error("prune containers", format!("{e:#?}")),
      };

      update.success = log.success;
      update.status = UpdateStatus::Complete;
      update.end_ts = Some(monitor_timestamp());
      update.logs.push(log);

      self.update_update(update.clone()).await?;

      Ok(update)
    };

    self
      .action_states
      .server
      .update_entry(server.id.to_string(), |entry| {
        entry.pruning_containers = true;
      })
      .await;

    let res = inner().await;

    self
      .action_states
      .server
      .update_entry(server.id, |entry| {
        entry.pruning_containers = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<PruneDockerNetworks, RequestUser> for State {
  async fn resolve(
    &self,
    PruneDockerNetworks { server_id }: PruneDockerNetworks,
    user: RequestUser,
  ) -> anyhow::Result<Update> {
    if self.action_states.server.busy(&server_id).await {
      return Err(anyhow!("server busy"));
    }

    let server: Server = self
      .get_resource_check_permissions(
        &server_id,
        &user,
        PermissionLevel::Execute,
      )
      .await?;

    let periphery = self.periphery_client(&server)?;

    let inner = || async {
      let start_ts = monitor_timestamp();
      let mut update = Update {
        target: ResourceTarget::Server(server_id.to_owned()),
        operation: Operation::PruneNetworksServer,
        start_ts,
        status: UpdateStatus::InProgress,
        success: true,
        operator: user.id.clone(),
        ..Default::default()
      };
      update.id = self.add_update(update.clone()).await?;

      let log = match periphery
        .request(requests::PruneNetworks {})
        .await
        .context(format!(
          "failed to prune networks on server {}",
          server.name
        )) {
        Ok(log) => log,
        Err(e) => Log::error("prune networks", format!("{e:#?}")),
      };

      update.success = log.success;
      update.status = UpdateStatus::Complete;
      update.end_ts = Some(monitor_timestamp());
      update.logs.push(log);

      self.update_update(update.clone()).await?;

      Ok(update)
    };

    self
      .action_states
      .server
      .update_entry(server_id.to_string(), |entry| {
        entry.pruning_networks = true;
      })
      .await;

    let res = inner().await;

    self
      .action_states
      .server
      .update_entry(server_id.to_string(), |entry| {
        entry.pruning_networks = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<PruneDockerImages, RequestUser> for State {
  async fn resolve(
    &self,
    PruneDockerImages { server_id }: PruneDockerImages,
    user: RequestUser,
  ) -> anyhow::Result<Update> {
    if self.action_states.server.busy(&server_id).await {
      return Err(anyhow!("server busy"));
    }

    let server: Server = self
      .get_resource_check_permissions(
        &server_id,
        &user,
        PermissionLevel::Execute,
      )
      .await?;

    let periphery = self.periphery_client(&server)?;

    let inner = || async {
      let start_ts = monitor_timestamp();
      let mut update = Update {
        target: ResourceTarget::Server(server_id.to_owned()),
        operation: Operation::PruneImagesServer,
        start_ts,
        status: UpdateStatus::InProgress,
        success: true,
        operator: user.id.clone(),
        ..Default::default()
      };
      update.id = self.add_update(update.clone()).await?;

      let log = match periphery
        .request(requests::PruneImages {})
        .await
        .context(format!(
          "failed to prune images on server {}",
          server.name
        )) {
        Ok(log) => log,
        Err(e) => Log::error("prune images", format!("{e:#?}")),
      };

      update.success = log.success;
      update.status = UpdateStatus::Complete;
      update.end_ts = Some(monitor_timestamp());
      update.logs.push(log);

      self.update_update(update.clone()).await?;

      Ok(update)
    };

    self
      .action_states
      .server
      .update_entry(server_id.to_string(), |entry| {
        entry.pruning_images = true;
      })
      .await;

    let res = inner().await;

    self
      .action_states
      .server
      .update_entry(server_id.to_string(), |entry| {
        entry.pruning_images = false;
      })
      .await;

    res
  }
}
