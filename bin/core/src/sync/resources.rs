use std::collections::HashMap;

use formatting::{bold, colored, muted, Color};
use komodo_client::{
  api::execute::Execution,
  entities::{
    action::Action,
    alerter::Alerter,
    build::Build,
    builder::{Builder, BuilderConfig},
    deployment::{Deployment, DeploymentImage},
    procedure::Procedure,
    repo::Repo,
    server::Server,
    server_template::ServerTemplate,
    stack::Stack,
    sync::ResourceSync,
    tag::Tag,
    update::Log,
    user::sync_user,
    ResourceTarget, ResourceTargetVariant,
  },
};
use partial_derive2::{MaybeNone, PartialDiff};

use crate::{
  api::write::WriteArgs,
  resource::KomodoResource,
  sync::{
    execute::{run_update_description, run_update_tags},
    ToUpdateItem,
  },
};

use super::{
  execute::ExecuteResourceSync,
  include_resource_by_resource_type_and_name,
  include_resource_by_tags, AllResourcesById, ResourceSyncTrait,
  ToCreate, ToDelete, ToUpdate,
};

impl ResourceSyncTrait for Server {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Server(id)
  }

  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
    _resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    Ok(original.partial_diff(update))
  }
}

impl ExecuteResourceSync for Server {}

impl ResourceSyncTrait for Deployment {
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
        version: *version,
      };
    }

    Ok(original.partial_diff(update))
  }
}

impl ExecuteResourceSync for Deployment {}

impl ResourceSyncTrait for Stack {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Stack(id)
  }

  fn get_diff(
    mut original: Self::Config,
    update: Self::PartialConfig,
    resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    // Need to replace server id with name
    original.server_id = resources
      .servers
      .get(&original.server_id)
      .map(|s| s.name.clone())
      .unwrap_or_default();

    Ok(original.partial_diff(update))
  }
}

impl ExecuteResourceSync for Stack {}

impl ResourceSyncTrait for Build {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Build(id)
  }

  fn get_diff(
    mut original: Self::Config,
    update: Self::PartialConfig,
    resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    original.builder_id = resources
      .builders
      .get(&original.builder_id)
      .map(|b| b.name.clone())
      .unwrap_or_default();

    Ok(original.partial_diff(update))
  }

  fn validate_diff(diff: &mut Self::ConfigDiff) {
    if let Some((_, to)) = &diff.version {
      // When setting a build back to "latest" version,
      // Don't actually set version to None.
      // You can do this on the db, or set it to 0.0.1
      if to.is_none() {
        diff.version = None;
      }
    }
  }
}

impl ExecuteResourceSync for Build {}

impl ResourceSyncTrait for Repo {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Repo(id)
  }

  fn get_diff(
    mut original: Self::Config,
    update: Self::PartialConfig,
    resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    // Need to replace server id with name
    original.server_id = resources
      .servers
      .get(&original.server_id)
      .map(|s| s.name.clone())
      .unwrap_or_default();

    // Need to replace builder id with name
    original.builder_id = resources
      .builders
      .get(&original.builder_id)
      .map(|s| s.name.clone())
      .unwrap_or_default();

    Ok(original.partial_diff(update))
  }
}

impl ExecuteResourceSync for Repo {}

impl ResourceSyncTrait for Alerter {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Alerter(id)
  }

  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
    _resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    Ok(original.partial_diff(update))
  }
}

impl ExecuteResourceSync for Alerter {}

impl ResourceSyncTrait for Builder {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Builder(id)
  }

  fn get_diff(
    mut original: Self::Config,
    update: Self::PartialConfig,
    resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    // need to replace server builder id with name
    if let BuilderConfig::Server(config) = &mut original {
      config.server_id = resources
        .servers
        .get(&config.server_id)
        .map(|s| s.name.clone())
        .unwrap_or_default();
    }

    Ok(original.partial_diff(update))
  }
}

impl ExecuteResourceSync for Builder {}

impl ResourceSyncTrait for ServerTemplate {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::ServerTemplate(id)
  }

  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
    _resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    Ok(original.partial_diff(update))
  }
}

impl ExecuteResourceSync for ServerTemplate {}

impl ResourceSyncTrait for Action {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Action(id)
  }

  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
    _resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    Ok(original.partial_diff(update))
  }
}

impl ExecuteResourceSync for Action {}

impl ResourceSyncTrait for ResourceSync {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::ResourceSync(id)
  }

  fn include_resource(
    name: &String,
    config: &Self::Config,
    match_resource_type: Option<ResourceTargetVariant>,
    match_resources: Option<&[String]>,
    resource_tags: &[String],
    id_to_tags: &HashMap<String, Tag>,
    match_tags: &[String],
  ) -> bool {
    if !include_resource_by_resource_type_and_name::<ResourceSync>(
      match_resource_type,
      match_resources,
      name,
    ) || !include_resource_by_tags(
      resource_tags,
      id_to_tags,
      match_tags,
    ) {
      return false;
    }
    // don't include fresh sync
    let contents_empty = config.file_contents.is_empty();
    if contents_empty
      && !config.files_on_host
      && config.repo.is_empty()
    {
      return false;
    }
    // The file contents MUST be empty
    contents_empty &&
    // The sync must be files on host mode OR git repo mode
    (config.files_on_host || !config.repo.is_empty())
  }

  fn include_resource_partial(
    name: &String,
    config: &Self::PartialConfig,
    match_resource_type: Option<ResourceTargetVariant>,
    match_resources: Option<&[String]>,
    resource_tags: &[String],
    id_to_tags: &HashMap<String, Tag>,
    match_tags: &[String],
  ) -> bool {
    if !include_resource_by_resource_type_and_name::<ResourceSync>(
      match_resource_type,
      match_resources,
      name,
    ) || !include_resource_by_tags(
      resource_tags,
      id_to_tags,
      match_tags,
    ) {
      return false;
    }
    // don't include fresh sync
    let contents_empty = config
      .file_contents
      .as_ref()
      .map(String::is_empty)
      .unwrap_or(true);
    let files_on_host = config.files_on_host.unwrap_or_default();
    if contents_empty
      && !files_on_host
      && config.repo.as_ref().map(String::is_empty).unwrap_or(true)
    {
      return false;
    }
    // The file contents MUST be empty
    contents_empty &&
    // The sync must be files on host mode OR git repo mode
    (files_on_host || !config.repo.as_deref().unwrap_or_default().is_empty())
  }

  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
    _resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    Ok(original.partial_diff(update))
  }
}

impl ExecuteResourceSync for ResourceSync {}

impl ResourceSyncTrait for Procedure {
  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Procedure(id)
  }

  fn get_diff(
    mut original: Self::Config,
    update: Self::PartialConfig,
    resources: &AllResourcesById,
  ) -> anyhow::Result<Self::ConfigDiff> {
    for stage in &mut original.stages {
      for execution in &mut stage.executions {
        match &mut execution.execution {
          Execution::None(_) => {}
          Execution::RunProcedure(config) => {
            config.procedure = resources
              .procedures
              .get(&config.procedure)
              .map(|p| p.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchRunProcedure(_config) => {}
          Execution::RunAction(config) => {
            config.action = resources
              .actions
              .get(&config.action)
              .map(|p| p.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchRunAction(_config) => {}
          Execution::RunBuild(config) => {
            config.build = resources
              .builds
              .get(&config.build)
              .map(|b| b.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchRunBuild(_config) => {}
          Execution::CancelBuild(config) => {
            config.build = resources
              .builds
              .get(&config.build)
              .map(|b| b.name.clone())
              .unwrap_or_default();
          }
          Execution::Deploy(config) => {
            config.deployment = resources
              .deployments
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchDeploy(_config) => {}
          Execution::PullDeployment(config) => {
            config.deployment = resources
              .deployments
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::StartDeployment(config) => {
            config.deployment = resources
              .deployments
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::RestartDeployment(config) => {
            config.deployment = resources
              .deployments
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PauseDeployment(config) => {
            config.deployment = resources
              .deployments
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::UnpauseDeployment(config) => {
            config.deployment = resources
              .deployments
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::StopDeployment(config) => {
            config.deployment = resources
              .deployments
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::DestroyDeployment(config) => {
            config.deployment = resources
              .deployments
              .get(&config.deployment)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchDestroyDeployment(_config) => {}
          Execution::CloneRepo(config) => {
            config.repo = resources
              .repos
              .get(&config.repo)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchCloneRepo(_config) => {}
          Execution::PullRepo(config) => {
            config.repo = resources
              .repos
              .get(&config.repo)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchPullRepo(_config) => {}
          Execution::BuildRepo(config) => {
            config.repo = resources
              .repos
              .get(&config.repo)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchBuildRepo(_config) => {}
          Execution::CancelRepoBuild(config) => {
            config.repo = resources
              .repos
              .get(&config.repo)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::StartContainer(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::RestartContainer(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PauseContainer(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::UnpauseContainer(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::StopContainer(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::DestroyContainer(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::StartAllContainers(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::RestartAllContainers(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PauseAllContainers(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::UnpauseAllContainers(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::StopAllContainers(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PruneContainers(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::DeleteNetwork(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PruneNetworks(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::DeleteImage(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PruneImages(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::DeleteVolume(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PruneVolumes(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PruneDockerBuilders(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PruneBuildx(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::PruneSystem(config) => {
            config.server = resources
              .servers
              .get(&config.server)
              .map(|d| d.name.clone())
              .unwrap_or_default();
          }
          Execution::RunSync(config) => {
            config.sync = resources
              .syncs
              .get(&config.sync)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::CommitSync(config) => {
            config.sync = resources
              .syncs
              .get(&config.sync)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::DeployStack(config) => {
            config.stack = resources
              .stacks
              .get(&config.stack)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchDeployStack(_config) => {}
          Execution::DeployStackIfChanged(config) => {
            config.stack = resources
              .stacks
              .get(&config.stack)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchDeployStackIfChanged(_config) => {}
          Execution::PullStack(config) => {
            config.stack = resources
              .stacks
              .get(&config.stack)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::StartStack(config) => {
            config.stack = resources
              .stacks
              .get(&config.stack)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::RestartStack(config) => {
            config.stack = resources
              .stacks
              .get(&config.stack)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::PauseStack(config) => {
            config.stack = resources
              .stacks
              .get(&config.stack)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::UnpauseStack(config) => {
            config.stack = resources
              .stacks
              .get(&config.stack)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::StopStack(config) => {
            config.stack = resources
              .stacks
              .get(&config.stack)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::DestroyStack(config) => {
            config.stack = resources
              .stacks
              .get(&config.stack)
              .map(|s| s.name.clone())
              .unwrap_or_default();
          }
          Execution::BatchDestroyStack(_config) => {}
          Execution::TestAlerter(config) => {
            config.alerter = resources
              .alerters
              .get(&config.alerter)
              .map(|a| a.name.clone())
              .unwrap_or_default();
          }
          Execution::Sleep(_) => {}
        }
      }
    }
    Ok(original.partial_diff(update))
  }
}

impl ExecuteResourceSync for Procedure {
  async fn execute_sync_updates(
    mut to_create: ToCreate<Self::PartialConfig>,
    mut to_update: ToUpdate<Self::PartialConfig>,
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

    for name in to_delete {
      if let Err(e) = crate::resource::delete::<Procedure>(
        &name,
        &WriteArgs {
          user: sync_user().to_owned(),
        },
      )
      .await
      {
        has_error = true;
        log.push_str(&format!(
          "\n{}: failed to delete {} '{}' | {e:#}",
          colored("ERROR", Color::Red),
          Self::resource_type(),
          bold(&name),
        ))
      } else {
        log.push_str(&format!(
          "\n{}: {} {} '{}'",
          muted("INFO"),
          colored("deleted", Color::Red),
          Self::resource_type(),
          bold(&name)
        ));
      }
    }

    if to_update.is_empty() && to_create.is_empty() {
      let stage = "Update Procedures";
      return Some(if has_error {
        Log::error(stage, log)
      } else {
        Log::simple(stage, log)
      });
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
            &mut log,
            &mut has_error,
          )
          .await;
        }
        if *update_tags {
          run_update_tags::<Procedure>(
            id.clone(),
            &name,
            tags,
            &mut log,
            &mut has_error,
          )
          .await;
        }
        if !resource.config.is_none() {
          if let Err(e) = crate::resource::update::<Procedure>(
            id,
            resource.config.clone(),
            sync_user(),
          )
          .await
          {
            if i == 9 {
              has_error = true;
              log.push_str(&format!(
                "\n{}: failed to update {} '{}' | {e:#}",
                colored("ERROR", Color::Red),
                Self::resource_type(),
                bold(&name)
              ));
            }
            continue;
          }
        }

        log.push_str(&format!(
          "\n{}: {} '{}' updated",
          muted("INFO"),
          Self::resource_type(),
          bold(&name)
        ));
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
        let id = match crate::resource::create::<Procedure>(
          &name,
          resource.config.clone(),
          sync_user(),
        )
        .await
        {
          Ok(resource) => resource.id,
          Err(e) => {
            if i == 9 {
              has_error = true;
              log.push_str(&format!(
                "\n{}: failed to create {} '{}' | {e:#}",
                colored("ERROR", Color::Red),
                Self::resource_type(),
                bold(&name)
              ));
            }
            continue;
          }
        };
        run_update_tags::<Procedure>(
          id.clone(),
          &name,
          tags,
          &mut log,
          &mut has_error,
        )
        .await;
        run_update_description::<Procedure>(
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
        to_pull.push(name);
      }
      to_create.retain(|resource| !to_pull.contains(&resource.name));

      if to_update.is_empty() && to_create.is_empty() {
        let stage = "Update Procedures";
        return Some(if has_error {
          Log::error(stage, log)
        } else {
          Log::simple(stage, log)
        });
      }
    }
    warn!("procedure sync loop exited after max iterations");

    Some(Log::error(
      "run procedure",
      String::from("procedure sync loop exited after max iterations"),
    ))
  }
}
