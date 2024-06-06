use std::collections::HashMap;

use colored::Colorize;
use monitor_client::{
  api::{
    execute::Execution,
    write::{CreateProcedure, DeleteProcedure, UpdateProcedure},
  },
  entities::{
    procedure::{
      PartialProcedureConfig, Procedure, ProcedureConfig,
      ProcedureConfigDiff,
    },
    resource::Resource,
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::{MaybeNone, PartialDiff};

use crate::{
  maps::{
    id_to_build, id_to_deployment, id_to_procedure, id_to_repo,
    id_to_resource_sync, id_to_server, name_to_procedure,
  },
  state::monitor_client,
  sync::resource::{
    run_update_description, run_update_tags, ResourceSync, ToCreate,
    ToDelete, ToUpdate, ToUpdateItem,
  },
};

impl ResourceSync for Procedure {
  type Config = ProcedureConfig;
  type Info = ();
  type PartialConfig = PartialProcedureConfig;
  type ConfigDiff = ProcedureConfigDiff;

  fn display() -> &'static str {
    "procedure"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Procedure(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>
  {
    name_to_procedure()
  }

  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(CreateProcedure {
        name: resource.name,
        config: resource.config,
      })
      .await
      .map(|p| p.id)
  }

  async fn update(
    id: String,
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(UpdateProcedure {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn run_updates(
    mut to_create: ToCreate<Self::PartialConfig>,
    mut to_update: ToUpdate<Self::PartialConfig>,
    to_delete: ToDelete,
  ) {
    for name in to_delete {
      if let Err(e) = crate::state::monitor_client()
        .write(DeleteProcedure { id: name.clone() })
        .await
      {
        warn!("failed to delete procedure {name} | {e:#}",);
      } else {
        info!(
          "{} procedure '{}'",
          "deleted".red().bold(),
          name.bold(),
        );
      }
    }

    if to_update.is_empty() && to_create.is_empty() {
      return;
    }

    for i in 0..10 {
      let mut to_pull = Vec::new();
      for ToUpdateItem {
        id,
        resource,
        update_description,
        update_tags,
      } in &to_update
      {
        // Update resource
        let name = resource.name.clone();
        let tags = resource.tags.clone();
        let description = resource.description.clone();
        if *update_description {
          run_update_description::<Procedure>(
            id.clone(),
            &name,
            description,
          )
          .await;
        }
        if *update_tags {
          run_update_tags::<Procedure>(id.clone(), &name, tags).await;
        }
        if !resource.config.is_none() {
          if let Err(e) =
            Self::update(id.clone(), resource.clone()).await
          {
            if i == 9 {
              warn!(
                "failed to update {} {name} | {e:#}",
                Self::display()
              );
            }
          }
        }

        info!("{} {name} updated", Self::display());
        // have to clone out so to_update is mutable
        to_pull.push(id.clone());
      }
      //
      to_update.retain(|resource| !to_pull.contains(&resource.id));

      let mut to_pull = Vec::new();
      for resource in &to_create {
        let name = resource.name.clone();
        let tags = resource.tags.clone();
        let description = resource.description.clone();
        let id = match Self::create(resource.clone()).await {
          Ok(id) => id,
          Err(e) => {
            if i == 9 {
              warn!(
                "failed to create {} {name} | {e:#}",
                Self::display(),
              );
            }
            continue;
          }
        };
        run_update_tags::<Procedure>(id.clone(), &name, tags).await;
        run_update_description::<Procedure>(id, &name, description)
          .await;
        info!("{} {name} created", Self::display());
        to_pull.push(name);
      }
      to_create.retain(|resource| !to_pull.contains(&resource.name));

      if to_update.is_empty() && to_create.is_empty() {
        info!("all procedures synced");
        return;
      }
    }
    warn!("procedure sync loop exited after max iterations");
  }

  fn get_diff(
    mut original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff> {
    for stage in &mut original.stages {
      for execution in &mut stage.executions {
        match &mut execution.execution {
          Execution::None(_) => {}
          Execution::RunProcedure(config) => {
            config.procedure = id_to_procedure()
              .get(&config.procedure)
              .map(|p| p.name.clone())
              .unwrap_or_default();
          }
          Execution::RunBuild(config) => {
            config.build = id_to_build()
              .get(&config.build)
              .map(|b| b.name.clone())
              .unwrap_or_default();
          }
          Execution::Deploy(config) => {
            config.deployment = id_to_deployment()
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::StartContainer(config) => {
            config.deployment = id_to_deployment()
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::StopContainer(config) => {
            config.deployment = id_to_deployment()
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::RemoveContainer(config) => {
            config.deployment = id_to_deployment()
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::CloneRepo(config) => {
            config.repo = id_to_repo()
              .get(&config.repo)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PullRepo(config) => {
            config.repo = id_to_repo()
              .get(&config.repo)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::StopAllContainers(config) => {
            config.server = id_to_server()
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PruneNetworks(config) => {
            config.server = id_to_server()
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PruneImages(config) => {
            config.server = id_to_server()
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PruneContainers(config) => {
            config.server = id_to_server()
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::RunSync(config) => {
            config.sync = id_to_resource_sync()
              .get(&config.sync)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
        }
      }
    }
    Ok(original.partial_diff(update))
  }

  async fn delete(_: String) -> anyhow::Result<()> {
    unreachable!()
  }
}
