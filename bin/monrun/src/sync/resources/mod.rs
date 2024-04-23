use std::collections::HashMap;

use monitor_client::{
  api::write::{UpdateDescription, UpdateTagsOnResource},
  entities::{
    resource::{Resource, ResourceListItem},
    update::ResourceTarget,
  },
};

use crate::monitor_client;

pub mod alerter;
pub mod build;
pub mod builder;
pub mod deployment;
pub mod procedure;
pub mod repo;
pub mod server;

type ToUpdate<T> = Vec<(String, Resource<T>)>;
type ToCreate<T> = Vec<Resource<T>>;
type UpdatesResult<T> = (ToCreate<T>, ToUpdate<T>);

pub trait ResourceSync {
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
  ) -> UpdatesResult<Self::PartialConfig> {
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
        "\n{} TO CREATE: {}",
        Self::display(),
        to_create
          .iter()
          .map(|item| item.name.as_str())
          .collect::<Vec<_>>()
          .join(", ")
      );
    }

    if !to_update.is_empty() {
      println!(
        "\n{} TO UPDATE: {}",
        Self::display(),
        to_update
          .iter()
          .map(|(_, item)| item.name.as_str())
          .collect::<Vec<_>>()
          .join(", ")
      );
    }

    (to_create, to_update)
  }

  async fn run_updates(
    to_create: ToCreate<Self::PartialConfig>,
    to_update: ToUpdate<Self::PartialConfig>,
  ) {
    let log_after = !to_update.is_empty() || !to_create.is_empty();

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
      Self::update_tags(id.clone(), &name, tags).await;
      Self::update_description(id, description).await;
      info!("{} {name} created", Self::display());
    }

    for (id, resource) in to_update {
      // Update resource
      let name = resource.name.clone();
      let tags = resource.tags.clone();
      let description = resource.description.clone();
      if let Err(e) = Self::update(id.clone(), resource).await {
        warn!("failed to update {} {name} | {e:#}", Self::display());
      }
      Self::update_tags(id.clone(), &name, tags).await;
      Self::update_description(id, description).await;
      info!("{} {name} updated", Self::display());
    }

    if log_after {
      info!(
        "============ {}s synced ✅ ============",
        Self::display()
      );
    }
  }

  async fn update_tags(
    resource_id: String,
    resource_name: &str,
    tags: Vec<String>,
  ) {
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
