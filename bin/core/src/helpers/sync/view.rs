use std::collections::HashMap;

use anyhow::Context;
use komodo_client::entities::{
  sync::{DiffData, ResourceDiff},
  tag::Tag,
  toml::ResourceToml,
};
use mungos::find::find_collect;
use partial_derive2::MaybeNone;

use super::{AllResourcesById, ResourceSyncTrait};

pub async fn push_updates_for_view<Resource: ResourceSyncTrait>(
  resources: Vec<ResourceToml<Resource::PartialConfig>>,
  delete: bool,
  all_resources: &AllResourcesById,
  all_tags: &HashMap<String, Tag>,
  match_tags: &[String],
  diffs: &mut Vec<ResourceDiff>,
) -> anyhow::Result<()> {
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
            current: super::toml::resource_to_toml::<Resource>(
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

        let proposed =
          super::toml::resource_toml_to_toml::<Resource>(&resource)?;

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
            proposed,
            current: super::toml::resource_to_toml::<Resource>(
              original.clone(),
              all_resources,
              all_tags,
            )?,
          },
        });
      }
      None => {
        diffs.push(ResourceDiff {
          target: Resource::resource_target(resource.name.clone()),
          data: DiffData::Create {
            proposed: super::toml::resource_toml_to_toml::<Resource>(
              &resource,
            )?,
          },
        });
      }
    }
  }

  Ok(())
}
