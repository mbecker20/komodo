use std::collections::HashMap;

use anyhow::Context;
use axum::async_trait;
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
    deployment::{Deployment, DeploymentImage},
    permission::{PermissionLevel, UserTarget},
    procedure::Procedure,
    repo::Repo,
    resource::Resource,
    server::Server,
    server_template::ServerTemplate,
    toml::{
      PermissionToml, ResourceToml, ResourcesToml, UserGroupToml,
    },
    update::ResourceTarget,
    user::User,
  },
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::{
  helpers::{
    query::get_user_user_group_ids, resource::StateResource,
  },
  state::{db_client, State},
};

#[async_trait]
impl Resolve<ExportAllResourcesToToml, User> for State {
  async fn resolve(
    &self,
    ExportAllResourcesToToml {}: ExportAllResourcesToToml,
    user: User,
  ) -> anyhow::Result<ExportAllResourcesToTomlResponse> {
    let mut targets = Vec::<ResourceTarget>::new();

    targets.extend(
      Alerter::list_resource_list_items_for_user(
        Default::default(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Alerter(resource.id)),
    );
    targets.extend(
      Builder::list_resource_list_items_for_user(
        Default::default(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Builder(resource.id)),
    );
    targets.extend(
      Server::list_resource_list_items_for_user(
        Default::default(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Server(resource.id)),
    );
    targets.extend(
      Deployment::list_resource_list_items_for_user(
        Default::default(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Deployment(resource.id)),
    );
    targets.extend(
      Build::list_resource_list_items_for_user(
        Default::default(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Build(resource.id)),
    );
    targets.extend(
      Repo::list_resource_list_items_for_user(
        Default::default(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Repo(resource.id)),
    );
    targets.extend(
      Procedure::list_resource_list_items_for_user(
        Default::default(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::Procedure(resource.id)),
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
        },
        user,
      )
      .await
  }
}

#[async_trait]
impl Resolve<ExportResourcesToToml, User> for State {
  async fn resolve(
    &self,
    ExportResourcesToToml {
      targets,
      user_groups,
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
          let alerter = Alerter::get_resource_check_permissions(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res.alerters.push(convert_resource(alerter, &names.tags))
        }
        ResourceTarget::ServerTemplate(id) => {
          let template =
            ServerTemplate::get_resource_check_permissions(
              &id,
              &user,
              PermissionLevel::Read,
            )
            .await?;
          res
            .server_templates
            .push(convert_resource(template, &names.tags))
        }
        ResourceTarget::Server(id) => {
          let server = Server::get_resource_check_permissions(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res.servers.push(convert_resource(server, &names.tags))
        }
        ResourceTarget::Builder(id) => {
          let mut builder = Builder::get_resource_check_permissions(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          // replace server id of builder
          if let BuilderConfig::Server(config) = &mut builder.config {
            config.server_id.clone_from(
              names.servers.get(&id).unwrap_or(&String::new()),
            )
          }
          res.builders.push(convert_resource(builder, &names.tags))
        }
        ResourceTarget::Build(id) => {
          let mut build = Build::get_resource_check_permissions(
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
          res.builds.push(convert_resource(build, &names.tags))
        }
        ResourceTarget::Deployment(id) => {
          let mut deployment =
            Deployment::get_resource_check_permissions(
              &id,
              &user,
              PermissionLevel::Read,
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
          res
            .deployments
            .push(convert_resource(deployment, &names.tags))
        }
        ResourceTarget::Repo(id) => {
          let mut repo = Repo::get_resource_check_permissions(
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
          res.repos.push(convert_resource(repo, &names.tags))
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

    add_user_groups(user_groups, &mut res, &user)
      .await
      .context("failed to add user groups")?;

    let toml = toml::to_string_pretty(&res)
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
  let mut procedure = Procedure::get_resource_check_permissions(
    id,
    user,
    PermissionLevel::Read,
  )
  .await?;
  for execution in &mut procedure.config.executions {
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
      Execution::Deploy(exec) => exec.deployment.clone_from(
        names
          .deployments
          .get(&exec.deployment)
          .unwrap_or(&String::new()),
      ),
      Execution::StartContainer(exec) => exec.deployment.clone_from(
        names
          .deployments
          .get(&exec.deployment)
          .unwrap_or(&String::new()),
      ),
      Execution::StopContainer(exec) => exec.deployment.clone_from(
        names
          .deployments
          .get(&exec.deployment)
          .unwrap_or(&String::new()),
      ),
      Execution::RemoveContainer(exec) => exec.deployment.clone_from(
        names
          .deployments
          .get(&exec.deployment)
          .unwrap_or(&String::new()),
      ),
      Execution::CloneRepo(exec) => exec.repo.clone_from(
        names.repos.get(&exec.repo).unwrap_or(&String::new()),
      ),
      Execution::PullRepo(exec) => exec.repo.clone_from(
        names.repos.get(&exec.repo).unwrap_or(&String::new()),
      ),
      Execution::StopAllContainers(exec) => exec.server.clone_from(
        names.servers.get(&exec.server).unwrap_or(&String::new()),
      ),
      Execution::PruneDockerNetworks(exec) => exec.server.clone_from(
        names.servers.get(&exec.server).unwrap_or(&String::new()),
      ),
      Execution::PruneDockerImages(exec) => exec.server.clone_from(
        names.servers.get(&exec.server).unwrap_or(&String::new()),
      ),
      Execution::PruneDockerContainers(exec) => {
        exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        )
      }
      Execution::None(_) => continue,
    }
  }
  res
    .procedures
    .push(convert_resource(procedure, &names.tags));
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
    })
  }
}

async fn add_user_groups(
  user_groups: Vec<String>,
  res: &mut ResourcesToml,
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
      .map(|permission| PermissionToml {
        target: permission.resource_target,
        level: permission.level,
      })
      .collect();
    res.user_groups.push(UserGroupToml {
      name: ug.name,
      users: ug
        .users
        .into_iter()
        .filter_map(|user_id| usernames.get(&user_id).cloned())
        .collect(),
      permissions,
    });
  }
  Ok(())
}

fn convert_resource<Config, Info: Default, PartialConfig>(
  resource: Resource<Config, Info>,
  tag_names: &HashMap<String, String>,
) -> ResourceToml<PartialConfig>
where
  Config: Into<PartialConfig>,
{
  ResourceToml {
    name: resource.name,
    tags: resource
      .tags
      .iter()
      .filter_map(|t| tag_names.get(t).cloned())
      .collect(),
    description: resource.description,
    config: resource.config.into(),
  }
}
