use std::collections::HashMap;

use anyhow::Context;
use monitor_client::{
  api::{
    execute::Execution,
    read::{
      ExportAllResourcesToToml, ExportAllResourcesToTomlResponse,
      ExportResourcesToToml, ExportResourcesToTomlResponse,
      GetUserGroup, ListUserTargetPermissions,
    },
  },
  entities::{
    alerter::Alerter,
    build::Build,
    builder::{Builder, BuilderConfig},
    deployment::{
      conversions_to_string, term_signal_labels_to_string,
      Deployment, DeploymentImage,
    },
    environment_vars_to_string,
    permission::{PermissionLevel, UserTarget},
    procedure::Procedure,
    repo::Repo,
    resource::{Resource, ResourceQuery},
    server::Server,
    server_template::ServerTemplate,
    stack::Stack,
    sync::ResourceSync,
    toml::{
      PermissionToml, ResourceToml, ResourcesToml, UserGroupToml,
    },
    ResourceTarget,
    user::User,
  },
};
use mungos::find::find_collect;
use ordered_hash_map::OrderedHashMap;
use partial_derive2::PartialDiff;
use resolver_api::Resolve;
use serde_json::Value;

use crate::{
  helpers::query::get_user_user_group_ids,
  resource::{self, MonitorResource},
  state::{db_client, State},
};

impl Resolve<ExportAllResourcesToToml, User> for State {
  async fn resolve(
    &self,
    ExportAllResourcesToToml { tags }: ExportAllResourcesToToml,
    user: User,
  ) -> anyhow::Result<ExportAllResourcesToTomlResponse> {
    let mut targets = Vec::<ResourceTarget>::new();

    targets.extend(
      resource::list_for_user::<Alerter>(
        ResourceQuery::builder().tags(tags.clone()).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Alerter(resource.id)),
    );
    targets.extend(
      resource::list_for_user::<Builder>(
        ResourceQuery::builder().tags(tags.clone()).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Builder(resource.id)),
    );
    targets.extend(
      resource::list_for_user::<Server>(
        ResourceQuery::builder().tags(tags.clone()).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Server(resource.id)),
    );
    targets.extend(
      resource::list_for_user::<Deployment>(
        ResourceQuery::builder().tags(tags.clone()).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Deployment(resource.id)),
    );
    targets.extend(
      resource::list_for_user::<Stack>(
        ResourceQuery::builder().tags(tags.clone()).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Stack(resource.id)),
    );
    targets.extend(
      resource::list_for_user::<Build>(
        ResourceQuery::builder().tags(tags.clone()).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Build(resource.id)),
    );
    targets.extend(
      resource::list_for_user::<Repo>(
        ResourceQuery::builder().tags(tags.clone()).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Repo(resource.id)),
    );
    targets.extend(
      resource::list_for_user::<Procedure>(
        ResourceQuery::builder().tags(tags.clone()).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Procedure(resource.id)),
    );
    targets.extend(
      resource::list_for_user::<ServerTemplate>(
        ResourceQuery::builder().tags(tags.clone()).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::ServerTemplate(resource.id)),
    );
    targets.extend(
      resource::list_for_user::<ResourceSync>(
        ResourceQuery::builder().tags(tags).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::ResourceSync(resource.id)),
    );

    let user_groups = if user.admin {
      find_collect(&db_client().await.user_groups, None, None)
        .await
        .context("failed to query db for user groups")?
        .into_iter()
        .map(|user_group| user_group.id)
        .collect()
    } else {
      get_user_user_group_ids(&user.id).await?
    };

    self
      .resolve(
        ExportResourcesToToml {
          targets,
          user_groups,
          include_variables: true,
        },
        user,
      )
      .await
  }
}

impl Resolve<ExportResourcesToToml, User> for State {
  async fn resolve(
    &self,
    ExportResourcesToToml {
      targets,
      user_groups,
      include_variables,
    }: ExportResourcesToToml,
    user: User,
  ) -> anyhow::Result<ExportResourcesToTomlResponse> {
    let mut res = ResourcesToml::default();
    let names = ResourceNames::new()
      .await
      .context("failed to init resource name maps")?;
    for target in targets {
      match target {
        ResourceTarget::Alerter(id) => {
          let alerter = resource::get_check_permissions::<Alerter>(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res
            .alerters
            .push(convert_resource::<Alerter>(alerter, &names.tags))
        }
        ResourceTarget::ResourceSync(id) => {
          let sync = resource::get_check_permissions::<ResourceSync>(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res
            .resource_syncs
            .push(convert_resource::<ResourceSync>(sync, &names.tags))
        }
        ResourceTarget::ServerTemplate(id) => {
          let template = resource::get_check_permissions::<
            ServerTemplate,
          >(
            &id, &user, PermissionLevel::Read
          )
          .await?;
          res.server_templates.push(
            convert_resource::<ServerTemplate>(template, &names.tags),
          )
        }
        ResourceTarget::Server(id) => {
          let server = resource::get_check_permissions::<Server>(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res
            .servers
            .push(convert_resource::<Server>(server, &names.tags))
        }
        ResourceTarget::Builder(id) => {
          let mut builder =
            resource::get_check_permissions::<Builder>(
              &id,
              &user,
              PermissionLevel::Read,
            )
            .await?;
          // replace server id of builder
          if let BuilderConfig::Server(config) = &mut builder.config {
            config.server_id.clone_from(
              names
                .servers
                .get(&config.server_id)
                .unwrap_or(&String::new()),
            )
          }
          res
            .builders
            .push(convert_resource::<Builder>(builder, &names.tags))
        }
        ResourceTarget::Build(id) => {
          let mut build = resource::get_check_permissions::<Build>(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          // replace builder id of build
          build.config.builder_id.clone_from(
            names
              .builders
              .get(&build.config.builder_id)
              .unwrap_or(&String::new()),
          );
          res
            .builds
            .push(convert_resource::<Build>(build, &names.tags))
        }
        ResourceTarget::Deployment(id) => {
          let mut deployment = resource::get_check_permissions::<
            Deployment,
          >(
            &id, &user, PermissionLevel::Read
          )
          .await?;
          // replace deployment server with name
          deployment.config.server_id.clone_from(
            names
              .servers
              .get(&deployment.config.server_id)
              .unwrap_or(&String::new()),
          );
          // replace deployment build id with name
          if let DeploymentImage::Build { build_id, .. } =
            &mut deployment.config.image
          {
            build_id.clone_from(
              names.builds.get(build_id).unwrap_or(&String::new()),
            );
          }
          res.deployments.push(convert_resource::<Deployment>(
            deployment,
            &names.tags,
          ))
        }
        ResourceTarget::Repo(id) => {
          let mut repo = resource::get_check_permissions::<Repo>(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          // replace repo server with name
          repo.config.server_id.clone_from(
            names
              .servers
              .get(&repo.config.server_id)
              .unwrap_or(&String::new()),
          );
          // replace repo builder with name
          repo.config.builder_id.clone_from(
            names
              .builders
              .get(&repo.config.builder_id)
              .unwrap_or(&String::new()),
          );
          res.repos.push(convert_resource::<Repo>(repo, &names.tags))
        }
        ResourceTarget::Stack(id) => {
          let mut stack = resource::get_check_permissions::<Stack>(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          // replace stack server with name
          stack.config.server_id.clone_from(
            names
              .servers
              .get(&stack.config.server_id)
              .unwrap_or(&String::new()),
          );
          res
            .stacks
            .push(convert_resource::<Stack>(stack, &names.tags))
        }
        ResourceTarget::Procedure(id) => {
          add_procedure(&id, &mut res, &user, &names)
            .await
            .with_context(|| {
              format!("failed to add procedure {id}")
            })?;
        }
        ResourceTarget::System(_) => continue,
      };
    }

    add_user_groups(user_groups, &mut res, &names, &user)
      .await
      .context("failed to add user groups")?;

    if include_variables {
      res.variables =
        find_collect(&db_client().await.variables, None, None)
          .await
          .context("failed to get variables from db")?;
    }

    let toml = serialize_resources_toml(&res)
      .context("failed to serialize resources to toml")?;

    Ok(ExportResourcesToTomlResponse { toml })
  }
}

async fn add_procedure(
  id: &str,
  res: &mut ResourcesToml,
  user: &User,
  names: &ResourceNames,
) -> anyhow::Result<()> {
  let mut procedure = resource::get_check_permissions::<Procedure>(
    id,
    user,
    PermissionLevel::Read,
  )
  .await?;

  for stage in &mut procedure.config.stages {
    for execution in &mut stage.executions {
      match &mut execution.execution {
        Execution::RunProcedure(exec) => exec.procedure.clone_from(
          names
            .procedures
            .get(&exec.procedure)
            .unwrap_or(&String::new()),
        ),
        Execution::RunBuild(exec) => exec.build.clone_from(
          names.builds.get(&exec.build).unwrap_or(&String::new()),
        ),
        Execution::CancelBuild(exec) => exec.build.clone_from(
          names.builds.get(&exec.build).unwrap_or(&String::new()),
        ),
        Execution::Deploy(exec) => exec.deployment.clone_from(
          names
            .deployments
            .get(&exec.deployment)
            .unwrap_or(&String::new()),
        ),
        Execution::StartDeployment(exec) => {
          exec.deployment.clone_from(
            names
              .deployments
              .get(&exec.deployment)
              .unwrap_or(&String::new()),
          )
        }
        Execution::RestartDeployment(exec) => {
          exec.deployment.clone_from(
            names
              .deployments
              .get(&exec.deployment)
              .unwrap_or(&String::new()),
          )
        }
        Execution::PauseDeployment(exec) => {
          exec.deployment.clone_from(
            names
              .deployments
              .get(&exec.deployment)
              .unwrap_or(&String::new()),
          )
        }
        Execution::UnpauseDeployment(exec) => {
          exec.deployment.clone_from(
            names
              .deployments
              .get(&exec.deployment)
              .unwrap_or(&String::new()),
          )
        }
        Execution::StopDeployment(exec) => {
          exec.deployment.clone_from(
            names
              .deployments
              .get(&exec.deployment)
              .unwrap_or(&String::new()),
          )
        }
        Execution::DestroyDeployment(exec) => {
          exec.deployment.clone_from(
            names
              .deployments
              .get(&exec.deployment)
              .unwrap_or(&String::new()),
          )
        }
        Execution::CloneRepo(exec) => exec.repo.clone_from(
          names.repos.get(&exec.repo).unwrap_or(&String::new()),
        ),
        Execution::PullRepo(exec) => exec.repo.clone_from(
          names.repos.get(&exec.repo).unwrap_or(&String::new()),
        ),
        Execution::BuildRepo(exec) => exec.repo.clone_from(
          names.repos.get(&exec.repo).unwrap_or(&String::new()),
        ),
        Execution::CancelRepoBuild(exec) => exec.repo.clone_from(
          names.repos.get(&exec.repo).unwrap_or(&String::new()),
        ),
        Execution::StartContainer(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::RestartContainer(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::PauseContainer(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::UnpauseContainer(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::StopContainer(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::DestroyContainer(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::StartAllContainers(exec) => {
          exec.server.clone_from(
            names.servers.get(&exec.server).unwrap_or(&String::new()),
          )
        }
        Execution::RestartAllContainers(exec) => {
          exec.server.clone_from(
            names.servers.get(&exec.server).unwrap_or(&String::new()),
          )
        }
        Execution::PauseAllContainers(exec) => {
          exec.server.clone_from(
            names.servers.get(&exec.server).unwrap_or(&String::new()),
          )
        }
        Execution::UnpauseAllContainers(exec) => {
          exec.server.clone_from(
            names.servers.get(&exec.server).unwrap_or(&String::new()),
          )
        }
        Execution::StopAllContainers(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::PruneContainers(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::PruneNetworks(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::PruneImages(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::PruneVolumes(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::PruneSystem(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::RunSync(exec) => exec.sync.clone_from(
          names.syncs.get(&exec.sync).unwrap_or(&String::new()),
        ),
        Execution::DeployStack(exec) => exec.stack.clone_from(
          names.stacks.get(&exec.stack).unwrap_or(&String::new()),
        ),
        Execution::StartStack(exec) => exec.stack.clone_from(
          names.stacks.get(&exec.stack).unwrap_or(&String::new()),
        ),
        Execution::RestartStack(exec) => exec.stack.clone_from(
          names.stacks.get(&exec.stack).unwrap_or(&String::new()),
        ),
        Execution::PauseStack(exec) => exec.stack.clone_from(
          names.stacks.get(&exec.stack).unwrap_or(&String::new()),
        ),
        Execution::UnpauseStack(exec) => exec.stack.clone_from(
          names.stacks.get(&exec.stack).unwrap_or(&String::new()),
        ),
        Execution::StopStack(exec) => exec.stack.clone_from(
          names.stacks.get(&exec.stack).unwrap_or(&String::new()),
        ),
        Execution::DestroyStack(exec) => exec.stack.clone_from(
          names.stacks.get(&exec.stack).unwrap_or(&String::new()),
        ),
        Execution::Sleep(_) | Execution::None(_) => {}
      }
    }
  }

  res
    .procedures
    .push(convert_resource::<Procedure>(procedure, &names.tags));
  Ok(())
}

struct ResourceNames {
  tags: HashMap<String, String>,
  servers: HashMap<String, String>,
  builders: HashMap<String, String>,
  builds: HashMap<String, String>,
  repos: HashMap<String, String>,
  deployments: HashMap<String, String>,
  procedures: HashMap<String, String>,
  syncs: HashMap<String, String>,
  stacks: HashMap<String, String>,
  alerters: HashMap<String, String>,
  templates: HashMap<String, String>,
}

impl ResourceNames {
  async fn new() -> anyhow::Result<ResourceNames> {
    let db = db_client().await;
    Ok(ResourceNames {
      tags: find_collect(&db.tags, None, None)
        .await
        .context("failed to get all tags")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
      servers: find_collect(&db.servers, None, None)
        .await
        .context("failed to get all servers")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
      builders: find_collect(&db.builders, None, None)
        .await
        .context("failed to get all builders")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
      builds: find_collect(&db.builds, None, None)
        .await
        .context("failed to get all builds")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
      repos: find_collect(&db.repos, None, None)
        .await
        .context("failed to get all repos")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
      deployments: find_collect(&db.deployments, None, None)
        .await
        .context("failed to get all deployments")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
      procedures: find_collect(&db.procedures, None, None)
        .await
        .context("failed to get all procedures")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
      syncs: find_collect(&db.resource_syncs, None, None)
        .await
        .context("failed to get all resource syncs")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
      stacks: find_collect(&db.stacks, None, None)
        .await
        .context("failed to get all stacks")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
      alerters: find_collect(&db.alerters, None, None)
        .await
        .context("failed to get all alerters")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
      templates: find_collect(&db.server_templates, None, None)
        .await
        .context("failed to get all server templates")?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect::<HashMap<_, _>>(),
    })
  }
}

async fn add_user_groups(
  user_groups: Vec<String>,
  res: &mut ResourcesToml,
  names: &ResourceNames,
  user: &User,
) -> anyhow::Result<()> {
  let db = db_client().await;

  let usernames = find_collect(&db.users, None, None)
    .await?
    .into_iter()
    .map(|user| (user.id, user.username))
    .collect::<HashMap<_, _>>();

  for user_group in user_groups {
    let ug = State
      .resolve(GetUserGroup { user_group }, user.clone())
      .await?;
    // this method is admin only, but we already know user can see user group if above does not return Err
    let permissions = State
      .resolve(
        ListUserTargetPermissions {
          user_target: UserTarget::UserGroup(ug.id),
        },
        User {
          admin: true,
          ..Default::default()
        },
      )
      .await?
      .into_iter()
      .map(|mut permission| {
        match &mut permission.resource_target {
          ResourceTarget::Build(id) => {
            *id = names.builds.get(id).cloned().unwrap_or_default()
          }
          ResourceTarget::Builder(id) => {
            *id = names.builders.get(id).cloned().unwrap_or_default()
          }
          ResourceTarget::Deployment(id) => {
            *id =
              names.deployments.get(id).cloned().unwrap_or_default()
          }
          ResourceTarget::Server(id) => {
            *id = names.servers.get(id).cloned().unwrap_or_default()
          }
          ResourceTarget::Repo(id) => {
            *id = names.repos.get(id).cloned().unwrap_or_default()
          }
          ResourceTarget::Alerter(id) => {
            *id = names.alerters.get(id).cloned().unwrap_or_default()
          }
          ResourceTarget::Procedure(id) => {
            *id =
              names.procedures.get(id).cloned().unwrap_or_default()
          }
          ResourceTarget::ServerTemplate(id) => {
            *id = names.templates.get(id).cloned().unwrap_or_default()
          }
          ResourceTarget::ResourceSync(id) => {
            *id = names.syncs.get(id).cloned().unwrap_or_default()
          }
          ResourceTarget::Stack(id) => {
            *id = names.stacks.get(id).cloned().unwrap_or_default()
          }
          ResourceTarget::System(_) => {}
        }
        PermissionToml {
          target: permission.resource_target,
          level: permission.level,
        }
      })
      .collect();
    res.user_groups.push(UserGroupToml {
      name: ug.name,
      users: ug
        .users
        .into_iter()
        .filter_map(|user_id| usernames.get(&user_id).cloned())
        .collect(),
      all: ug.all,
      permissions,
    });
  }
  Ok(())
}

fn convert_resource<R: MonitorResource>(
  resource: Resource<R::Config, R::Info>,
  tag_names: &HashMap<String, String>,
) -> ResourceToml<R::PartialConfig> {
  // This makes sure all non-necessary (defaulted) fields don't make it into final toml
  let partial: R::PartialConfig = resource.config.into();
  let config = R::Config::default().minimize_partial(partial);
  ResourceToml {
    name: resource.name,
    tags: resource
      .tags
      .iter()
      .filter_map(|t| tag_names.get(t).cloned())
      .collect(),
    description: resource.description,
    deploy: false,
    after: Default::default(),
    latest_hash: false,
    config,
  }
}

fn serialize_resources_toml(
  resources: &ResourcesToml,
) -> anyhow::Result<String> {
  let mut res = String::new();

  let options = toml_pretty::Options::default()
    .tab("  ")
    .skip_empty_string(true)
    .max_inline_array_length(30);

  for server in &resources.servers {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    res.push_str("[[server]]\n");
    res.push_str(
      &toml_pretty::to_string(&server, options)
        .context("failed to serialize servers to toml")?,
    );
  }

  for deployment in &resources.deployments {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    res.push_str("[[deployment]]\n");
    let mut parsed: OrderedHashMap<String, Value> =
      serde_json::from_str(&serde_json::to_string(&deployment)?)?;
    let config = parsed
      .get_mut("config")
      .context("deployment has no config?")?
      .as_object_mut()
      .context("config is not object?")?;
    if let Some(DeploymentImage::Build { version, .. }) =
      &deployment.config.image
    {
      let image = config
        .get_mut("image")
        .context("deployment has no image")?
        .get_mut("params")
        .context("deployment image has no params")?
        .as_object_mut()
        .context("deployment image params is not object")?;
      if version.is_none() {
        image.remove("version");
      } else {
        image.insert(
          "version".to_string(),
          Value::String(version.to_string()),
        );
      }
    }
    if let Some(term_signal_labels) =
      &deployment.config.term_signal_labels
    {
      config.insert(
        "term_signal_labels".to_string(),
        Value::String(term_signal_labels_to_string(
          term_signal_labels,
        )),
      );
    }
    if let Some(ports) = &deployment.config.ports {
      config.insert(
        "ports".to_string(),
        Value::String(conversions_to_string(ports)),
      );
    }
    if let Some(volumes) = &deployment.config.volumes {
      config.insert(
        "volumes".to_string(),
        Value::String(conversions_to_string(volumes)),
      );
    }
    if let Some(environment) = &deployment.config.environment {
      config.insert(
        "environment".to_string(),
        Value::String(environment_vars_to_string(environment)),
      );
    }
    if let Some(labels) = &deployment.config.labels {
      config.insert(
        "labels".to_string(),
        Value::String(environment_vars_to_string(labels)),
      );
    }
    res.push_str(
      &toml_pretty::to_string(&parsed, options)
        .context("failed to serialize deployments to toml")?,
    );
  }

  for stack in &resources.stacks {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    res.push_str("[[stack]]\n");
    let mut parsed: OrderedHashMap<String, Value> =
      serde_json::from_str(&serde_json::to_string(&stack)?)?;
    let config = parsed
      .get_mut("config")
      .context("stack has no config?")?
      .as_object_mut()
      .context("config is not object?")?;
    if let Some(environment) = &stack.config.environment {
      config.insert(
        "environment".to_string(),
        Value::String(environment_vars_to_string(environment)),
      );
    }
    res.push_str(
      &toml_pretty::to_string(&parsed, options)
        .context("failed to serialize stacks to toml")?,
    );
  }

  for build in &resources.builds {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    let mut parsed: OrderedHashMap<String, Value> =
      serde_json::from_str(&serde_json::to_string(&build)?)?;
    let config = parsed
      .get_mut("config")
      .context("build has no config?")?
      .as_object_mut()
      .context("config is not object?")?;
    if let Some(version) = &build.config.version {
      config.insert(
        "version".to_string(),
        Value::String(version.to_string()),
      );
    }
    if let Some(build_args) = &build.config.build_args {
      config.insert(
        "build_args".to_string(),
        Value::String(environment_vars_to_string(build_args)),
      );
    }
    if let Some(labels) = &build.config.labels {
      config.insert(
        "labels".to_string(),
        Value::String(environment_vars_to_string(labels)),
      );
    }
    res.push_str("[[build]]\n");
    res.push_str(
      &toml_pretty::to_string(&parsed, options)
        .context("failed to serialize builds to toml")?,
    );
  }

  for repo in &resources.repos {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    res.push_str("[[repo]]\n");
    res.push_str(
      &toml_pretty::to_string(&repo, options)
        .context("failed to serialize repos to toml")?,
    );
  }

  for procedure in &resources.procedures {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    let mut parsed: OrderedHashMap<String, Value> =
      serde_json::from_str(&serde_json::to_string(&procedure)?)?;
    let config = parsed
      .get_mut("config")
      .context("procedure has no config?")?
      .as_object_mut()
      .context("config is not object?")?;

    let stages = config
      .remove("stages")
      .context("procedure config has no stages")?;
    let stages = stages.as_array().context("stages is not array")?;

    res.push_str("[[procedure]]\n");
    res.push_str(
      &toml_pretty::to_string(&parsed, options)
        .context("failed to serialize procedures to toml")?,
    );

    for stage in stages {
      res.push_str("\n\n[[procedure.config.stage]]\n");
      res.push_str(
        &toml_pretty::to_string(stage, options)
          .context("failed to serialize procedures to toml")?,
      );
    }
  }

  for alerter in &resources.alerters {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    res.push_str("[[alerter]]\n");
    res.push_str(
      &toml_pretty::to_string(&alerter, options)
        .context("failed to serialize alerters to toml")?,
    );
  }

  for builder in &resources.builders {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    res.push_str("[[builder]]\n");
    res.push_str(
      &toml_pretty::to_string(&builder, options)
        .context("failed to serialize builders to toml")?,
    );
  }

  for server_template in &resources.server_templates {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    res.push_str("[[server_template]]\n");
    res.push_str(
      &toml_pretty::to_string(&server_template, options)
        .context("failed to serialize server_templates to toml")?,
    );
  }

  for resource_sync in &resources.resource_syncs {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    res.push_str("[[resource_sync]]\n");
    res.push_str(
      &toml_pretty::to_string(&resource_sync, options)
        .context("failed to serialize resource_syncs to toml")?,
    );
  }

  for variable in &resources.variables {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    res.push_str("[[variable]]\n");
    res.push_str(
      &toml_pretty::to_string(&variable, options)
        .context("failed to serialize variables to toml")?,
    );
  }

  for user_group in &resources.user_groups {
    if !res.is_empty() {
      res.push_str("\n\n##\n\n");
    }
    res.push_str("[[user_group]]\n");
    res.push_str(
      &toml_pretty::to_string(&user_group, options)
        .context("failed to serialize user_groups to toml")?,
    );
  }

  Ok(res)
}
