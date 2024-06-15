use std::collections::HashMap;

use anyhow::Context;
use monitor_client::entities::{
  deployment::{
    Deployment, DeploymentConfig, DeploymentState,
    PartialDeploymentConfig,
  },
  sync::SyncUpdate,
  tag::Tag,
  toml::ResourceToml,
};
use mungos::find::find_collect;
use partial_derive2::{Diff, FieldDiff, MaybeNone};

use crate::{
  helpers::formatting::{bold, colored, muted, Color},
  resource::MonitorResource,
  state::deployment_status_cache,
};

use super::resource::{AllResourcesById, ResourceSync};

/// Turns all the diffs into a readable string
pub async fn get_updates_for_view(
  resources: Vec<ResourceToml<PartialDeploymentConfig>>,
  delete: bool,
  all_resources: &AllResourcesById,
  id_to_tags: &HashMap<String, Tag>,
) -> anyhow::Result<Option<SyncUpdate>> {
  let map = find_collect(Deployment::coll().await, None, None)
    .await
    .context("failed to get deployments from db")?
    .into_iter()
    .map(|r| (r.name.clone(), r))
    .collect::<HashMap<_, _>>();

  let mut update = SyncUpdate {
    log: format!("{} Updates", Deployment::resource_type()),
    ..Default::default()
  };

  let mut to_delete = Vec::<String>::new();
  if delete {
    for resource in map.values() {
      if !resources.iter().any(|r| r.name == resource.name) {
        update.to_delete += 1;
        to_delete.push(resource.name.clone())
      }
    }
  }

  let status_cache = deployment_status_cache();

  for mut resource in resources {
    match map.get(&resource.name) {
      Some(original) => {
        // First merge toml resource config (partial) onto default resource config.
        // Makes sure things that aren't defined in toml (come through as None) actually get removed.
        let config: DeploymentConfig = resource.config.into();
        resource.config = config.into();

        Deployment::validate_partial_config(&mut resource.config);

        let mut diff = Deployment::get_diff(
          original.config.clone(),
          resource.config,
          all_resources,
        )?;

        Deployment::validate_diff(&mut diff);

        let original_tags = original
          .tags
          .iter()
          .filter_map(|id| id_to_tags.get(id).map(|t| t.name.clone()))
          .collect::<Vec<_>>();

        let (to_deploy, state) = if resource.deploy {
          let state = status_cache
            .get_or_insert_default(&original.id)
            .await
            .curr
            .state;
          let to_deploy = match state {
            DeploymentState::Unknown => false,
            DeploymentState::Running => {
              // Needs to only check config fields that affect docker run
              diff.server_id.is_some()
                || diff.image.is_some()
                || diff.image_registry.is_some()
                || diff.skip_secret_interp.is_some()
                || diff.network.is_some()
                || diff.restart.is_some()
                || diff.command.is_some()
                || diff.extra_args.is_some()
                || diff.ports.is_some()
                || diff.volumes.is_some()
                || diff.environment.is_some()
                || diff.labels.is_some()
            }
            // All other cases will require Deploy to enter Running state
            _ => true,
          };
          (to_deploy, state)
        } else {
          // The state in this case doesn't matter and won't be read (as long as it isn't 'Unknown' which will log in all cases)
          (false, DeploymentState::NotDeployed)
        };

        // Only proceed if there are any fields to update,
        // or a change to tags / description
        if diff.is_none()
          && !to_deploy
          && resource.description == original.description
          && resource.tags == original_tags
        {
          if state == DeploymentState::Unknown {
            update.log.push_str(&format!(
              "\n\n{}: {}: '{}'\nDeployment sync actions could not be computed due to Unknown deployment state\n-------------------",
              colored("ERROR", Color::Red),
              Deployment::resource_type(),
              bold(&resource.name)
            ));
          }
          continue;
        }

        update.to_update += 1;

        update.log.push_str(&format!(
          "\n\n{}: {}: '{}'\n-------------------",
          colored("UPDATE", Color::Blue),
          Deployment::resource_type(),
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
            colored(&format!("{:?}", original_tags), Color::Red);
          let to =
            colored(&format!("{:?}", resource.tags), Color::Green);
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
              colored(&from, Color::Red),
              muted("to"),
              colored(&to, Color::Green)
            )
          },
        ));
        if state == DeploymentState::Unknown {
          lines.push(format!(
						"{}: Deployment sync actions {} due to Unknown deployment state",
						colored("ERROR", Color::Red),
						bold("could not be computed")
					));
        } else if to_deploy {
          let mut line = format!(
            "{}: deployment is currently in {} state, {}",
            muted("deploy"),
            colored(&state.to_string(), Color::Red),
            bold("sync will trigger deploy")
          );
          if !resource.after.is_empty() {
            line.push_str(&format!(
              "\n{}: {:?}",
              muted("deploy after"),
              resource.after
            ));
          }
          lines.push(line);
        }
        update.log.push('\n');
        update.log.push_str(&lines.join("\n-------------------\n"));
      }
      None => {
        update.to_create += 1;
        let mut lines = vec![
          format!(
            "{}: {}",
            muted("description"),
            resource.description,
          ),
          format!("{}: {:?}", muted("tags"), resource.tags,),
          format!(
            "{}: {}",
            muted("config"),
            serde_json::to_string_pretty(&resource.config)
              .context("failed to serialize config to json")?
          ),
        ];
        if resource.deploy {
          lines.push(format!(
            "{}: {}",
            muted("will deploy"),
            colored("true", Color::Green)
          ));
          if !resource.after.is_empty() {
            lines.push(format!(
              "{}: {:?}",
              muted("deploy after"),
              resource.after
            ));
          }
        }
        update.log.push_str(&format!(
          "\n\n{}: {}: {}\n{}",
          colored("CREATE", Color::Green),
          Deployment::resource_type(),
          bold(&resource.name),
          lines.join("\n")
        ))
      }
    }
  }

  for name in to_delete {
    update.log.push_str(&format!(
      "\n\n{}: {}: '{}'\n-------------------",
      colored("DELETE", Color::Red),
      Deployment::resource_type(),
      bold(&name)
    ));
  }

  let any_change = update.to_create > 0
    || update.to_update > 0
    || update.to_delete > 0;

  Ok(any_change.then_some(update))
}

// /// Gets all the resources to update. For use in sync execution.
// pub async fn get_updates_for_execution<Resource: ResourceSync>(
//   resources: Vec<ResourceToml<Resource::PartialConfig>>,
//   delete: bool,
//   all_resources: &AllResourcesById,
//   id_to_tags: &HashMap<String, Tag>,
// ) -> anyhow::Result<UpdatesResult<Resource::PartialConfig>> {
//   let map = find_collect(Resource::coll().await, None, None)
//     .await
//     .context("failed to get resources from db")?
//     .into_iter()
//     .map(|r| (r.name.clone(), r))
//     .collect::<HashMap<_, _>>();

//   let mut to_create = ToCreate::<Resource::PartialConfig>::new();
//   let mut to_update = ToUpdate::<Resource::PartialConfig>::new();
//   let mut to_delete = ToDelete::new();

//   if delete {
//     for resource in map.values() {
//       if !resources.iter().any(|r| r.name == resource.name) {
//         to_delete.push(resource.name.clone());
//       }
//     }
//   }

//   for mut resource in resources {
//     match map.get(&resource.name) {
//       Some(original) => {
//         // First merge toml resource config (partial) onto default resource config.
//         // Makes sure things that aren't defined in toml (come through as None) actually get removed.
//         let config: Resource::Config = resource.config.into();
//         resource.config = config.into();

//         Resource::validate_partial_config(&mut resource.config);

//         let mut diff = Resource::get_diff(
//           original.config.clone(),
//           resource.config,
//           all_resources,
//         )?;

//         Resource::validate_diff(&mut diff);

//         let original_tags = original
//           .tags
//           .iter()
//           .filter_map(|id| id_to_tags.get(id).map(|t| t.name.clone()))
//           .collect::<Vec<_>>();

//         // Only proceed if there are any fields to update,
//         // or a change to tags / description
//         if diff.is_none()
//           && resource.description == original.description
//           && resource.tags == original_tags
//         {
//           continue;
//         }

//         // Minimizes updates through diffing.
//         resource.config = diff.into();

//         let update = ToUpdateItem {
//           id: original.id.clone(),
//           update_description: resource.description
//             != original.description,
//           update_tags: resource.tags != original_tags,
//           resource,
//         };

//         to_update.push(update);
//       }
//       None => to_create.push(resource),
//     }
//   }

//   Ok((to_create, to_update, to_delete))
// }
