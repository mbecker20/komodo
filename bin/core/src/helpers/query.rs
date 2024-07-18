use std::{
  collections::{HashMap, HashSet},
  str::FromStr,
};

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  deployment::{Deployment, DeploymentState},
  permission::PermissionLevel,
  server::{Server, ServerState},
  tag::Tag,
  update::{ResourceTargetVariant, Update},
  user::{admin_service_user, User},
  user_group::UserGroup,
  variable::Variable,
  Operation,
};
use mungos::{
  by_id::find_one_by_id,
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::FindOneOptions,
  },
};

use crate::{config::core_config, resource, state::db_client};

#[instrument(level = "debug")]
pub async fn get_user(user_id: &str) -> anyhow::Result<User> {
  if let Some(user) = admin_service_user(user_id) {
    return Ok(user);
  }
  find_one_by_id(&db_client().await.users, user_id)
    .await
    .context("failed to query mongo for user")?
    .with_context(|| format!("no user found with id {user_id}"))
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

#[instrument(level = "debug")]
pub async fn get_user_permission_on_resource(
  user: &User,
  resource_variant: ResourceTargetVariant,
  resource_id: &str,
) -> anyhow::Result<PermissionLevel> {
  if user.admin {
    return Ok(PermissionLevel::Write);
  }

  // Start with base of Read or None
  let mut base = if core_config().transparent_mode {
    PermissionLevel::Read
  } else {
    PermissionLevel::None
  };

  // Overlay users base on resource variant
  if let Some(level) = user.all.get(&resource_variant).cloned() {
    if level > base {
      base = level;
    }
  }
  if base == PermissionLevel::Write {
    // No reason to keep going if already Write at this point.
    return Ok(PermissionLevel::Write);
  }

  // Overlay any user groups base on resource variant
  let groups = get_user_user_groups(&user.id).await?;
  for group in &groups {
    if let Some(level) = group.all.get(&resource_variant).cloned() {
      if level > base {
        base = level;
      }
    }
  }
  if base == PermissionLevel::Write {
    // No reason to keep going if already Write at this point.
    return Ok(PermissionLevel::Write);
  }

  // Overlay any specific permissions
  let permission = find_collect(
    &db_client().await.permissions,
    doc! {
      "$or": user_target_query(&user.id, &groups)?,
      "resource_target.type": resource_variant.as_ref(),
      "resource_target.id": resource_id
    },
    None,
  )
  .await
  .context("failed to query db for permissions")?
  .into_iter()
  // get the max permission user has between personal / any user groups
  .fold(base, |level, permission| {
    if permission.level > level {
      permission.level
    } else {
      level
    }
  });
  Ok(permission)
}

/// Returns None if still no need to filter by resource id (eg transparent mode, group membership with all access).
#[instrument(level = "debug")]
pub async fn get_resource_ids_for_user(
  user: &User,
  resource_type: ResourceTargetVariant,
) -> anyhow::Result<Option<Vec<ObjectId>>> {
  // Check admin or transparent mode
  if user.admin || core_config().transparent_mode {
    return Ok(None);
  }

  // Check user 'all' on variant
  if let Some(level) = user.all.get(&resource_type).cloned() {
    if level > PermissionLevel::None {
      return Ok(None);
    }
  }

  // Check user groups 'all' on variant
  let groups = get_user_user_groups(&user.id).await?;
  for group in &groups {
    if let Some(level) = group.all.get(&resource_type).cloned() {
      if level > PermissionLevel::None {
        return Ok(None);
      }
    }
  }

  // Get specific ids
  let ids = find_collect(
    &db_client().await.permissions,
    doc! {
      "$or": user_target_query(&user.id, &groups)?,
      "resource_target.type": resource_type.as_ref(),
      "level": { "$in": ["Read", "Execute", "Write"] }
    },
    None,
  )
  .await
  .context("failed to query permissions on db")?
  .into_iter()
  .map(|p| p.resource_target.extract_variant_id().1.to_string())
  // collect into hashset first to remove any duplicates
  .collect::<HashSet<_>>()
  .into_iter()
  .flat_map(|id| ObjectId::from_str(&id))
  .collect::<Vec<_>>();

  Ok(Some(ids))
}

pub fn id_or_name_filter(id_or_name: &str) -> Document {
  match ObjectId::from_str(id_or_name) {
    Ok(id) => doc! { "_id": id },
    Err(_) => doc! { "name": id_or_name },
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
