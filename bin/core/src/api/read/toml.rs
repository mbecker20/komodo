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
    deployment::{Deployment, DeploymentImage},
    permission::{PermissionLevel, UserTarget},
    procedure::Procedure,
    repo::Repo,
    resource::{Resource, ResourceQuery},
    server::Server,
    server_template::ServerTemplate,
    sync::ResourceSync,
    toml::{
      PermissionToml, ResourceToml, ResourcesToml, UserGroupToml,
    },
    update::ResourceTarget,
    user::User,
  },
};
use mungos::find::find_collect;
use partial_derive2::PartialDiff;
use resolver_api::Resolve;

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
        ResourceQuery::builder().tags(tags).build(),
        &user,
      )
      .await?
      .into_iter()
      .map(|resource| ResourceTarget::ServerTemplate(resource.id)),
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
              names.servers.get(&id).unwrap_or(&String::new()),
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
          res.repos.push(convert_resource::<Repo>(repo, &names.tags))
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

    if include_variables {
      res.variables =
        find_collect(&db_client().await.variables, None, None)
          .await
          .context("failed to get variables from db")?;
    }

    let toml = toml::to_string(&res)
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
        Execution::Deploy(exec) => exec.deployment.clone_from(
          names
            .deployments
            .get(&exec.deployment)
            .unwrap_or(&String::new()),
        ),
        Execution::StartContainer(exec) => {
          exec.deployment.clone_from(
            names
              .deployments
              .get(&exec.deployment)
              .unwrap_or(&String::new()),
          )
        }
        Execution::StopContainer(exec) => exec.deployment.clone_from(
          names
            .deployments
            .get(&exec.deployment)
            .unwrap_or(&String::new()),
        ),
        Execution::RemoveContainer(exec) => {
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
        Execution::StopAllContainers(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::PruneNetworks(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::PruneImages(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::PruneContainers(exec) => exec.server.clone_from(
          names.servers.get(&exec.server).unwrap_or(&String::new()),
        ),
        Execution::RunSync(exec) => exec.sync.clone_from(
          names.syncs.get(&exec.sync).unwrap_or(&String::new()),
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
    config,
  }
}
