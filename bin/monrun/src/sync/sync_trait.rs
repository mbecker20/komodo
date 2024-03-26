use std::collections::HashMap;

use async_trait::async_trait;
use monitor_client::{
  api::{
    read::ListTags,
    write::{
      self, CreateTag, UpdateDescription, UpdateTagsOnResource,
    },
  },
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
    update::ResourceTarget,
  },
};

use crate::{
  maps::{
    name_to_alerter, name_to_build, name_to_builder,
    name_to_deployment, name_to_repo, name_to_server,
  },
  monitor_client,
};

type ToUpdate<T> = Vec<(String, Resource<T>)>;
type ToCreate<T> = Vec<Resource<T>>;
type UpdatesResult<T> = (ToUpdate<T>, ToCreate<T>);

#[async_trait]
pub trait Sync {
  type PartialConfig: Clone + Send + 'static;
  type ListItemInfo: 'static;

  fn display() -> &'static str;

  fn resource_target(id: String) -> ResourceTarget;

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>;

  /// Returns created id
  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<String>;

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
    let mut tag_name_to_id = monitor_client()
      .read(ListTags::default())
      .await
      .expect("failed to ListTags mid run")
      .into_iter()
      .map(|tag| (tag.name, tag.id))
      .collect::<HashMap<_, _>>();

    for (id, resource) in to_update {
      // Update resource
      let name = resource.name.clone();
      let tags = resource.tags.clone();
      let description = resource.description.clone();
      if let Err(e) = Self::update(id.clone(), resource).await {
        warn!("failed to update {} {name} | {e:#}", Self::display());
      }
      Self::update_tags(
        id.clone(),
        &name,
        &tags,
        &mut tag_name_to_id,
      )
      .await;
      Self::update_description(id, description).await;
    }

    for resource in to_create {
      let name = resource.name.clone();
      let tags = resource.tags.clone();
      let description = resource.description.clone();
      let id = match Self::create(resource).await {
        Ok(id) => id,
        Err(e) => {
          warn!(
            "failed to create {} {name} | {e:#}",
            Self::display(),
          );
          continue;
        }
      };
      Self::update_tags(
        id.clone(),
        &name,
        &tags,
        &mut tag_name_to_id,
      )
      .await;
      Self::update_description(id, description).await;
    }
  }

  async fn update_tags(
    resource_id: String,
    resource_name: &str,
    tags: &[String],
    tag_name_to_id: &mut HashMap<String, String>,
  ) {
    // make sure all tags are created
    for tag_name in tags {
      if !tag_name_to_id.contains_key(tag_name) {
        let tag_id = monitor_client()
          .write(CreateTag {
            name: tag_name.to_string(),
          })
          .await
          .expect("failed to CreateTag mid run")
          .id;
        tag_name_to_id.insert(tag_name.to_string(), tag_id);
      }
    }

    // get Vec<tag_id>
    let tags = tags
      .iter()
      .map(|tag_name| {
        tag_name_to_id
          .get(tag_name)
          .expect("somehow didn't find tag at this point")
          .to_string()
      })
      .collect();

    // Update tags
    if let Err(e) = monitor_client()
      .write(UpdateTagsOnResource {
        target: Self::resource_target(resource_id),
        tags,
      })
      .await
    {
      warn!(
        "failed to update tags on {} {resource_name} | {e:#}",
        Self::display(),
      );
    }
  }

  async fn update_description(id: String, description: String) {
    if let Err(e) = monitor_client()
      .write(UpdateDescription {
        target: Self::resource_target(id.clone()),
        description,
      })
      .await
    {
      warn!("failed to update resource {id} description | {e:#}");
    }
  }
}

#[async_trait]
impl Sync for Server {
  type ListItemInfo = ServerListItemInfo;
  type PartialConfig = PartialServerConfig;

  fn display() -> &'static str {
    "server"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Server(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_server()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(write::CreateServer {
        name: resource.name,
        config: resource.config,
      })
      .await
      .map(|res| res.id)
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

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Deployment(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_deployment()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(write::CreateDeployment {
        name: resource.name,
        config: resource.config,
      })
      .await
      .map(|res| res.id)
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

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Build(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_build()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(write::CreateBuild {
        name: resource.name,
        config: resource.config,
      })
      .await
      .map(|res| res.id)
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

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Builder(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_builder()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(write::CreateBuilder {
        name: resource.name,
        config: resource.config,
      })
      .await
      .map(|res| res.id)
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

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Alerter(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_alerter()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(write::CreateAlerter {
        name: resource.name,
        config: resource.config,
      })
      .await
      .map(|res| res.id)
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

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Repo(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_repo()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(write::CreateRepo {
        name: resource.name,
        config: resource.config,
      })
      .await
      .map(|res| res.id)
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
