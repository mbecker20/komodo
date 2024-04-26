use std::collections::HashMap;

use anyhow::Context;
use axum::async_trait;
use monitor_client::{
  api::read::{
    ExportAllResourcesToToml, ExportAllResourcesToTomlResponse,
    ExportResourcesToToml, ExportResourcesToTomlResponse,
    GetUserGroup, ListUserTargetPermissions,
  },
  entities::{
    alerter::Alerter,
    build::Build,
    builder::Builder,
    deployment::Deployment,
    permission::{PermissionLevel, UserTarget},
    procedure::Procedure,
    repo::Repo,
    resource::Resource,
    server::Server,
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
  db::db_client,
  helpers::{
    query::get_user_user_group_ids, resource::StateResource,
  },
  state::State,
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
      Alerter::list_resources_for_user(Default::default(), &user)
        .await?
        .into_iter()
        .map(|resource| ResourceTarget::Alerter(resource.id)),
    );
    targets.extend(
      Builder::list_resources_for_user(Default::default(), &user)
        .await?
        .into_iter()
        .map(|resource| ResourceTarget::Builder(resource.id)),
    );
    targets.extend(
      Server::list_resources_for_user(Default::default(), &user)
        .await?
        .into_iter()
        .map(|resource| ResourceTarget::Server(resource.id)),
    );
    targets.extend(
      Deployment::list_resources_for_user(Default::default(), &user)
        .await?
        .into_iter()
        .map(|resource| ResourceTarget::Deployment(resource.id)),
    );
    targets.extend(
      Build::list_resources_for_user(Default::default(), &user)
        .await?
        .into_iter()
        .map(|resource| ResourceTarget::Build(resource.id)),
    );
    targets.extend(
      Repo::list_resources_for_user(Default::default(), &user)
        .await?
        .into_iter()
        .map(|resource| ResourceTarget::Repo(resource.id)),
    );
    targets.extend(
      Procedure::list_resources_for_user(Default::default(), &user)
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
    let tag_names = find_collect(&db_client().await.tags, None, None)
      .await
      .context("failed to get all tags")?
      .into_iter()
      .map(|t| (t.id, t.name))
      .collect::<HashMap<_, _>>();
    for target in targets {
      match target {
        ResourceTarget::Alerter(id) => {
          let alerter = Alerter::get_resource_check_permissions(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res.alerters.push(convert_resource(alerter, &tag_names))
        }
        ResourceTarget::Build(id) => {
          let build = Build::get_resource_check_permissions(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res.builds.push(convert_resource(build, &tag_names))
        }
        ResourceTarget::Builder(id) => {
          let builder = Builder::get_resource_check_permissions(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res.builders.push(convert_resource(builder, &tag_names))
        }
        ResourceTarget::Deployment(id) => {
          let deployment =
            Deployment::get_resource_check_permissions(
              &id,
              &user,
              PermissionLevel::Read,
            )
            .await?;
          res
            .deployments
            .push(convert_resource(deployment, &tag_names))
        }
        ResourceTarget::Server(id) => {
          let server = Server::get_resource_check_permissions(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res.servers.push(convert_resource(server, &tag_names))
        }
        ResourceTarget::Repo(id) => {
          let repo = Repo::get_resource_check_permissions(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res.repos.push(convert_resource(repo, &tag_names))
        }
        ResourceTarget::Procedure(id) => {
          let procedure = Procedure::get_resource_check_permissions(
            &id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
          res.procedures.push(convert_resource(procedure, &tag_names))
        }
        ResourceTarget::System(_) => continue,
      };
    }

    let db = db_client().await;

    let usernames = find_collect(&db.users, None, None)
      .await?
      .into_iter()
      .map(|user| (user.id, user.username))
      .collect::<HashMap<_, _>>();

    for user_group in user_groups {
      let ug = self
        .resolve(GetUserGroup { user_group }, user.clone())
        .await?;
      // this method is admin only, but we already know user can see user group if above does not return Err
      let permissions = self
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

    let toml = toml::to_string_pretty(&res)
      .context("failed to serialize resources to toml")?;
    Ok(ExportResourcesToTomlResponse { toml })
  }
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
    updated_at: 0,
  }
}
