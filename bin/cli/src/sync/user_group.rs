use std::cmp::Ordering;

use anyhow::Context;
use monitor_client::{
  api::{
    read::ListUserTargetPermissions,
    write::{
      CreateUserGroup, SetUsersInUserGroup, UpdatePermissionOnTarget,
    },
  },
  entities::{
    permission::UserTarget,
    toml::{PermissionToml, UserGroupToml},
    update::ResourceTarget,
  },
};

use crate::{
  maps::{
    id_to_alerter, id_to_build, id_to_builder, id_to_deployment,
    id_to_procedure, id_to_repo, id_to_server, id_to_server_template,
    id_to_user, name_to_user_group,
  },
  monitor_client,
};

pub async fn get_updates(
  user_groups: Vec<UserGroupToml>,
) -> anyhow::Result<(Vec<UserGroupToml>, Vec<UserGroupToml>)> {
  let map = name_to_user_group();

  let mut to_create = Vec::<UserGroupToml>::new();
  let mut to_update = Vec::<UserGroupToml>::new();

  for mut user_group in user_groups {
    match map.get(&user_group.name).cloned() {
      Some(original) => {
        // replace the user ids with usernames
        let mut users = original
          .users
          .into_iter()
          .filter_map(|user_id| {
            id_to_user().get(&user_id).map(|u| u.username.clone())
          })
          .collect::<Vec<_>>();

        let mut permissions = monitor_client()
          .read(ListUserTargetPermissions {
            user_target: UserTarget::UserGroup(original.id),
          })
          .await
          .context("failed to query for UserGroup permissions")?
          .into_iter()
          .map(|mut p| {
            // replace the ids with names
            match &mut p.resource_target {
              ResourceTarget::System(_) => {}
              ResourceTarget::Build(id) => {
                *id = id_to_build()
                  .get(id)
                  .map(|b| b.name.clone())
                  .unwrap_or_default()
              }
              ResourceTarget::Builder(id) => {
                *id = id_to_builder()
                  .get(id)
                  .map(|b| b.name.clone())
                  .unwrap_or_default()
              }
              ResourceTarget::Deployment(id) => {
                *id = id_to_deployment()
                  .get(id)
                  .map(|b| b.name.clone())
                  .unwrap_or_default()
              }
              ResourceTarget::Server(id) => {
                *id = id_to_server()
                  .get(id)
                  .map(|b| b.name.clone())
                  .unwrap_or_default()
              }
              ResourceTarget::Repo(id) => {
                *id = id_to_repo()
                  .get(id)
                  .map(|b| b.name.clone())
                  .unwrap_or_default()
              }
              ResourceTarget::Alerter(id) => {
                *id = id_to_alerter()
                  .get(id)
                  .map(|b| b.name.clone())
                  .unwrap_or_default()
              }
              ResourceTarget::Procedure(id) => {
                *id = id_to_procedure()
                  .get(id)
                  .map(|b| b.name.clone())
                  .unwrap_or_default()
              }
              ResourceTarget::ServerTemplate(id) => {
                *id = id_to_server_template()
                  .get(id)
                  .map(|b| b.name.clone())
                  .unwrap_or_default()
              }
            }
            PermissionToml {
              target: p.resource_target,
              level: p.level,
            }
          })
          .collect::<Vec<_>>();

        users.sort();
        user_group.users.sort();

        user_group.permissions.sort_by(sort_permissions);
        permissions.sort_by(sort_permissions);

        // only push update after failed diff
        if user_group.users != users
          || user_group.permissions != permissions
        {
          // no update from users
          to_update.push(user_group);
        }
      }
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

  Ok((to_create, to_update))
}

/// order permissions in deterministic way
fn sort_permissions(
  a: &PermissionToml,
  b: &PermissionToml,
) -> Ordering {
  let (a_t, a_id) = a.target.extract_variant_id();
  let (b_t, b_id) = b.target.extract_variant_id();
  match (a_t.cmp(&b_t), a_id.cmp(b_id)) {
    (Ordering::Greater, _) => Ordering::Greater,
    (Ordering::Less, _) => Ordering::Less,
    (_, Ordering::Greater) => Ordering::Greater,
    (_, Ordering::Less) => Ordering::Less,
    _ => Ordering::Equal,
  }
}

pub async fn run_updates(
  to_create: Vec<UserGroupToml>,
  to_update: Vec<UserGroupToml>,
) {
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
