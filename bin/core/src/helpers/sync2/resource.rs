use std::{collections::HashMap, str::FromStr};

use anyhow::Context;
use formatting::{bold, colored, muted, Color};
use komodo_client::{
  api::write::{UpdateDescription, UpdateTagsOnResource},
  entities::{
    self,
    alerter::Alerter,
    build::Build,
    builder::Builder,
    deployment::Deployment,
    procedure::Procedure,
    repo::Repo,
    server::Server,
    server_template::ServerTemplate,
    stack::Stack,
    sync::{DiffData, ResourceDiff},
    tag::Tag,
    toml::ResourceToml,
    update::Log,
    user::sync_user,
    ResourceTarget,
  },
};
use mungos::{find::find_collect, mongodb::bson::oid::ObjectId};
use partial_derive2::{Diff, FieldDiff, MaybeNone};
use resolver_api::Resolve;

use crate::{resource::KomodoResource, state::State};

use super::toml::{resource_toml_to_toml, ToToml};

pub type ToUpdate<T> = Vec<ToUpdateItem<T>>;
pub type ToCreate<T> = Vec<ResourceToml<T>>;
/// Vec of resource names
pub type ToDelete = Vec<String>;

type UpdatesResult<T> = (ToCreate<T>, ToUpdate<T>, ToDelete);

pub struct ToUpdateItem<T: Default> {
  pub id: String,
  pub resource: ResourceToml<T>,
  pub update_description: bool,
  pub update_tags: bool,
}

pub fn include_resource_by_tags(
  resource_tags: &[String],
  id_to_tags: &HashMap<String, Tag>,
  match_tags: &[String],
) -> bool {
  let tag_names = resource_tags
    .iter()
    .filter_map(|resource_tag| {
      match ObjectId::from_str(resource_tag) {
        Ok(_) => id_to_tags.get(resource_tag).map(|tag| &tag.name),
        Err(_) => Some(resource_tag),
      }
    })
    .collect::<Vec<_>>();
  match_tags.iter().all(|tag| tag_names.contains(&tag))
}

pub trait ResourceSyncTrait: ToToml + Sized {
  fn resource_target(id: String) -> ResourceTarget;

  /// To exclude resource syncs with "file_contents" (they aren't compatible)
  fn include_resource(
    _config: &Self::Config,
    resource_tags: &[String],
    id_to_tags: &HashMap<String, Tag>,
    match_tags: &[String],
  ) -> bool {
    include_resource_by_tags(resource_tags, id_to_tags, match_tags)
  }

  /// To exclude resource syncs with "file_contents" (they aren't compatible)
  fn include_resource_partial(
    _config: &Self::PartialConfig,
    resource_tags: &[String],
    id_to_tags: &HashMap<String, Tag>,
    match_tags: &[String],
  ) -> bool {
    include_resource_by_tags(resource_tags, id_to_tags, match_tags)
  }

  /// Apply any changes to incoming toml partial config
  /// before it is diffed against existing config
  fn validate_partial_config(_config: &mut Self::PartialConfig) {}

  /// Diffs the declared toml (partial) against the full existing config.
  /// Removes all fields from toml (partial) that haven't changed.
  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
    resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff>;

  /// Apply any changes to computed config diff
  /// before logging
  fn validate_diff(_diff: &mut Self::ConfigDiff) {}

  async fn run_updates(
    to_create: ToCreate<Self::PartialConfig>,
    to_update: ToUpdate<Self::PartialConfig>,
    to_delete: ToDelete,
  ) -> Option<Log> {
    if to_create.is_empty()
      && to_update.is_empty()
      && to_delete.is_empty()
    {
      return None;
    }

    let mut has_error = false;
    let mut log =
      format!("running updates on {}s", Self::resource_type());

    for resource in to_create {
      let name = resource.name.clone();
      let tags = resource.tags.clone();
      let description = resource.description.clone();
      let id = match crate::resource::create::<Self>(
        &resource.name,
        resource.config,
        sync_user(),
      )
      .await
      {
        Ok(resource) => resource.id,
        Err(e) => {
          has_error = true;
          log.push_str(&format!(
            "\n{}: failed to create {} '{}' | {e:#}",
            colored("ERROR", Color::Red),
            Self::resource_type(),
            bold(&name)
          ));
          continue;
        }
      };
      run_update_tags::<Self>(
        id.clone(),
        &name,
        tags,
        &mut log,
        &mut has_error,
      )
      .await;
      run_update_description::<Self>(
        id,
        &name,
        description,
        &mut log,
        &mut has_error,
      )
      .await;
      log.push_str(&format!(
        "\n{}: {} {} '{}'",
        muted("INFO"),
        colored("created", Color::Green),
        Self::resource_type(),
        bold(&name)
      ));
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
          &mut log,
          &mut has_error,
        )
        .await;
      }

      if update_tags {
        run_update_tags::<Self>(
          id.clone(),
          &name,
          tags,
          &mut log,
          &mut has_error,
        )
        .await;
      }

      if !resource.config.is_none() {
        if let Err(e) = crate::resource::update::<Self>(
          &id,
          resource.config,
          sync_user(),
        )
        .await
        {
          has_error = true;
          log.push_str(&format!(
            "\n{}: failed to update config on {} '{}' | {e:#}",
            colored("ERROR", Color::Red),
            Self::resource_type(),
            bold(&name),
          ))
        } else {
          log.push_str(&format!(
            "\n{}: {} {} '{}' configuration",
            muted("INFO"),
            colored("updated", Color::Blue),
            Self::resource_type(),
            bold(&name)
          ));
        }
      }
    }

    for resource in to_delete {
      if let Err(e) =
        crate::resource::delete::<Self>(&resource, sync_user()).await
      {
        has_error = true;
        log.push_str(&format!(
          "\n{}: failed to delete {} '{}' | {e:#}",
          colored("ERROR", Color::Red),
          Self::resource_type(),
          bold(&resource),
        ))
      } else {
        log.push_str(&format!(
          "\n{}: {} {} '{}'",
          muted("INFO"),
          colored("deleted", Color::Red),
          Self::resource_type(),
          bold(&resource)
        ));
      }
    }

    let stage = format!("Update {}s", Self::resource_type());
    Some(if has_error {
      Log::error(&stage, log)
    } else {
      Log::simple(&stage, log)
    })
  }
}

/// Turns all the diffs into a readable string
pub async fn push_updates_for_view<Resource: ResourceSyncTrait>(
  resources: Vec<ResourceToml<Resource::PartialConfig>>,
  delete: bool,
  all_resources: &AllResourcesById,
  all_tags: &HashMap<String, Tag>,
  match_tags: &[String],
  diffs: &mut Vec<ResourceDiff>,
) -> anyhow::Result<Vec<ResourceDiff>> {
  let map = find_collect(Resource::coll().await, None, None)
    .await
    .context("failed to get resources from db")?
    .into_iter()
    .filter(|r| {
      Resource::include_resource(
        &r.config, &r.tags, all_tags, match_tags,
      )
    })
    .map(|r| (r.name.clone(), r))
    .collect::<HashMap<_, _>>();

  if delete {
    for resource in map.values() {
      if !resources.iter().any(|r| r.name == resource.name) {
        diffs.push(ResourceDiff {
          target: Resource::resource_target(resource.name.clone()),
          data: DiffData::Delete {
            current: super::toml::resource_to_toml(
              resource.clone(),
              all_resources,
              all_tags,
            )?,
          },
        });
      }
    }
  }

  for mut resource in resources {
    // only resource that might not be included is resource sync
    if !Resource::include_resource_partial(
      &resource.config,
      &resource.tags,
      all_tags,
      match_tags,
    ) {
      continue;
    }
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
          all_resources,
        )?;

        Resource::validate_diff(&mut diff);

        let original_tags = original
          .tags
          .iter()
          .filter_map(|id| all_tags.get(id).map(|t| t.name.clone()))
          .collect::<Vec<_>>();

        // Only proceed if there are any fields to update,
        // or a change to tags / description
        if diff.is_none()
          && resource.description == original.description
          && resource.tags == original_tags
        {
          continue;
        }

        diffs.push(ResourceDiff {
          target: Resource::resource_target(resource.name.clone()),
          data: DiffData::Update {
            proposed: resource_toml_to_toml(&resource)?,
            current: super::toml::resource_to_toml(
              original.clone(),
              all_resources,
              all_tags,
            )?,
          },
        });

        update.to_update += 1;

        update.log.push_str(&format!(
          "\n\n{}: {}: '{}'\n-------------------",
          colored("UPDATE", Color::Blue),
          Resource::resource_type(),
          bold(&resource.name)
        ));

        let mut lines = Vec::<String>::new();
        if resource.description != original.description {
          lines.push(format!(
            "{}: 'description'\n{}:  {}\n{}:    {}",
            muted("field"),
            muted("from"),
            colored(&original.description, Color::Red),
            muted("to"),
            colored(&resource.description, Color::Green)
          ));
        }
        if resource.tags != original_tags {
          let from =
            colored(format!("{:?}", original_tags), Color::Red);
          let to =
            colored(format!("{:?}", resource.tags), Color::Green);
          lines.push(format!(
            "{}: 'tags'\n{}:  {from}\n{}:    {to}",
            muted("field"),
            muted("from"),
            muted("to"),
          ));
        }
        lines.extend(diff.iter_field_diffs().map(
          |FieldDiff { field, from, to }| {
            format!(
              "{}: '{field}'\n{}:  {}\n{}:    {}",
              muted("field"),
              muted("from"),
              colored(from, Color::Red),
              muted("to"),
              colored(to, Color::Green)
            )
          },
        ));
        update.log.push('\n');
        update.log.push_str(&lines.join("\n-------------------\n"));
      }
      None => {
        update.to_create += 1;
        update.log.push_str(&format!(
          "\n\n{}: {}: {}\n{}: {}\n{}: {:?}\n{}: {}",
          colored("CREATE", Color::Green),
          Resource::resource_type(),
          bold(&resource.name),
          muted("description"),
          resource.description,
          muted("tags"),
          resource.tags,
          muted("config"),
          serde_json::to_string_pretty(&resource.config)
            .context("failed to serialize config to json")?
        ))
      }
    }
  }

  Ok(diffs)
}

/// Gets all the resources to update. For use in sync execution.
pub async fn get_updates_for_execution<
  Resource: ResourceSyncTrait,
>(
  resources: Vec<ResourceToml<Resource::PartialConfig>>,
  delete: bool,
  all_resources: &AllResourcesById,
  id_to_tags: &HashMap<String, Tag>,
  match_tags: &[String],
) -> anyhow::Result<UpdatesResult<Resource::PartialConfig>> {
  let map = find_collect(Resource::coll().await, None, None)
    .await
    .context("failed to get resources from db")?
    .into_iter()
    .filter(|r| {
      Resource::include_resource(
        &r.config, &r.tags, id_to_tags, match_tags,
      )
    })
    .map(|r| (r.name.clone(), r))
    .collect::<HashMap<_, _>>();

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
    // only resource that might not be included is resource sync
    if !Resource::include_resource_partial(
      &resource.config,
      &resource.tags,
      id_to_tags,
      match_tags,
    ) {
      continue;
    }
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
          all_resources,
        )?;

        Resource::validate_diff(&mut diff);

        let original_tags = original
          .tags
          .iter()
          .filter_map(|id| id_to_tags.get(id).map(|t| t.name.clone()))
          .collect::<Vec<_>>();

        // Only proceed if there are any fields to update,
        // or a change to tags / description
        if diff.is_none()
          && resource.description == original.description
          && resource.tags == original_tags
        {
          continue;
        }

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
      None => to_create.push(resource),
    }
  }

  Ok((to_create, to_update, to_delete))
}

pub async fn run_update_tags<Resource: ResourceSyncTrait>(
  id: String,
  name: &str,
  tags: Vec<String>,
  log: &mut String,
  has_error: &mut bool,
) {
  // Update tags
  if let Err(e) = State
    .resolve(
      UpdateTagsOnResource {
        target: Resource::resource_target(id),
        tags,
      },
      sync_user().to_owned(),
    )
    .await
  {
    *has_error = true;
    log.push_str(&format!(
      "\n{}: failed to update tags on {} '{}' | {e:#}",
      colored("ERROR", Color::Red),
      Resource::resource_type(),
      bold(name),
    ))
  } else {
    log.push_str(&format!(
      "\n{}: {} {} '{}' tags",
      muted("INFO"),
      colored("updated", Color::Blue),
      Resource::resource_type(),
      bold(name)
    ));
  }
}

pub async fn run_update_description<Resource: ResourceSyncTrait>(
  id: String,
  name: &str,
  description: String,
  log: &mut String,
  has_error: &mut bool,
) {
  if let Err(e) = State
    .resolve(
      UpdateDescription {
        target: Resource::resource_target(id.clone()),
        description,
      },
      sync_user().to_owned(),
    )
    .await
  {
    *has_error = true;
    log.push_str(&format!(
      "\n{}: failed to update description on {} '{}' | {e:#}",
      colored("ERROR", Color::Red),
      Resource::resource_type(),
      bold(name),
    ))
  } else {
    log.push_str(&format!(
      "\n{}: {} {} '{}' description",
      muted("INFO"),
      colored("updated", Color::Blue),
      Resource::resource_type(),
      bold(name)
    ));
  }
}

pub struct AllResourcesById {
  pub servers: HashMap<String, Server>,
  pub deployments: HashMap<String, Deployment>,
  pub stacks: HashMap<String, Stack>,
  pub builds: HashMap<String, Build>,
  pub repos: HashMap<String, Repo>,
  pub procedures: HashMap<String, Procedure>,
  pub builders: HashMap<String, Builder>,
  pub alerters: HashMap<String, Alerter>,
  pub templates: HashMap<String, ServerTemplate>,
  pub syncs: HashMap<String, entities::sync::ResourceSync>,
}

impl AllResourcesById {
  /// Use `match_tags` to filter resources by tag.
  pub async fn load(
    id_to_tags: &HashMap<String, Tag>,
    match_tags: &[String],
  ) -> anyhow::Result<Self> {
    Ok(Self {
      servers: crate::resource::get_id_to_resource_map::<Server>(
        id_to_tags, match_tags,
      )
      .await?,
      deployments: crate::resource::get_id_to_resource_map::<
        Deployment,
      >(id_to_tags, match_tags)
      .await?,
      builds: crate::resource::get_id_to_resource_map::<Build>(
        id_to_tags, match_tags,
      )
      .await?,
      repos: crate::resource::get_id_to_resource_map::<Repo>(
        id_to_tags, match_tags,
      )
      .await?,
      procedures:
        crate::resource::get_id_to_resource_map::<Procedure>(
          id_to_tags, match_tags,
        )
        .await?,
      builders: crate::resource::get_id_to_resource_map::<Builder>(
        id_to_tags, match_tags,
      )
      .await?,
      alerters: crate::resource::get_id_to_resource_map::<Alerter>(
        id_to_tags, match_tags,
      )
      .await?,
      templates: crate::resource::get_id_to_resource_map::<
        ServerTemplate,
      >(id_to_tags, match_tags)
      .await?,
      syncs: crate::resource::get_id_to_resource_map::<
        entities::sync::ResourceSync,
      >(id_to_tags, match_tags)
      .await?,
      stacks: crate::resource::get_id_to_resource_map::<
        entities::stack::Stack,
      >(id_to_tags, match_tags)
      .await?,
    })
  }
}
