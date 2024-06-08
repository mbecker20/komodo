use std::collections::HashMap;

use colored::Colorize;
use monitor_client::{
  api::write::{UpdateDescription, UpdateTagsOnResource},
  entities::{
    resource::Resource, toml::ResourceToml, update::ResourceTarget,
  },
};
use partial_derive2::{Diff, FieldDiff, MaybeNone, PartialDiff};
use serde::Serialize;

use crate::maps::id_to_tag;

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
    + From<Self::ConfigDiff>
    + Serialize
    + MaybeNone
    + 'static;
  type ConfigDiff: Diff + MaybeNone;

  fn display() -> &'static str;

  fn resource_target(id: String) -> ResourceTarget;

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>;

  /// Creates the resource and returns created id.
  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String>;

  /// Updates the resource at id with the partial config.
  async fn update(
    id: String,
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<()>;

  /// Apply any changes to incoming toml partial config
  /// before it is diffed against existing config
  fn validate_partial_config(_config: &mut Self::PartialConfig) {}

  /// Diffs the declared toml (partial) against the full existing config.
  /// Removes all fields from toml (partial) that haven't changed.
  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff>;

  /// Apply any changes to computed config diff
  /// before logging
  fn validate_diff(_diff: &mut Self::ConfigDiff) {}

  /// Deletes the target resource
  async fn delete(id_or_name: String) -> anyhow::Result<()>;

  async fn run_updates(
    to_create: ToCreate<Self::PartialConfig>,
    to_update: ToUpdate<Self::PartialConfig>,
    to_delete: ToDelete,
  ) {
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
      run_update_tags::<Self>(id.clone(), &name, tags).await;
      run_update_description::<Self>(id, &name, description).await;
      info!(
        "{} {} '{}'",
        "created".green().bold(),
        Self::display(),
        name.bold(),
      );
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
        run_update_description::<Self>(
          id.clone(),
          &name,
          description,
        )
        .await;
      }

      if update_tags {
        run_update_tags::<Self>(id.clone(), &name, tags).await;
      }

      if !resource.config.is_none() {
        if let Err(e) = Self::update(id, resource).await {
          warn!(
            "failed to update config on {} {name} | {e:#}",
            Self::display()
          );
        } else {
          info!(
            "{} {} '{}' configuration",
            "updated".blue().bold(),
            Self::display(),
            name.bold(),
          );
        }
      }
    }

    for resource in to_delete {
      if let Err(e) = Self::delete(resource.clone()).await {
        warn!(
          "failed to delete {} {resource} | {e:#}",
          Self::display()
        );
      } else {
        info!(
          "{} {} '{}'",
          "deleted".red().bold(),
          Self::display(),
          resource.bold(),
        );
      }
    }
  }
}

/// Gets all the resources to update, logging along the way.
pub fn get_updates<Resource: ResourceSync>(
  resources: Vec<ResourceToml<Resource::PartialConfig>>,
  delete: bool,
) -> anyhow::Result<UpdatesResult<Resource::PartialConfig>> {
  let map = Resource::name_to_resource();

  let mut to_create = ToCreate::<Resource::PartialConfig>::new();
  let mut to_update = ToUpdate::<Resource::PartialConfig>::new();
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
        let config: Resource::Config = resource.config.into();
        resource.config = config.into();

        Resource::validate_partial_config(&mut resource.config);

        let mut diff = Resource::get_diff(
          original.config.clone(),
          resource.config,
        )?;

        Resource::validate_diff(&mut diff);

        let original_tags = original
          .tags
          .iter()
          .filter_map(|id| {
            id_to_tag().get(id).map(|t| t.name.clone())
          })
          .collect::<Vec<_>>();

        // Only proceed if there are any fields to update,
        // or a change to tags / description
        if diff.is_none()
          && resource.description == original.description
          && resource.tags == original_tags
        {
          continue;
        }

        println!(
          "\n{}: {}: '{}'\n-------------------",
          "UPDATE".blue(),
          Resource::display(),
          resource.name.bold(),
        );
        let mut lines = Vec::<String>::new();
        if resource.description != original.description {
          lines.push(format!(
            "{}: 'description'\n{}:  {}\n{}:    {}",
            "field".dimmed(),
            "from".dimmed(),
            original.description.red(),
            "to".dimmed(),
            resource.description.green()
          ))
        }
        if resource.tags != original_tags {
          let from = format!("{:?}", original_tags).red();
          let to = format!("{:?}", resource.tags).green();
          lines.push(format!(
            "{}: 'tags'\n{}:  {from}\n{}:    {to}",
            "field".dimmed(),
            "from".dimmed(),
            "to".dimmed(),
          ));
        }
        lines.extend(diff.iter_field_diffs().map(
          |FieldDiff { field, from, to }| {
            format!(
              "{}: '{field}'\n{}:  {}\n{}:    {}",
              "field".dimmed(),
              "from".dimmed(),
              from.red(),
              "to".dimmed(),
              to.green()
            )
          },
        ));
        println!("{}", lines.join("\n-------------------\n"));

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
        println!(
          "\n{}: {}: {}\n{}: {}\n{}: {:?}\n{}: {}",
          "CREATE".green(),
          Resource::display(),
          resource.name.bold().green(),
          "description".dimmed(),
          resource.description,
          "tags".dimmed(),
          resource.tags,
          "config".dimmed(),
          serde_json::to_string_pretty(&resource.config)?
        );
        to_create.push(resource);
      }
    }
  }

  for name in &to_delete {
    println!(
      "\n{}: {}: '{}'\n-------------------",
      "DELETE".red(),
      Resource::display(),
      name.bold(),
    );
  }

  Ok((to_create, to_update, to_delete))
}

pub async fn run_update_tags<Resource: ResourceSync>(
  id: String,
  name: &str,
  tags: Vec<String>,
) {
  // Update tags
  if let Err(e) = crate::state::monitor_client()
    .write(UpdateTagsOnResource {
      target: Resource::resource_target(id),
      tags,
    })
    .await
  {
    warn!(
      "failed to update tags on {} {name} | {e:#}",
      Resource::display(),
    );
  } else {
    info!(
      "{} {} '{}' tags",
      "updated".blue().bold(),
      Resource::display(),
      name.bold(),
    );
  }
}

pub async fn run_update_description<Resource: ResourceSync>(
  id: String,
  name: &str,
  description: String,
) {
  if let Err(e) = crate::state::monitor_client()
    .write(UpdateDescription {
      target: Resource::resource_target(id.clone()),
      description,
    })
    .await
  {
    warn!("failed to update resource {id} description | {e:#}");
  } else {
    info!(
      "{} {} '{}' description",
      "updated".blue().bold(),
      Resource::display(),
      name.bold(),
    );
  }
}
