use monitor_client::{
  api::write::{
    CreateUserGroup, SetUsersInUserGroup, UpdatePermissionOnTarget,
  },
  entities::{
    permission::{PermissionLevel, UserTarget},
    update::ResourceTarget,
  },
};
use serde::Deserialize;

use crate::{maps::name_to_user_group, monitor_client};

#[derive(Debug, Clone, Deserialize)]
pub struct UserGroupToml {
  pub name: String,

  #[serde(default)]
  pub users: Vec<String>,

  #[serde(default)]
  pub permissions: Vec<PermissionToml>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PermissionToml {
  pub target: ResourceTarget,
  pub level: PermissionLevel,
}

pub fn get_updates(
  user_groups: Vec<UserGroupToml>,
) -> (Vec<UserGroupToml>, Vec<UserGroupToml>) {
  let map = name_to_user_group();

  let mut to_create = Vec::<UserGroupToml>::new();
  let mut to_update = Vec::<UserGroupToml>::new();

  for user_group in user_groups {
    match map.get(&user_group.name).cloned() {
      Some(_) => to_update.push(user_group),
      None => to_create.push(user_group),
    }
  }

  if !to_create.is_empty() {
    println!(
      "\nUSER GROUPS TO CREATE: {}",
      to_create
        .iter()
        .map(|item| item.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    );
  }

  if !to_update.is_empty() {
    println!(
      "\nUSER GROUPS TO UPDATE: {}",
      to_update
        .iter()
        .map(|item| item.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    );
  }

  (to_create, to_update)
}

pub async fn run_updates(
  to_create: Vec<UserGroupToml>,
  to_update: Vec<UserGroupToml>,
) {
  let log_after = !to_update.is_empty() || !to_create.is_empty();

  // Create the non-existant user groups
  for user_group in to_create {
    // Create the user group
    if let Err(e) = monitor_client()
      .write(CreateUserGroup {
        name: user_group.name.clone(),
      })
      .await
    {
      warn!(
        "failed to create user group {} | {e:#}",
        user_group.name
      );
      continue;
    };

    set_users(user_group.name.clone(), user_group.users).await;
    update_permissions(user_group.name, user_group.permissions).await;
  }

  // Update the existing user groups
  for user_group in to_update {
    set_users(user_group.name.clone(), user_group.users).await;
    update_permissions(user_group.name, user_group.permissions).await;
  }

  if log_after {
    info!("============ user groups synced ✅ ============");
  }
}

async fn set_users(user_group: String, users: Vec<String>) {
  if !users.is_empty() {
    if let Err(e) = monitor_client()
      .write(SetUsersInUserGroup {
        user_group: user_group.clone(),
        users,
      })
      .await
    {
      warn!("failed to set users in group {user_group} | {e:#}");
    }
  }
}

async fn update_permissions(
  user_group: String,
  permissions: Vec<PermissionToml>,
) {
  for PermissionToml { target, level } in permissions {
    if let Err(e) = monitor_client()
      .write(UpdatePermissionOnTarget {
        user_target: UserTarget::UserGroup(user_group.clone()),
        resource_target: target.clone(),
        permission: level,
      })
      .await
    {
      warn!(
        "failed to set permssion in group {user_group} | target: {target:?} | {e:#}",
      );
    }
  }
}
