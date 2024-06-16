use std::{collections::HashMap, time::Duration};

use anyhow::Context;
use formatting::{bold, colored, muted, Color};
use futures::future::join_all;
use monitor_client::{
  api::{execute::Deploy, read::GetBuildVersions},
  entities::{
    deployment::{
      Deployment, DeploymentConfig, DeploymentImage, DeploymentState,
      PartialDeploymentConfig,
    },
    sync::SyncUpdate,
    tag::Tag,
    toml::ResourceToml,
    update::{Log, ResourceTarget},
    user::sync_user,
  },
};
use mungos::find::find_collect;
use partial_derive2::{Diff, FieldDiff, MaybeNone, PartialDiff};
use resolver_api::Resolve;

use crate::{
  api::execute::ExecuteRequest,
  helpers::update::init_execution_update,
  resource::MonitorResource,
  state::{deployment_status_cache, State},
};

use super::resource::{
  run_update_description, run_update_tags, AllResourcesById,
  ResourceSync,
};

pub type ToUpdate = Vec<ToUpdateItem>;
pub type ToCreate = Vec<ResourceToml<PartialDeploymentConfig>>;
/// Vec of resource names
pub type ToDelete = Vec<String>;

type UpdatesResult = (ToCreate, ToUpdate, ToDelete);

pub struct ToUpdateItem {
  pub id: String,
  pub resource: ResourceToml<PartialDeploymentConfig>,
  pub update_description: bool,
  pub update_tags: bool,
  pub deploy: bool,
}

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

  let mut to_deploy_cache = HashMap::<String, bool>::new();
  let mut to_deploy_build_cache = HashMap::<String, String>::new();

  for mut resource in resources.clone() {
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

        let (to_deploy, state, reason) = extract_to_deploy_and_state(
          all_resources,
          &map,
          &resources,
          resource.name.clone(),
          &mut to_deploy_cache,
          &mut to_deploy_build_cache,
        )
        .await?;

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
              colored(from, Color::Red),
              muted("to"),
              colored(to, Color::Green)
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
          let mut line = if state == DeploymentState::Running {
            format!(
              "{}: {reason}, {}",
              muted("deploy"),
              bold("sync will trigger deploy")
            )
          } else {
            format!(
              "{}: deployment is currently in {} state, {}",
              muted("deploy"),
              colored(&state.to_string(), Color::Red),
              bold("sync will trigger deploy")
            )
          };
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

/// Gets all the resources to update. For use in sync execution.
pub async fn get_updates_for_execution(
  resources: Vec<ResourceToml<PartialDeploymentConfig>>,
  delete: bool,
  all_resources: &AllResourcesById,
  id_to_tags: &HashMap<String, Tag>,
) -> anyhow::Result<UpdatesResult> {
  let map = find_collect(Deployment::coll().await, None, None)
    .await
    .context("failed to get deployments from db")?
    .into_iter()
    .map(|r| (r.name.clone(), r))
    .collect::<HashMap<_, _>>();

  let mut to_create = ToCreate::new();
  let mut to_update = ToUpdate::new();
  let mut to_delete = ToDelete::new();

  if delete {
    for resource in map.values() {
      if !resources.iter().any(|r| r.name == resource.name) {
        to_delete.push(resource.name.clone());
      }
    }
  }

  let mut to_deploy_cache = HashMap::<String, bool>::new();
  let mut to_deploy_build_cache = HashMap::<String, String>::new();

  for mut resource in resources.clone() {
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

        let (to_deploy, _state, _reason) =
          extract_to_deploy_and_state(
            all_resources,
            &map,
            &resources,
            resource.name.clone(),
            &mut to_deploy_cache,
            &mut to_deploy_build_cache,
          )
          .await?;

        // Only proceed if there are any fields to update,
        // or a change to tags / description
        if diff.is_none()
          && !to_deploy
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
          deploy: to_deploy,
        };

        to_update.push(update);
      }
      None => to_create.push(resource),
    }
  }

  Ok((to_create, to_update, to_delete))
}

type Res<'a> = std::pin::Pin<
  Box<
    dyn std::future::Future<
        Output = anyhow::Result<(bool, DeploymentState, String)>,
      > + Send
      + 'a,
  >,
>;

fn extract_to_deploy_and_state<'a>(
  all_resources: &'a AllResourcesById,
  map: &'a HashMap<String, Deployment>,
  resources: &'a [ResourceToml<PartialDeploymentConfig>],
  name: String,
  // name to 'to_deploy'
  cache: &'a mut HashMap<String, bool>,
  // build id to latest built version string
  build_cache: &'a mut HashMap<String, String>,
) -> Res<'a> {
  Box::pin(async move {
    let mut reason = String::new();
    let Some(deployment) = resources.iter().find(|r| r.name == name)
    else {
      // this case should be unreachable, the names come off of a loop over resources
      cache.insert(name, false);
      return Ok((false, DeploymentState::Unknown, reason));
    };
    if deployment.deploy {
      let Some(original) = map.get(&name) else {
        // not created, definitely deploy
        cache.insert(name, true);
        // Don't need reason here, will be populated automatically
        return Ok((true, DeploymentState::NotDeployed, reason));
      };

      // First merge toml resource config (partial) onto default resource config.
      // Makes sure things that aren't defined in toml (come through as None) actually get removed.
      let config: DeploymentConfig = deployment.config.clone().into();
      let mut config: PartialDeploymentConfig = config.into();

      Deployment::validate_partial_config(&mut config);

      let mut diff = Deployment::get_diff(
        original.config.clone(),
        config,
        all_resources,
      )?;

      Deployment::validate_diff(&mut diff);

      let status = &deployment_status_cache()
        .get_or_insert_default(&original.id)
        .await
        .curr;
      let state = status.state;

      let mut to_deploy = match state {
        DeploymentState::Unknown => false,
        DeploymentState::Running => {
          // Needs to only check config fields that affect docker run
          let changed = diff.server_id.is_some()
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
            || diff.labels.is_some();
          if changed {
            reason = String::from("deployment config has changed")
          }
          changed
        }
        // All other cases will require Deploy to enter Running state.
        // Don't need reason here as this case is handled outside, using returned state.
        _ => true,
      };

      // Check if build attached, version latest, and there is a new build.
      if !to_deploy {
        // only need to check original, if diff.image was Some, to_deploy would be true.
        if let DeploymentImage::Build { build_id, version } =
          &original.config.image
        {
          // check if version is none, ie use latest build
          if version.is_none() {
            let deployed_version = status
              .container
              .as_ref()
              .and_then(|c| c.image.split(':').last())
              .unwrap_or("0.0.0");
            match build_cache.get(build_id) {
              Some(version) if deployed_version != version => {
                to_deploy = true;
              }
              Some(_) => {}
              None => {
                let Some(version) = State
                  .resolve(
                    GetBuildVersions {
                      build: build_id.to_string(),
                      limit: Some(1),
                      ..Default::default()
                    },
                    sync_user().to_owned(),
                  )
                  .await
                  .context("failed to get build versions")?
                  .pop()
                else {
                  // this case shouldn't ever happen, how would deployment be deployed if build was never built?
                  return Ok((
                    false,
                    DeploymentState::NotDeployed,
                    reason,
                  ));
                };
                let version = version.version.to_string();
                build_cache
                  .insert(build_id.to_string(), version.clone());
                if deployed_version != version {
                  to_deploy = true;
                  reason =
                    String::from("attached build has new version")
                }
              }
            };
          }
        }
      }

      // Still need to check 'after' if they need deploy
      if !to_deploy {
        for name in &deployment.after {
          match cache.get(name) {
            Some(will_deploy) if *will_deploy => {
              to_deploy = true;
              break;
            }
            Some(_) => {}
            None => {
              let (will_deploy, _, _) = extract_to_deploy_and_state(
                all_resources,
                map,
                resources,
                name.to_string(),
                cache,
                build_cache,
              )
              .await?;
              if will_deploy {
                to_deploy = true;
                reason = format!(
                  "parent dependency '{}' is deploying",
                  bold(name)
                );
                break;
              }
            }
          }
        }
      }

      cache.insert(name, to_deploy);
      Ok((to_deploy, state, reason))
    } else {
      // The state in this case doesn't matter and won't be read (as long as it isn't 'Unknown' which will log in all cases)
      cache.insert(name, false);
      Ok((false, DeploymentState::NotDeployed, reason))
    }
  })
}

pub async fn run_updates(
  to_create: ToCreate,
  to_update: ToUpdate,
  to_delete: ToDelete,
) -> Option<Vec<Log>> {
  if to_create.is_empty()
    && to_update.is_empty()
    && to_delete.is_empty()
  {
    return None;
  }

  let mut has_error = false;
  let mut log = String::new();

  // Collect all the deployment names that need to be deployed
  // and their 'after' dependencies
  let mut to_deploy = Vec::<(String, Vec<String>)>::new();

  for resource in to_create {
    let name = resource.name.clone();
    let tags = resource.tags.clone();
    let description = resource.description.clone();
    let id = match crate::resource::create::<Deployment>(
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
          Deployment::resource_type(),
          bold(&name)
        ));
        continue;
      }
    };
    run_update_tags::<Deployment>(
      id.clone(),
      &name,
      tags,
      &mut log,
      &mut has_error,
    )
    .await;
    run_update_description::<Deployment>(
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
      Deployment::resource_type(),
      bold(&name)
    ));
    if resource.deploy {
      to_deploy.push((resource.name, resource.after));
    }
  }

  for ToUpdateItem {
    id,
    resource,
    update_description,
    update_tags,
    deploy,
  } in to_update
  {
    // Update resource
    let name = resource.name.clone();
    let tags = resource.tags.clone();
    let description = resource.description.clone();

    if update_description {
      run_update_description::<Deployment>(
        id.clone(),
        &name,
        description,
        &mut log,
        &mut has_error,
      )
      .await;
    }

    if update_tags {
      run_update_tags::<Deployment>(
        id.clone(),
        &name,
        tags,
        &mut log,
        &mut has_error,
      )
      .await;
    }

    let mut config_update_error = false;
    if !resource.config.is_none() {
      if let Err(e) = crate::resource::update::<Deployment>(
        &id,
        resource.config,
        sync_user(),
      )
      .await
      {
        has_error = true;
        config_update_error = true;
        log.push_str(&format!(
          "\n{}: failed to update config on {} '{}' | {e:#}",
          colored("ERROR", Color::Red),
          Deployment::resource_type(),
          bold(&name),
        ))
      } else {
        log.push_str(&format!(
          "\n{}: {} {} '{}' configuration",
          muted("INFO"),
          colored("updated", Color::Blue),
          Deployment::resource_type(),
          bold(&name)
        ));
      }
    }

    if !config_update_error && deploy {
      to_deploy.push((resource.name, resource.after));
    }
  }

  for resource in to_delete {
    if let Err(e) =
      crate::resource::delete::<Deployment>(&resource, sync_user())
        .await
    {
      has_error = true;
      log.push_str(&format!(
        "\n{}: failed to delete {} '{}' | {e:#}",
        colored("ERROR", Color::Red),
        Deployment::resource_type(),
        bold(&resource),
      ))
    } else {
      log.push_str(&format!(
        "\n{}: {} {} '{}'",
        muted("INFO"),
        colored("deleted", Color::Red),
        Deployment::resource_type(),
        bold(&resource)
      ));
    }
  }

  let mut logs = Vec::with_capacity(1);

  let stage = format!("Update {}s", Deployment::resource_type());
  if has_error {
    let log = format!(
      "running updates on {}s{log}",
      Deployment::resource_type()
    );
    logs.push(Log::error(&stage, log));
    return Some(logs);
  } else if !log.is_empty() {
    let log = format!(
      "running updates on {}s{log}",
      Deployment::resource_type()
    );
    logs.push(Log::simple(&stage, log));
  }

  if to_deploy.is_empty() {
    return Some(logs);
  }

  let mut log = format!(
    "{}: running executions to sync deployment state",
    muted("INFO")
  );
  let mut round = 1;

  while !to_deploy.is_empty() {
    // Collect all waiting deployments without waiting dependencies.
    let good_to_deploy = to_deploy
      .iter()
      .filter(|(_, after)| {
        to_deploy.iter().all(|(name, _)| !after.contains(name))
      })
      .map(|(name, _)| name.clone())
      .collect::<Vec<_>>();

    // Deploy the ones ready for deployment
    let res =
      join_all(good_to_deploy.iter().map(|name| async move {
        let res = async {
          let req = ExecuteRequest::Deploy(Deploy {
            deployment: name.to_string(),
            stop_signal: None,
            stop_time: None,
          });
          let user = sync_user();
          let update = init_execution_update(&req, user).await?;
          let ExecuteRequest::Deploy(req) = req else {
            unreachable!()
          };
          State.resolve(req, (user.to_owned(), update)).await
        }
        .await;
        (name, res)
      }))
      .await;

    // Log results of deploy
    for (name, res) in res {
      if let Err(e) = res {
        has_error = true;
        log.push_str(&format!(
          "\n{}: failed to deploy '{}' in round {} | {e:#}",
          colored("ERROR", Color::Red),
          bold(name),
          bold(round)
        ));
      } else {
        log.push_str(&format!(
          "\n{}: deployed '{}' in round {}",
          muted("INFO"),
          bold(name),
          bold(round)
        ));
      }
    }

    // Early exit if any deploy has errors
    if has_error {
      log.push_str(&format!(
        "\n{}: exited in round {} {}",
        muted("INFO"),
        bold(round),
        colored("with errors", Color::Red)
      ));
      logs.push(Log::error("Sync Deployment State", log));
      return Some(logs);
    }

    // Remove the deployed ones from 'to_deploy'
    to_deploy.retain(|(name, _)| !good_to_deploy.contains(name));

    // If there must be another round, these are dependent on the first round.
    // Sleep for 1s to allow for first round to startup
    if !to_deploy.is_empty() {
      // Increment the round
      round += 1;
      tokio::time::sleep(Duration::from_secs(1)).await;
    }
  }

  log.push_str(&format!(
    "\n{}: finished after {} round{}",
    muted("INFO"),
    bold(round),
    (round > 1).then_some("s").unwrap_or_default()
  ));

  logs.push(Log::simple("Sync Deployment State", log));

  Some(logs)
}

impl ResourceSync for Deployment {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Deployment(id)
  }

  fn get_diff(
    mut original: Self::Config,
    update: Self::PartialConfig,
    resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    // need to replace the server id with name
    original.server_id = resources
      .servers
      .get(&original.server_id)
      .map(|s| s.name.clone())
      .unwrap_or_default();

    // need to replace the build id with name
    if let DeploymentImage::Build { build_id, version } =
      &original.image
    {
      original.image = DeploymentImage::Build {
        build_id: resources
          .builds
          .get(build_id)
          .map(|b| b.name.clone())
          .unwrap_or_default(),
        version: version.clone(),
      };
    }

    Ok(original.partial_diff(update))
  }
}
