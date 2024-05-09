use std::collections::HashMap;

use colored::Colorize;
use monitor_client::{
  api::write::{UpdateDescription, UpdateTagsOnResource},
  entities::{
    resource::{Resource, ResourceListItem},
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::{Diff, FieldDiff, MaybeNone, PartialDiff};

use crate::{cli_args, maps::id_to_tag, monitor_client};

pub mod alerter;
pub mod build;
pub mod builder;
pub mod deployment;
pub mod procedure;
pub mod repo;
pub mod server;
pub mod server_template;

type ToUpdate<T> = Vec<ToUpdateItem<T>>;
type ToCreate<T> = Vec<ResourceToml<T>>;
type UpdatesResult<T> = (ToCreate<T>, ToUpdate<T>);

pub struct ToUpdateItem<T> {
  pub id: String,
  pub resource: ResourceToml<T>,
  pub update_description: bool,
  pub update_tags: bool,
}

pub trait ResourceSync {
  type Config: Clone
    + Send
    + PartialDiff<Self::PartialConfig, Self::ConfigDiff>
    + 'static;
  type Info: Default;
  type PartialConfig: std::fmt::Debug
    + Clone
    + Send
    + From<Self::ConfigDiff>
    + 'static;
  type ConfigDiff: Diff + MaybeNone;
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
  ) -> anyhow::Result<Resource<Self::Config, Self::Info>>;

  /// Diffs the declared toml (partial) against the full existing config.
  /// Removes all fields from toml (partial) that haven't changed.
  async fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff>;

  async fn get_updates(
    resources: Vec<ResourceToml<Self::PartialConfig>>,
  ) -> anyhow::Result<UpdatesResult<Self::PartialConfig>> {
    let map = Self::name_to_resource();

    let mut to_create = ToCreate::<Self::PartialConfig>::new();
    let mut to_update = ToUpdate::<Self::PartialConfig>::new();

    let quiet = cli_args().quiet;

    for mut resource in resources {
      match map.get(&resource.name).map(|s| s.id.clone()) {
        Some(id) => {
          // Get the full original config for the resource.
          let original = Self::get(id.clone()).await?;

          let diff =
            Self::get_diff(original.config, resource.config).await?;

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

          if !quiet {
            println!(
              "\n{}: {}: '{}'\n-------------------",
              "UPDATE".blue(),
              Self::display(),
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
          }

          // Minimizes updates through diffing.
          resource.config = diff.into();

          let update = ToUpdateItem {
            id,
            update_description: resource.description
              != original.description,
            update_tags: resource.tags != original_tags,
            resource,
          };

          to_update.push(update);
        }
        None => {
          if !quiet {
            println!(
              "{}: {}: {}: {resource:#?}",
              "CREATE".green(),
              Self::display(),
              resource.name.bold().green(),
            )
          }
          to_create.push(resource);
        }
      }
    }

    if quiet && !to_create.is_empty() {
      println!(
        "\n{}s {}: {:#?}",
        Self::display(),
        "TO CREATE".green(),
        to_create.iter().map(|item| item.name.as_str())
      );
    }

    if quiet && !to_update.is_empty() {
      println!(
        "\n{}s {}: {:#?}",
        Self::display(),
        "TO UPDATE".blue(),
        to_update
          .iter()
          .map(|update| update.resource.name.as_str())
          .collect::<Vec<_>>()
      );
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
      Self::update_description(id, &name, description).await;
      info!("{} {name} created", Self::display());
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
        Self::update_description(id.clone(), &name, description)
          .await;
      }

      if update_tags {
        Self::update_tags(id.clone(), &name, tags).await;
      }

      if let Err(e) = Self::update(id, resource).await {
        warn!(
          "failed to update config on {} {name} | {e:#}",
          Self::display()
        );
      } else {
        info!("updated {} {name} config", Self::display());
      }

      info!("{} {name} updated", Self::display());
    }

    if log_after {
      info!(
        "============ {}s synced ✅ ============",
        Self::display()
      );
    }
  }

  async fn update_tags(id: String, name: &str, tags: Vec<String>) {
    // Update tags
    if let Err(e) = monitor_client()
      .write(UpdateTagsOnResource {
        target: Self::resource_target(id),
        tags,
      })
      .await
    {
      warn!(
        "failed to update tags on {} {name} | {e:#}",
        Self::display(),
      );
    } else {
      info!("updated {} {name} tags", Self::display());
    }
  }

  async fn update_description(
    id: String,
    name: &str,
    description: String,
  ) {
    if let Err(e) = monitor_client()
      .write(UpdateDescription {
        target: Self::resource_target(id.clone()),
        description,
      })
      .await
    {
      warn!("failed to update resource {id} description | {e:#}");
    } else {
      info!("updated {} {name} description", Self::display());
    }
  }
}
