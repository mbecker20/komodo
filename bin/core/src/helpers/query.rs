use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  alerter::Alerter,
  build::Build,
  builder::Builder,
  deployment::{Deployment, DeploymentState},
  permission::PermissionLevel,
  procedure::Procedure,
  repo::Repo,
  server::{Server, ServerState},
  server_template::ServerTemplate,
  stack::{ComposeProject, Stack, StackState},
  sync::ResourceSync,
  tag::Tag,
  update::{ResourceTarget, ResourceTargetVariant, Update},
  user::{admin_service_user, User},
  user_group::UserGroup,
  variable::Variable,
  Operation,
};
use mungos::{
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::FindOneOptions,
  },
};

use crate::{
  resource::{self, get_user_permission_on_resource},
  state::db_client,
};

#[instrument(level = "debug")]
// user: Id or username
pub async fn get_user(user: &str) -> anyhow::Result<User> {
  if let Some(user) = admin_service_user(user) {
    return Ok(user);
  }
  db_client()
    .await
    .users
    .find_one(id_or_username_filter(user))
    .await
    .context("failed to query mongo for user")?
    .with_context(|| format!("no user found with {user}"))
}

#[instrument(level = "debug")]
pub async fn get_server_with_status(
  server_id_or_name: &str,
) -> anyhow::Result<(Server, ServerState)> {
  let server = resource::get::<Server>(server_id_or_name).await?;
  if !server.config.enabled {
    return Ok((server, ServerState::Disabled));
  }
  let status = match super::periphery_client(&server)?
    .request(periphery_client::api::GetHealth {})
    .await
  {
    Ok(_) => ServerState::Ok,
    Err(_) => ServerState::NotOk,
  };
  Ok((server, status))
}

#[instrument(level = "debug")]
pub async fn get_deployment_state(
  deployment: &Deployment,
) -> anyhow::Result<DeploymentState> {
  if deployment.config.server_id.is_empty() {
    return Ok(DeploymentState::NotDeployed);
  }
  let (server, status) =
    get_server_with_status(&deployment.config.server_id).await?;
  if status != ServerState::Ok {
    return Ok(DeploymentState::Unknown);
  }
  let container = super::periphery_client(&server)?
    .request(periphery_client::api::container::GetContainerList {})
    .await?
    .into_iter()
    .find(|container| container.name == deployment.name);

  let state = match container {
    Some(container) => container.state,
    None => DeploymentState::NotDeployed,
  };

  Ok(state)
}

/// Can pass all the containers from the same server
pub fn get_stack_state_from_projects(
  stack: &Stack,
  projects: &[ComposeProject],
) -> StackState {
  let project_name = stack.project_name(false);
  let Some(status) = projects
    .iter()
    .find(|project| project.name == project_name)
    .and_then(|project| project.status.as_deref())
  else {
    return StackState::Down;
  };
  let Ok(states) = status
    .split(", ")
    .filter_map(|state| state.split('(').next())
    .map(|state| {
      state.parse::<DeploymentState>().with_context(|| {
        format!("failed to parse stack state entry: {state}")
      })
    })
    .collect::<anyhow::Result<Vec<_>>>()
    .inspect_err(|e| warn!("{e:#}"))
  else {
    return StackState::Unknown;
  };
  if states.is_empty() {
    return StackState::Down;
  }
  if states.len() > 1 {
    return StackState::Unhealthy;
  }
  match states[0] {
    DeploymentState::Unknown => StackState::Unknown,
    DeploymentState::NotDeployed => StackState::Down,
    DeploymentState::Created => StackState::Created,
    DeploymentState::Restarting => StackState::Restarting,
    DeploymentState::Running => StackState::Running,
    DeploymentState::Removing => StackState::Removing,
    DeploymentState::Paused => StackState::Paused,
    DeploymentState::Exited => StackState::Stopped,
    DeploymentState::Dead => StackState::Dead,
  }
}

/// Gets stack state fresh from periphery
#[instrument(level = "debug")]
pub async fn get_stack_state(
  stack: &Stack,
) -> anyhow::Result<StackState> {
  if stack.config.server_id.is_empty() {
    return Ok(StackState::Down);
  }
  let (server, status) =
    get_server_with_status(&stack.config.server_id).await?;
  if status != ServerState::Ok {
    return Ok(StackState::Unknown);
  }
  let projects = super::periphery_client(&server)?
    .request(periphery_client::api::compose::ListComposeProjects {})
    .await?;

  Ok(get_stack_state_from_projects(stack, &projects))
}

#[instrument(level = "debug")]
pub async fn get_tag(id_or_name: &str) -> anyhow::Result<Tag> {
  let query = match ObjectId::from_str(id_or_name) {
    Ok(id) => doc! { "_id": id },
    Err(_) => doc! { "name": id_or_name },
  };
  db_client()
    .await
    .tags
    .find_one(query)
    .await
    .context("failed to query mongo for tag")?
    .with_context(|| format!("no tag found matching {id_or_name}"))
}

#[instrument(level = "debug")]
pub async fn get_tag_check_owner(
  id_or_name: &str,
  user: &User,
) -> anyhow::Result<Tag> {
  let tag = get_tag(id_or_name).await?;
  if user.admin || tag.owner == user.id {
    return Ok(tag);
  }
  Err(anyhow!("user must be tag owner or admin"))
}

pub async fn get_id_to_tags(
  filter: impl Into<Option<Document>>,
) -> anyhow::Result<HashMap<String, Tag>> {
  let res = find_collect(&db_client().await.tags, filter, None)
    .await
    .context("failed to query db for tags")?
    .into_iter()
    .map(|tag| (tag.id.clone(), tag))
    .collect();
  Ok(res)
}

#[instrument(level = "debug")]
pub async fn get_user_user_groups(
  user_id: &str,
) -> anyhow::Result<Vec<UserGroup>> {
  find_collect(
    &db_client().await.user_groups,
    doc! {
      "users": user_id
    },
    None,
  )
  .await
  .context("failed to query db for user groups")
}

#[instrument(level = "debug")]
pub async fn get_user_user_group_ids(
  user_id: &str,
) -> anyhow::Result<Vec<String>> {
  let res = get_user_user_groups(user_id)
    .await?
    .into_iter()
    .map(|ug| ug.id)
    .collect();
  Ok(res)
}

pub fn user_target_query(
  user_id: &str,
  user_groups: &[UserGroup],
) -> anyhow::Result<Vec<Document>> {
  let mut user_target_query = vec![
    doc! { "user_target.type": "User", "user_target.id": user_id },
  ];
  let user_groups = user_groups.iter().map(|ug| {
    doc! {
      "user_target.type": "UserGroup", "user_target.id": &ug.id,
    }
  });
  user_target_query.extend(user_groups);
  Ok(user_target_query)
}

pub async fn get_user_permission_on_target(
  user: &User,
  target: &ResourceTarget,
) -> anyhow::Result<PermissionLevel> {
  match target {
    ResourceTarget::System(_) => Ok(PermissionLevel::None),
    ResourceTarget::Build(id) => {
      get_user_permission_on_resource::<Build>(user, id).await
    }
    ResourceTarget::Builder(id) => {
      get_user_permission_on_resource::<Builder>(user, id).await
    }
    ResourceTarget::Deployment(id) => {
      get_user_permission_on_resource::<Deployment>(user, id).await
    }
    ResourceTarget::Server(id) => {
      get_user_permission_on_resource::<Server>(user, id).await
    }
    ResourceTarget::Repo(id) => {
      get_user_permission_on_resource::<Repo>(user, id).await
    }
    ResourceTarget::Alerter(id) => {
      get_user_permission_on_resource::<Alerter>(user, id).await
    }
    ResourceTarget::Procedure(id) => {
      get_user_permission_on_resource::<Procedure>(user, id).await
    }
    ResourceTarget::ServerTemplate(id) => {
      get_user_permission_on_resource::<ServerTemplate>(user, id)
        .await
    }
    ResourceTarget::ResourceSync(id) => {
      get_user_permission_on_resource::<ResourceSync>(user, id).await
    }
    ResourceTarget::Stack(id) => {
      get_user_permission_on_resource::<Stack>(user, id).await
    }
  }
}

pub fn id_or_name_filter(id_or_name: &str) -> Document {
  match ObjectId::from_str(id_or_name) {
    Ok(id) => doc! { "_id": id },
    Err(_) => doc! { "name": id_or_name },
  }
}

pub fn id_or_username_filter(id_or_username: &str) -> Document {
  match ObjectId::from_str(id_or_username) {
    Ok(id) => doc! { "_id": id },
    Err(_) => doc! { "username": id_or_username },
  }
}

pub async fn get_global_variables(
) -> anyhow::Result<HashMap<String, String>> {
  Ok(
    find_collect(&db_client().await.variables, None, None)
      .await
      .context("failed to get all variables from db")?
      .into_iter()
      .map(|variable| (variable.name, variable.value))
      .collect(),
  )
}

pub async fn get_variable(name: &str) -> anyhow::Result<Variable> {
  db_client()
    .await
    .variables
    .find_one(doc! { "name": &name })
    .await
    .context("failed at call to db")?
    .with_context(|| {
      format!("no variable found with given name: {name}")
    })
}

pub async fn get_latest_update(
  resource_type: ResourceTargetVariant,
  id: &str,
  operation: Operation,
) -> anyhow::Result<Option<Update>> {
  db_client()
    .await
    .updates
    .find_one(doc! {
      "target.type": resource_type.as_ref(),
      "target.id": id,
      "operation": operation.as_ref()
    })
    .with_options(
      FindOneOptions::builder()
        .sort(doc! { "start_ts": -1 })
        .build(),
    )
    .await
    .context("failed to query db for latest update")
}
