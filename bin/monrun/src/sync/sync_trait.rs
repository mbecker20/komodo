use std::collections::HashMap;

use async_trait::async_trait;
use monitor_client::{
  api::write,
  entities::{
    alerter::{Alerter, AlerterListItemInfo, PartialAlerterConfig},
    build::{Build, BuildListItemInfo, PartialBuildConfig},
    builder::{Builder, BuilderListItemInfo, PartialBuilderConfig},
    deployment::{
      Deployment, DeploymentListItemInfo, PartialDeploymentConfig,
    },
    repo::{PartialRepoConfig, Repo, RepoInfo},
    resource::{Resource, ResourceListItem},
    server::{PartialServerConfig, Server, ServerListItemInfo},
  },
};

use crate::{
  maps::{
    name_to_alerter, name_to_build, name_to_builder,
    name_to_deployment, name_to_repo, name_to_server,
  },
  monitor_client,
};

#[async_trait]
impl Sync for Server {
  type ListItemInfo = ServerListItemInfo;
  type PartialConfig = PartialServerConfig;

  fn display() -> &'static str {
    "server"
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_server()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::CreateServer {
        name: resource.name,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn update(
    id: String,
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::UpdateServer {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}

#[async_trait]
impl Sync for Deployment {
  type PartialConfig = PartialDeploymentConfig;
  type ListItemInfo = DeploymentListItemInfo;

  fn display() -> &'static str {
    "deployment"
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_deployment()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::CreateDeployment {
        name: resource.name,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn update(
    id: String,
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::UpdateDeployment {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}

#[async_trait]
impl Sync for Build {
  type PartialConfig = PartialBuildConfig;
  type ListItemInfo = BuildListItemInfo;

  fn display() -> &'static str {
    "build"
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_build()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::CreateBuild {
        name: resource.name,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn update(
    id: String,
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::UpdateBuild {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}

#[async_trait]
impl Sync for Builder {
  type PartialConfig = PartialBuilderConfig;
  type ListItemInfo = BuilderListItemInfo;

  fn display() -> &'static str {
    "builder"
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_builder()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::CreateBuilder {
        name: resource.name,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn update(
    id: String,
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::UpdateBuilder {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}

#[async_trait]
impl Sync for Alerter {
  type PartialConfig = PartialAlerterConfig;
  type ListItemInfo = AlerterListItemInfo;

  fn display() -> &'static str {
    "alerter"
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_alerter()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::CreateAlerter {
        name: resource.name,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn update(
    id: String,
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::UpdateAlerter {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}

#[async_trait]
impl Sync for Repo {
  type PartialConfig = PartialRepoConfig;
  type ListItemInfo = RepoInfo;

  fn display() -> &'static str {
    "repo"
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_repo()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::CreateRepo {
        name: resource.name,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn update(
    id: String,
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::UpdateRepo {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}

type ToUpdate<T> = Vec<(String, Resource<T>)>;
type ToCreate<T> = Vec<Resource<T>>;
type UpdatesResult<T> = (ToUpdate<T>, ToCreate<T>);

#[async_trait]
pub trait Sync {
  type PartialConfig: Send + 'static;
  type ListItemInfo: 'static;

  fn display() -> &'static str;

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>;

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()>;

  async fn update(
    id: String,
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()>;

  fn get_updates(
    resources: Vec<Resource<Self::PartialConfig>>,
  ) -> anyhow::Result<UpdatesResult<Self::PartialConfig>> {
    let map = Self::name_to_resource();

    // (name, partial config)
    let mut to_update =
      Vec::<(String, Resource<Self::PartialConfig>)>::new();
    let mut to_create = Vec::<Resource<Self::PartialConfig>>::new();

    for resource in resources {
      match map.get(&resource.name).map(|s| s.id.clone()) {
        Some(id) => {
          to_update.push((id, resource));
        }
        None => {
          to_create.push(resource);
        }
      }
    }

    if !to_create.is_empty() {
      println!(
        "\nTO CREATE: {}",
        to_create
          .iter()
          .map(|item| item.name.as_str())
          .collect::<Vec<_>>()
          .join(", ")
      );
    }

    if !to_update.is_empty() {
      println!(
        "\nTO UPDATE: {}",
        to_update
          .iter()
          .map(|(_, item)| item.name.as_str())
          .collect::<Vec<_>>()
          .join(", ")
      );
    }

    Ok((to_update, to_create))
  }

  async fn run_updates(
    to_update: ToUpdate<Self::PartialConfig>,
    to_create: ToCreate<Self::PartialConfig>,
  ) {
    for (id, resource) in to_update {
      let name = resource.name.clone();
      if let Err(e) = Self::update(id, resource).await {
        warn!("failed to update {} {name} | {e:#}", Self::display(),)
      }
    }

    for resource in to_create {
      let name = resource.name.clone();
      if let Err(e) = Self::create(resource).await {
        warn!("failed to create {} {name} | {e:#}", Self::display(),)
      }
    }
  }
}
