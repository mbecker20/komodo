use std::collections::HashMap;

use anyhow::Context;
use komodo_client::{
  api::execute::Execution,
  entities::{
    alerter::Alerter,
    build::Build,
    builder::{Builder, BuilderConfig},
    deployment::{Deployment, DeploymentImage},
    procedure::Procedure,
    repo::Repo,
    resource::Resource,
    server::Server,
    server_template::ServerTemplate,
    stack::Stack,
    sync::ResourceSync,
    tag::Tag,
    toml::ResourceToml,
  },
};
use partial_derive2::PartialDiff;

use crate::resource::KomodoResource;

use super::resource::AllResourcesById;

const OPTIONS: toml_pretty::Options = toml_pretty::Options {
  tab: "  ",
  skip_empty_string: true,
  max_inline_array_length: 30,
  inline_array: false,
};

pub trait ToToml: KomodoResource {
  /// Replace linked ids (server_id, build_id, etc) with the resource name.
  fn replace_ids(
    _resource: &mut Resource<Self::Config, Self::Info>,
    _all: &AllResourcesById,
  ) {
  }

  fn push_to_toml(
    resource: &ResourceToml<Self::PartialConfig>,
    toml: &mut String,
  ) -> anyhow::Result<()> {
    toml.push_str(
      &toml_pretty::to_string(&resource, OPTIONS)
        .context("failed to serialize resource to toml")?,
    );
    Ok(())
  }
}

pub fn resource_toml_to_toml<R: ToToml>(
  resource: &ResourceToml<R::PartialConfig>,
) -> anyhow::Result<String> {
  let mut toml = String::new();
  toml.push_str(&format!(
    "[[{}]]\n",
    R::resource_type().as_ref().to_lowercase()
  ));
  R::push_to_toml(resource, &mut toml)?;
  Ok(toml)
}

pub fn resource_push_to_toml<R: ToToml>(
  mut resource: Resource<R::Config, R::Info>,
  toml: &mut String,
  all: &AllResourcesById,
  all_tags: &HashMap<String, Tag>,
) -> anyhow::Result<()> {
  R::replace_ids(&mut resource, all);
  if !toml.is_empty() {
    toml.push_str("\n\n##\n\n");
  }
  toml.push_str(&format!(
    "[[{}]]\n",
    R::resource_type().as_ref().to_lowercase()
  ));
  R::push_to_toml(&convert_resource::<R>(resource, all_tags), toml)?;
  Ok(())
}

pub fn resource_to_toml<R: ToToml>(
  resource: Resource<R::Config, R::Info>,
  all: &AllResourcesById,
  all_tags: &HashMap<String, Tag>,
) -> anyhow::Result<String> {
  let mut toml = String::new();
  resource_push_to_toml::<R>(resource, &mut toml, all, all_tags)?;
  Ok(toml)
}

fn convert_resource<R: KomodoResource>(
  resource: Resource<R::Config, R::Info>,
  all_tags: &HashMap<String, Tag>,
) -> ResourceToml<R::PartialConfig> {
  // This makes sure all non-necessary (defaulted) fields don't make it into final toml
  let partial: R::PartialConfig = resource.config.into();
  let config = R::Config::default().minimize_partial(partial);
  ResourceToml {
    name: resource.name,
    tags: resource
      .tags
      .iter()
      .filter_map(|t| all_tags.get(t).map(|t| t.name.clone()))
      .collect(),
    description: resource.description,
    deploy: false,
    after: Default::default(),
    latest_hash: false,
    config,
  }
}

// These have no linked resource ids to replace
impl ToToml for Alerter {}
impl ToToml for Server {}
impl ToToml for ServerTemplate {}
impl ToToml for ResourceSync {}

impl ToToml for Stack {
  fn replace_ids(
    resource: &mut Resource<Self::Config, Self::Info>,
    all: &AllResourcesById,
  ) {
    resource.config.server_id.clone_from(
      all
        .servers
        .get(&resource.config.server_id)
        .map(|s| &s.name)
        .unwrap_or(&String::new()),
    );
  }
}

impl ToToml for Deployment {
  fn replace_ids(
    resource: &mut Resource<Self::Config, Self::Info>,
    all: &AllResourcesById,
  ) {
    resource.config.server_id.clone_from(
      all
        .servers
        .get(&resource.config.server_id)
        .map(|s| &s.name)
        .unwrap_or(&String::new()),
    );
    if let DeploymentImage::Build { build_id, .. } =
      &mut resource.config.image
    {
      build_id.clone_from(
        all
          .builds
          .get(build_id)
          .map(|b| &b.name)
          .unwrap_or(&String::new()),
      );
    }
  }
}

impl ToToml for Build {
  fn replace_ids(
    resource: &mut Resource<Self::Config, Self::Info>,
    all: &AllResourcesById,
  ) {
    resource.config.builder_id.clone_from(
      all
        .builders
        .get(&resource.config.builder_id)
        .map(|s| &s.name)
        .unwrap_or(&String::new()),
    );
  }
}

impl ToToml for Repo {
  fn replace_ids(
    resource: &mut Resource<Self::Config, Self::Info>,
    all: &AllResourcesById,
  ) {
    resource.config.server_id.clone_from(
      all
        .servers
        .get(&resource.config.server_id)
        .map(|s| &s.name)
        .unwrap_or(&String::new()),
    );
    resource.config.builder_id.clone_from(
      all
        .builders
        .get(&resource.config.builder_id)
        .map(|s| &s.name)
        .unwrap_or(&String::new()),
    );
  }
}

impl ToToml for Builder {
  fn replace_ids(
    resource: &mut Resource<Self::Config, Self::Info>,
    all: &AllResourcesById,
  ) {
    if let BuilderConfig::Server(config) = &mut resource.config {
      config.server_id.clone_from(
        all
          .servers
          .get(&config.server_id)
          .map(|s| &s.name)
          .unwrap_or(&String::new()),
      )
    }
  }
}

impl ToToml for Procedure {
  fn replace_ids(
    resource: &mut Resource<Self::Config, Self::Info>,
    all: &AllResourcesById,
  ) {
    for stage in &mut resource.config.stages {
      for execution in &mut stage.executions {
        match &mut execution.execution {
          Execution::RunProcedure(exec) => exec.procedure.clone_from(
            all
              .procedures
              .get(&exec.procedure)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::RunBuild(exec) => exec.build.clone_from(
            all
              .builds
              .get(&exec.build)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::CancelBuild(exec) => exec.build.clone_from(
            all
              .builds
              .get(&exec.build)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::Deploy(exec) => exec.deployment.clone_from(
            all
              .deployments
              .get(&exec.deployment)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::StartDeployment(exec) => {
            exec.deployment.clone_from(
              all
                .deployments
                .get(&exec.deployment)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::RestartDeployment(exec) => {
            exec.deployment.clone_from(
              all
                .deployments
                .get(&exec.deployment)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::PauseDeployment(exec) => {
            exec.deployment.clone_from(
              all
                .deployments
                .get(&exec.deployment)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::UnpauseDeployment(exec) => {
            exec.deployment.clone_from(
              all
                .deployments
                .get(&exec.deployment)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::StopDeployment(exec) => {
            exec.deployment.clone_from(
              all
                .deployments
                .get(&exec.deployment)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::DestroyDeployment(exec) => {
            exec.deployment.clone_from(
              all
                .deployments
                .get(&exec.deployment)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::CloneRepo(exec) => exec.repo.clone_from(
            all
              .repos
              .get(&exec.repo)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::PullRepo(exec) => exec.repo.clone_from(
            all
              .repos
              .get(&exec.repo)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::BuildRepo(exec) => exec.repo.clone_from(
            all
              .repos
              .get(&exec.repo)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::CancelRepoBuild(exec) => exec.repo.clone_from(
            all
              .repos
              .get(&exec.repo)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::StartContainer(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::RestartContainer(exec) => {
            exec.server.clone_from(
              all
                .servers
                .get(&exec.server)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::PauseContainer(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::UnpauseContainer(exec) => {
            exec.server.clone_from(
              all
                .servers
                .get(&exec.server)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::StopContainer(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::DestroyContainer(exec) => {
            exec.server.clone_from(
              all
                .servers
                .get(&exec.server)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::StartAllContainers(exec) => {
            exec.server.clone_from(
              all
                .servers
                .get(&exec.server)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::RestartAllContainers(exec) => {
            exec.server.clone_from(
              all
                .servers
                .get(&exec.server)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::PauseAllContainers(exec) => {
            exec.server.clone_from(
              all
                .servers
                .get(&exec.server)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::UnpauseAllContainers(exec) => {
            exec.server.clone_from(
              all
                .servers
                .get(&exec.server)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::StopAllContainers(exec) => {
            exec.server.clone_from(
              all
                .servers
                .get(&exec.server)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::PruneContainers(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::DeleteNetwork(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::PruneNetworks(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::DeleteImage(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::PruneImages(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::DeleteVolume(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::PruneVolumes(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::PruneDockerBuilders(exec) => {
            exec.server.clone_from(
              all
                .servers
                .get(&exec.server)
                .map(|r| &r.name)
                .unwrap_or(&String::new()),
            )
          }
          Execution::PruneBuildx(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::PruneSystem(exec) => exec.server.clone_from(
            all
              .servers
              .get(&exec.server)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::RunSync(exec) => exec.sync.clone_from(
            all
              .syncs
              .get(&exec.sync)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::DeployStack(exec) => exec.stack.clone_from(
            all
              .stacks
              .get(&exec.stack)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::StartStack(exec) => exec.stack.clone_from(
            all
              .stacks
              .get(&exec.stack)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::RestartStack(exec) => exec.stack.clone_from(
            all
              .stacks
              .get(&exec.stack)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::PauseStack(exec) => exec.stack.clone_from(
            all
              .stacks
              .get(&exec.stack)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::UnpauseStack(exec) => exec.stack.clone_from(
            all
              .stacks
              .get(&exec.stack)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::StopStack(exec) => exec.stack.clone_from(
            all
              .stacks
              .get(&exec.stack)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::DestroyStack(exec) => exec.stack.clone_from(
            all
              .stacks
              .get(&exec.stack)
              .map(|r| &r.name)
              .unwrap_or(&String::new()),
          ),
          Execution::Sleep(_) | Execution::None(_) => {}
        }
      }
    }
  }
}
