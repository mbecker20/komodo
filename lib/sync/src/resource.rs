use std::collections::HashMap;

use monitor_client::entities::{
  resource::Resource, tag::Tag, toml::ResourceToml,
  update::ResourceTarget,
};
use partial_derive2::{Diff, MaybeNone, PartialDiff};
use serde::Serialize;

pub type ToUpdate<T> = Vec<ToUpdateItem<T>>;
pub type ToCreate<T> = Vec<ResourceToml<T>>;
/// Vec of resource names
pub type ToDelete = Vec<String>;

type UpdatesResult<T> = (ToCreate<T>, ToUpdate<T>, ToDelete);

pub struct ToUpdateItem<T> {
  pub id: String,
  pub resource: ResourceToml<T>,
  pub update_description: bool,
  pub update_tags: bool,
}

/// Implement this one depending on environment
pub trait ResourceSync: Sized {
  type Config: Clone
    + Default
    + Send
    + From<Self::PartialConfig>
    + PartialDiff<Self::PartialConfig, Self::ConfigDiff>
    + 'static;
  type Info: Default + 'static;
  type PartialConfig: std::fmt::Debug
    + Clone
    + Send
    + From<Self::Config>
    + Serialize
    + MaybeNone
    + From<Self::ConfigDiff>
    + 'static;
  type ConfigDiff: Diff + MaybeNone;

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>;

  /// Creates the resource and returns created id.
  #[allow(async_fn_in_trait)]
  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String>;

  /// Updates the resource at id with the partial config.
  #[allow(async_fn_in_trait)]
  async fn update(
    id: String,
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<()>;

  /// Deletes the target resource
  #[allow(async_fn_in_trait)]
  async fn delete(id_or_name: String) -> anyhow::Result<()>;

  #[allow(async_fn_in_trait)]
  async fn update_tags(
    id: String,
    tags: Vec<String>,
  ) -> anyhow::Result<()>;

  #[allow(async_fn_in_trait)]
  async fn update_description(
    id: String,
    description: String,
  ) -> anyhow::Result<()>;

  /// Diffs the declared toml (partial) against the full existing config.
  /// Removes all fields from toml (partial) that haven't changed.
  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff>;
}

pub trait ResourceSyncOuter<
  Implementer: ResourceSync,
  Logger: SyncLogger<Implementer>,
> where
  Self: Sized,
{
  fn display() -> &'static str;

  fn resource_target(id: String) -> ResourceTarget;

  #[allow(async_fn_in_trait)]
  async fn run_updates(
    to_create: ToCreate<Implementer::PartialConfig>,
    to_update: ToUpdate<Implementer::PartialConfig>,
    to_delete: ToDelete,
  ) {
    for resource in to_create {
      let name = resource.name.clone();
      let tags = resource.tags.clone();
      let description = resource.description.clone();
      let id = match Implementer::create(resource).await {
        Ok(id) => id,
        Err(e) => {
          Logger::log_failed_create(&name, e);
          // warn!(
          //   "failed to create {} {name} | {e:#}",
          //   Self::display(),
          // );
          continue;
        }
      };
      run_update_tags::<Implementer, Self, Logger>(
        id.clone(),
        &name,
        tags,
      )
      .await;
      run_update_description::<Implementer, Self, Logger>(
        id,
        &name,
        description,
      )
      .await;
      Logger::log_created(&name);
      // info!(
      //   "{} {} '{}'",
      //   "created".green().bold(),
      //   Self::display(),
      //   name.bold(),
      // );
    }

    for ToUpdateItem {
      id,
      resource,
      update_description,
      update_tags,
    } in to_update
    {
      // Update resource
      let name = resource.name.clone();
      let tags = resource.tags.clone();
      let description = resource.description.clone();

      if update_description {
        run_update_description::<Implementer, Self, Logger>(
          id.clone(),
          &name,
          description,
        )
        .await;
      }

      if update_tags {
        run_update_tags::<Implementer, Self, Logger>(
          id.clone(),
          &name,
          tags,
        )
        .await;
      }

      if !resource.config.is_none() {
        if let Err(e) = Implementer::update(id, resource).await {
          Logger::log_failed_update(&name, e);
          // warn!(
          //   "failed to update config on {} {name} | {e:#}",
          //   Self::display()
          // );
        } else {
          Logger::log_updated(&name);
          // info!(
          //   "{} {} '{}' configuration",
          //   "updated".blue().bold(),
          //   Self::display(),
          //   name.bold(),
          // );
        }
      }
    }

    for resource in to_delete {
      if let Err(e) = Implementer::delete(resource.clone()).await {
        Logger::log_failed_delete(&resource, e);
        // warn!(
        //   "failed to delete {} {resource} | {e:#}",
        //   Self::display()
        // );
      } else {
        Logger::log_deleted(&resource);
        // info!(
        //   "{} {} '{}'",
        //   "deleted".red().bold(),
        //   Self::display(),
        //   resource.bold(),
        // );
      }
    }
  }
}

pub trait SyncLogger<Implementer: ResourceSync>
where
  Self: Sized,
{
  fn log_to_create(
    resource: &ResourceToml<Implementer::PartialConfig>,
  );
  fn log_to_update(
    name: &str,
    description: &str,
    tags: &[String],
    diff: &Implementer::ConfigDiff,
  );
  fn log_to_delete(name: &str);

  fn log_created(name: &str);
  fn log_failed_create(name: &str, e: anyhow::Error);

  fn log_updated(name: &str);
  fn log_failed_update(name: &str, e: anyhow::Error);

  fn log_deleted(name: &str);
  fn log_failed_delete(name: &str, e: anyhow::Error);

  fn log_tags_updated(name: &str);
  fn log_failed_tag_update(name: &str, e: anyhow::Error);

  fn log_description_updated(name: &str);
  fn log_failed_description_update(name: &str, e: anyhow::Error);

  fn log_procedure_sync_failed_max_iter();
}

pub trait IdToTag {
  fn id_to_tag() -> HashMap<String, Tag>;
}

/// Gets all the resources to update, logging along the way.
pub fn get_updates<Implementer, Resource, Tags, Logger>(
  resources: Vec<ResourceToml<Implementer::PartialConfig>>,
  delete: bool,
) -> anyhow::Result<UpdatesResult<Implementer::PartialConfig>>
where
  Implementer: ResourceSync,
  Resource: ResourceSyncOuter<Implementer, Logger>,
  Tags: IdToTag,
  Logger: SyncLogger<Implementer>,
{
  let map = Implementer::name_to_resource();

  let mut to_create = ToCreate::<Implementer::PartialConfig>::new();
  let mut to_update = ToUpdate::<Implementer::PartialConfig>::new();
  let mut to_delete = ToDelete::new();

  if delete {
    for resource in map.values() {
      if !resources.iter().any(|r| r.name == resource.name) {
        to_delete.push(resource.name.clone());
      }
    }
  }

  for mut resource in resources {
    match map.get(&resource.name) {
      Some(original) => {
        // First merge toml resource config (partial) onto default resource config.
        // Makes sure things that aren't defined in toml (come through as None) actually get removed.
        let config: Implementer::Config = resource.config.into();
        resource.config = config.into();

        let diff = Implementer::get_diff(
          original.config.clone(),
          resource.config,
        )?;

        let tags = Tags::id_to_tag();
        let original_tags = original
          .tags
          .iter()
          .filter_map(|id| tags.get(id).map(|t| t.name.clone()))
          .collect::<Vec<_>>();

        // Only proceed if there are any fields to update,
        // or a change to tags / description
        if diff.is_none()
          && resource.description == original.description
          && resource.tags == original_tags
        {
          continue;
        }

        Logger::log_to_update(
          &resource.name,
          &resource.description,
          &resource.tags,
          &diff,
        );

        // println!(
        //   "\n{}: {}: '{}'\n-------------------",
        //   "UPDATE".blue(),
        //   Resource::display(),
        //   resource.name.bold(),
        // );
        // let mut lines = Vec::<String>::new();
        // if resource.description != original.description {
        //   lines.push(format!(
        //     "{}: 'description'\n{}:  {}\n{}:    {}",
        //     "field".dimmed(),
        //     "from".dimmed(),
        //     original.description.red(),
        //     "to".dimmed(),
        //     resource.description.green()
        //   ))
        // }
        // if resource.tags != original_tags {
        //   let from = format!("{:?}", original_tags).red();
        //   let to = format!("{:?}", resource.tags).green();
        //   lines.push(format!(
        //     "{}: 'tags'\n{}:  {from}\n{}:    {to}",
        //     "field".dimmed(),
        //     "from".dimmed(),
        //     "to".dimmed(),
        //   ));
        // }
        // lines.extend(diff.iter_field_diffs().map(
        //   |FieldDiff { field, from, to }| {
        //     format!(
        //       "{}: '{field}'\n{}:  {}\n{}:    {}",
        //       "field".dimmed(),
        //       "from".dimmed(),
        //       from.red(),
        //       "to".dimmed(),
        //       to.green()
        //     )
        //   },
        // ));
        // println!("{}", lines.join("\n-------------------\n"));

        // Minimizes updates through diffing.
        resource.config = diff.into();

        let update = ToUpdateItem {
          id: original.id.clone(),
          update_description: resource.description
            != original.description,
          update_tags: resource.tags != original_tags,
          resource,
        };

        to_update.push(update);
      }
      None => {
        Logger::log_to_create(&resource);
        // println!(
        //   "\n{}: {}: {}\n{}: {}\n{}: {:?}\n{}: {}",
        //   "CREATE".green(),
        //   Resource::display(),
        //   resource.name.bold().green(),
        //   "description".dimmed(),
        //   resource.description,
        //   "tags".dimmed(),
        //   resource.tags,
        //   "config".dimmed(),
        //   serde_json::to_string_pretty(&resource.config)?
        // );
        to_create.push(resource);
      }
    }
  }

  for name in &to_delete {
    Logger::log_to_delete(name);
    // println!(
    //   "\n{}: {}: '{}'\n-------------------",
    //   "DELETE".red(),
    //   Resource::display(),
    //   name.bold(),
    // );
  }

  Ok((to_create, to_update, to_delete))
}

pub async fn run_update_tags<Implementer, Resource, Logger>(
  id: String,
  name: &str,
  tags: Vec<String>,
) where
  Implementer: ResourceSync,
  Resource: ResourceSyncOuter<Implementer, Logger>,
  Logger: SyncLogger<Implementer>,
{
  // Update tags
  if let Err(e) = Implementer::update_tags(id, tags).await {
    Logger::log_failed_tag_update(name, e);
    // tracing::warn!(
    //   "failed to update tags on {} {name} | {e:#}",
    //   Resource::display(),
    // );
  } else {
    Logger::log_tags_updated(name);
    // tracing::info!(
    //   "{} {} '{}' tags",
    //   "updated".blue().bold(),
    //   Resource::display(),
    //   name.bold(),
    // );
  }
}

pub async fn run_update_description<Implementer, Resource, Logger>(
  id: String,
  name: &str,
  description: String,
) where
  Implementer: ResourceSync,
  Resource: ResourceSyncOuter<Implementer, Logger>,
  Logger: SyncLogger<Implementer>,
{
  if let Err(e) =
    Implementer::update_description(id, description).await
  {
    Logger::log_failed_description_update(name, e);
    // warn!("failed to update resource {id} description | {e:#}");
  } else {
    Logger::log_description_updated(name);
    // info!(
    //   "{} {} '{}' description",
    //   "updated".blue().bold(),
    //   Resource::display(),
    //   name.bold(),
    // );
  }
}
