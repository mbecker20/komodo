use anyhow::{anyhow, Context};
use async_trait::async_trait;
use futures::future::join_all;
use monitor_client::{
  api::execute::*,
  entities::{
    build::Build,
    deployment::{Deployment, DeploymentImage},
    get_image_name, monitor_timestamp,
    server::ServerStatus,
    update::{Log, ResourceTarget, Update, UpdateStatus},
    Operation, PermissionLevel, Version,
  },
};
use mungos::{find::find_collect, mongodb::bson::doc};
use periphery_client::requests;
use resolver_api::Resolve;
use serror::serialize_error_pretty;

use crate::{
  auth::RequestUser,
  helpers::{make_update, resource::StateResource},
  state::State,
};

#[async_trait]
impl Resolve<Deploy, RequestUser> for State {
  async fn resolve(
    &self,
    Deploy {
      deployment_id,
      stop_signal,
      stop_time,
    }: Deploy,
    user: RequestUser,
  ) -> anyhow::Result<Update> {
    if self.action_states.deployment.busy(&deployment_id).await {
      return Err(anyhow!("deployment busy"));
    }

    let mut deployment: Deployment = self
      .get_resource_check_permissions(
        &deployment_id,
        &user,
        PermissionLevel::Execute,
      )
      .await?;

    if deployment.config.server_id.is_empty() {
      return Err(anyhow!("deployment has no server configured"));
    }

    let (server, status) = self
      .get_server_with_status(&deployment.config.server_id)
      .await?;
    if status != ServerStatus::Ok {
      return Err(anyhow!(
        "cannot send action when server is unreachable or disabled"
      ));
    }

    let periphery = self.periphery_client(&server)?;

    let inner = || async move {
      let start_ts = monitor_timestamp();

      let version = match deployment.config.image {
        DeploymentImage::Build { build_id, version } => {
          let build: Build = self.get_resource(&build_id).await?;
          let image_name = get_image_name(&build);
          let version = if version.is_none() {
            build.config.version
          } else {
            version
          };
          deployment.config.image = DeploymentImage::Image {
            image: format!("{image_name}:{}", version.to_string()),
          };
          if deployment.config.docker_account.is_empty() {
            deployment.config.docker_account =
              build.config.docker_account;
          }
          version
        }
        DeploymentImage::Image { .. } => Version::default(),
      };

      let mut update = Update {
        target: ResourceTarget::Deployment(deployment.id.clone()),
        operation: Operation::DeployContainer,
        start_ts,
        status: UpdateStatus::InProgress,
        success: true,
        operator: user.id.clone(),
        version,
        ..Default::default()
      };

      update.id = self.add_update(update.clone()).await?;

      let log = match periphery
        .request(requests::Deploy {
          deployment,
          stop_signal,
          stop_time,
        })
        .await
      {
        Ok(log) => log,
        Err(e) => Log::error("deploy container", format!("{e:#?}")),
      };

      update.logs.push(log);
      update.finalize();
      self.update_cache_for_server(&server).await;
      self.update_update(update.clone()).await?;

      Ok(update)
    };

    self
      .action_states
      .deployment
      .update_entry(deployment_id.to_string(), |entry| {
        entry.deploying = true;
      })
      .await;

    let res = inner().await;

    self
      .action_states
      .deployment
      .update_entry(deployment_id, |entry| {
        entry.deploying = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<StartContainer, RequestUser> for State {
  async fn resolve(
    &self,
    StartContainer { deployment_id }: StartContainer,
    user: RequestUser,
  ) -> anyhow::Result<Update> {
    if self.action_states.deployment.busy(&deployment_id).await {
      return Err(anyhow!("deployment busy"));
    }

    let deployment: Deployment = self
      .get_resource_check_permissions(
        &deployment_id,
        &user,
        PermissionLevel::Execute,
      )
      .await?;

    if deployment.config.server_id.is_empty() {
      return Err(anyhow!("deployment has no server configured"));
    }

    let (server, status) = self
      .get_server_with_status(&deployment.config.server_id)
      .await?;
    if status != ServerStatus::Ok {
      return Err(anyhow!(
        "cannot send action when server is unreachable or disabled"
      ));
    }

    let periphery = self.periphery_client(&server)?;

    let inner = || async move {
      let start_ts = monitor_timestamp();

      let mut update = Update {
        target: ResourceTarget::Deployment(deployment.id.clone()),
        operation: Operation::StartContainer,
        start_ts,
        status: UpdateStatus::InProgress,
        success: true,
        operator: user.id.clone(),
        ..Default::default()
      };

      update.id = self.add_update(update.clone()).await?;

      let log = match periphery
        .request(requests::StartContainer {
          name: deployment.name.clone(),
        })
        .await
      {
        Ok(log) => log,
        Err(e) => Log::error("start container", format!("{e:#?}")),
      };

      update.logs.push(log);
      update.finalize();
      self.update_cache_for_server(&server).await;
      self.update_update(update.clone()).await?;

      Ok(update)
    };

    self
      .action_states
      .deployment
      .update_entry(deployment_id.to_string(), |entry| {
        entry.starting = true;
      })
      .await;

    let res = inner().await;

    self
      .action_states
      .deployment
      .update_entry(deployment_id, |entry| {
        entry.starting = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<StopContainer, RequestUser> for State {
  async fn resolve(
    &self,
    StopContainer {
      deployment_id,
      signal,
      time,
    }: StopContainer,
    user: RequestUser,
  ) -> anyhow::Result<Update> {
    if self.action_states.deployment.busy(&deployment_id).await {
      return Err(anyhow!("deployment busy"));
    }

    let deployment: Deployment = self
      .get_resource_check_permissions(
        &deployment_id,
        &user,
        PermissionLevel::Execute,
      )
      .await?;

    if deployment.config.server_id.is_empty() {
      return Err(anyhow!("deployment has no server configured"));
    }

    let (server, status) = self
      .get_server_with_status(&deployment.config.server_id)
      .await?;
    if status != ServerStatus::Ok {
      return Err(anyhow!(
        "cannot send action when server is unreachable or disabled"
      ));
    }

    let periphery = self.periphery_client(&server)?;

    let inner = || async move {
      let start_ts = monitor_timestamp();

      let mut update = Update {
        target: ResourceTarget::Deployment(deployment.id.clone()),
        operation: Operation::StopContainer,
        start_ts,
        status: UpdateStatus::InProgress,
        success: true,
        operator: user.id.clone(),
        ..Default::default()
      };

      update.id = self.add_update(update.clone()).await?;

      let log = match periphery
        .request(requests::StopContainer {
          name: deployment.name.clone(),
          signal: signal
            .unwrap_or(deployment.config.termination_signal)
            .into(),
          time: time
            .unwrap_or(deployment.config.termination_timeout)
            .into(),
        })
        .await
      {
        Ok(log) => log,
        Err(e) => Log::error("stop container", format!("{e:#?}")),
      };

      update.logs.push(log);
      update.finalize();
      self.update_cache_for_server(&server).await;
      self.update_update(update.clone()).await?;

      Ok(update)
    };

    self
      .action_states
      .deployment
      .update_entry(deployment_id.to_string(), |entry| {
        entry.stopping = true;
      })
      .await;

    let res = inner().await;

    self
      .action_states
      .deployment
      .update_entry(deployment_id, |entry| {
        entry.stopping = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<StopAllContainers, RequestUser> for State {
  async fn resolve(
    &self,
    StopAllContainers { server_id }: StopAllContainers,
    user: RequestUser,
  ) -> anyhow::Result<Update> {
    let (server, status) =
      self.get_server_with_status(&server_id).await?;
    if status != ServerStatus::Ok {
      return Err(anyhow!(
        "cannot send action when server is unreachable or disabled"
      ));
    }

    let deployments = find_collect(
      &self.db.deployments,
      doc! {
        "config.server_id": &server_id
      },
      None,
    )
    .await
    .context("failed to find deployments on server")?;
    let inner = || async move {
      let mut update = make_update(
        ResourceTarget::Server(server.id),
        Operation::StopAllContainers,
        &user,
      );
      let futures = deployments.iter().map(|deployment| async {
        (
          self
            .resolve(
              StopContainer {
                deployment_id: deployment.id.clone(),
                signal: None,
                time: None,
              },
              user.clone(),
            )
            .await,
          deployment.name.clone(),
          deployment.id.clone(),
        )
      });
      let results = join_all(futures).await;
      let deployment_names = deployments
        .iter()
        .map(|d| format!("{} ({})", d.name, d.id))
        .collect::<Vec<_>>()
        .join("\n");
      update.push_simple_log("stopping containers", deployment_names);
      for (res, name, id) in results {
        if let Err(e) = res {
          update.push_error_log(
            "stop container failure",
            format!(
              "failed to stop container {name} ({id})\n\n{}",
              serialize_error_pretty(e)
            ),
          );
        }
      }
      update.finalize();
      self.add_update(update.clone()).await?;
      Ok(update)
    };

    self
      .action_states
      .server
      .update_entry(server_id.to_string(), |entry| {
        entry.stopping_containers = true;
      })
      .await;

    let res = inner().await;

    self
      .action_states
      .server
      .update_entry(server_id, |entry| {
        entry.stopping_containers = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<RemoveContainer, RequestUser> for State {
  async fn resolve(
    &self,
    RemoveContainer {
      deployment_id,
      signal,
      time,
    }: RemoveContainer,
    user: RequestUser,
  ) -> anyhow::Result<Update> {
    if self.action_states.deployment.busy(&deployment_id).await {
      return Err(anyhow!("deployment busy"));
    }

    let deployment: Deployment = self
      .get_resource_check_permissions(
        &deployment_id,
        &user,
        PermissionLevel::Execute,
      )
      .await?;

    if deployment.config.server_id.is_empty() {
      return Err(anyhow!("deployment has no server configured"));
    }

    let (server, status) = self
      .get_server_with_status(&deployment.config.server_id)
      .await?;
    if status != ServerStatus::Ok {
      return Err(anyhow!(
        "cannot send action when server is unreachable or disabled"
      ));
    }

    let periphery = self.periphery_client(&server)?;

    let inner = || async move {
      let start_ts = monitor_timestamp();

      let mut update = Update {
        target: ResourceTarget::Deployment(deployment.id.clone()),
        operation: Operation::RemoveContainer,
        start_ts,
        status: UpdateStatus::InProgress,
        success: true,
        operator: user.id.clone(),
        ..Default::default()
      };

      update.id = self.add_update(update.clone()).await?;

      let log = match periphery
        .request(requests::RemoveContainer {
          name: deployment.name.clone(),
          signal: signal
            .unwrap_or(deployment.config.termination_signal)
            .into(),
          time: time
            .unwrap_or(deployment.config.termination_timeout)
            .into(),
        })
        .await
      {
        Ok(log) => log,
        Err(e) => Log::error("stop container", format!("{e:#?}")),
      };

      update.logs.push(log);
      update.finalize();
      self.update_cache_for_server(&server).await;
      self.update_update(update.clone()).await?;

      Ok(update)
    };

    self
      .action_states
      .deployment
      .update_entry(deployment_id.to_string(), |entry| {
        entry.removing = true;
      })
      .await;

    let res = inner().await;

    self
      .action_states
      .deployment
      .update_entry(deployment_id, |entry| {
        entry.removing = false;
      })
      .await;

    res
  }
}
