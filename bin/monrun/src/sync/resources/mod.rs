use std::collections::HashMap;

use monitor_client::{
  api::write::{UpdateDescription, UpdateTagsOnResource},
  entities::{
    resource::{Resource, ResourceListItem},
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::MaybeNone;

use crate::{cli_args, monitor_client};

pub mod alerter;
pub mod build;
pub mod builder;
pub mod deployment;
pub mod procedure;
pub mod repo;
pub mod server;
pub mod server_template;

type ToUpdate<T> = Vec<(String, ResourceToml<T>)>;
type ToCreate<T> = Vec<ResourceToml<T>>;
type UpdatesResult<T> = (ToCreate<T>, ToUpdate<T>);

pub trait ResourceSync {
  type PartialConfig: std::fmt::Debug
    + Clone
    + Send
    + MaybeNone
    + 'static;
  type FullConfig: Clone + Send + 'static;
  type FullInfo: Default;
  type ListItemInfo: 'static;

  fn display() -> &'static str;

  fn resource_target(id: String) -> ResourceTarget;

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>;

  /// Creates the resource and returns created id.
  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String>;

  /// Updates the resource at id with the partial config.
  async fn update(
    id: String,
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<()>;

  async fn get(
    id: String,
  ) -> anyhow::Result<Resource<Self::FullConfig, Self::FullInfo>>;

  /// Diffs the declared toml (partial) against the full existing config.
  /// Removes all fields from toml (partial) that haven't changed.
  async fn minimize_update(
    original: Self::FullConfig,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::PartialConfig>;

  async fn get_updates(
    resources: Vec<ResourceToml<Self::PartialConfig>>,
  ) -> anyhow::Result<UpdatesResult<Self::PartialConfig>> {
    let map = Self::name_to_resource();

    let mut to_create = ToCreate::<Self::PartialConfig>::new();
    let mut to_update = ToUpdate::<Self::PartialConfig>::new();

    for mut resource in resources {
      match map.get(&resource.name).map(|s| s.id.clone()) {
        Some(id) => {
          // Get the full original config for the resource.
          let original = Self::get(id.clone()).await?.config;

          // Minimizes updates through diffing.
          resource.config =
            Self::minimize_update(original, resource.config).await?;

          // Only try to update if there are any fields to update.
          if !resource.config.is_none() {
            to_update.push((id, resource));
          }
        }
        None => {
          to_create.push(resource);
        }
      }
    }

    let verbose = cli_args().verbose;

    if !to_create.is_empty() {
      if verbose {
        println!("\n{} TO CREATE:\n{to_create:#?}", Self::display(),);
      } else {
        println!(
          "\n{} TO CREATE: {:#?}",
          Self::display(),
          to_create
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>()
        );
      }
    }

    if !to_update.is_empty() {
      if verbose {
        println!(
          "\n{} TO UPDATE:\n{:#?}",
          Self::display(),
          to_update.iter().map(|(_, r)| r).collect::<Vec<_>>()
        );
      } else {
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
    }

    Ok((to_create, to_update))
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
