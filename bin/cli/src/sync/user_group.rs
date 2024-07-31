use std::cmp::Ordering;

use anyhow::Context;
use colored::Colorize;
use monitor_client::{
  api::{
    read::ListUserTargetPermissions,
    write::{
      CreateUserGroup, DeleteUserGroup, SetUsersInUserGroup,
      UpdatePermissionOnTarget,
    },
  },
  entities::{
    permission::UserTarget,
    toml::{PermissionToml, UserGroupToml},
    update::ResourceTarget,
  },
};

use crate::maps::{
  id_to_alerter, id_to_build, id_to_builder, id_to_deployment, id_to_procedure, id_to_repo, id_to_resource_sync, id_to_server, id_to_server_template, id_to_stack, id_to_user, name_to_user_group
};

pub struct UpdateItem {
  user_group: UserGroupToml,
  update_users: bool,
  update_permissions: bool,
}

pub struct DeleteItem {
  id: String,
  name: String,
}

pub async fn get_updates(
  user_groups: Vec<UserGroupToml>,
  delete: bool,
) -> anyhow::Result<(
  Vec<UserGroupToml>,
  Vec<UpdateItem>,
  Vec<DeleteItem>,
)> {
  let map = name_to_user_group();

  let mut to_create = Vec::<UserGroupToml>::new();
  let mut to_update = Vec::<UpdateItem>::new();
  let mut to_delete = Vec::<DeleteItem>::new();

  if delete {
    for user_group in map.values() {
      if !user_groups.iter().any(|ug| ug.name == user_group.name) {
        to_delete.push(DeleteItem {
          id: user_group.id.clone(),
          name: user_group.name.clone(),
        });
      }
    }
  }

  let id_to_user = id_to_user();

  for mut user_group in user_groups {
    let original = match map.get(&user_group.name).cloned() {
      Some(original) => original,
      None => {
        println!(
          "\n{}: user group: {}\n{}: {:?}\n{}: {:?}",
          "CREATE".green(),
          user_group.name.bold().green(),
          "users".dimmed(),
          user_group.users,
          "permissions".dimmed(),
          user_group.permissions,
        );
        to_create.push(user_group);
        continue;
      }
    };

    let mut original_users = original
      .users
      .into_iter()
      .filter_map(|user_id| {
        id_to_user.get(&user_id).map(|u| u.username.clone())
      })
      .collect::<Vec<_>>();

    let mut original_permissions = crate::state::monitor_client()
      .read(ListUserTargetPermissions {
        user_target: UserTarget::UserGroup(original.id),
      })
      .await
      .context("failed to query for existing UserGroup permissions")?
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
          ResourceTarget::ResourceSync(id) => {
            *id = id_to_resource_sync()
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Stack(id) => {
            *id = id_to_stack()
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

    original_users.sort();
    user_group.users.sort();

    user_group.permissions.sort_by(sort_permissions);
    original_permissions.sort_by(sort_permissions);

    let update_users = user_group.users != original_users;
    let update_permissions =
      user_group.permissions != original_permissions;

    // only push update after failed diff
    if update_users || update_permissions {
      println!(
        "\n{}: user group: '{}'\n-------------------",
        "UPDATE".blue(),
        user_group.name.bold(),
      );
      let mut lines = Vec::<String>::new();
      if update_users {
        let adding = user_group
          .users
          .iter()
          .filter(|user| !original_users.contains(user))
          .map(|user| user.as_str())
          .collect::<Vec<_>>();
        let adding = if adding.is_empty() {
          String::from("None").into()
        } else {
          adding.join(", ").green()
        };
        let removing = original_users
          .iter()
          .filter(|user| !user_group.users.contains(user))
          .map(|user| user.as_str())
          .collect::<Vec<_>>();
        let removing = if removing.is_empty() {
          String::from("None").into()
        } else {
          removing.join(", ").red()
        };
        lines.push(format!(
          "{}:    'users'\n{}: {removing}\n{}:   {adding}",
          "field".dimmed(),
          "removing".dimmed(),
          "adding".dimmed(),
        ))
      }
      if update_permissions {
        let adding = user_group
          .permissions
          .iter()
          .filter(|permission| {
            !original_permissions.contains(permission)
          })
          .map(|permission| format!("{permission:?}"))
          .collect::<Vec<_>>();
        let adding = if adding.is_empty() {
          String::from("None").into()
        } else {
          adding.join(", ").green()
        };
        let removing = original_permissions
          .iter()
          .filter(|permission| {
            !user_group.permissions.contains(permission)
          })
          .map(|permission| format!("{permission:?}"))
          .collect::<Vec<_>>();
        let removing = if removing.is_empty() {
          String::from("None").into()
        } else {
          removing.join(", ").red()
        };
        lines.push(format!(
          "{}:    'permissions'\n{}: {removing}\n{}:   {adding}",
          "field".dimmed(),
          "removing".dimmed(),
          "adding".dimmed()
        ))
      }
      println!("{}", lines.join("\n-------------------\n"));
      to_update.push(UpdateItem {
        user_group,
        update_users,
        update_permissions,
      });
    }
  }

  for d in &to_delete {
    println!(
      "\n{}: user group: '{}'\n-------------------",
      "DELETE".red(),
      d.name.bold(),
    );
  }

  Ok((to_create, to_update, to_delete))
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
  to_update: Vec<UpdateItem>,
  to_delete: Vec<DeleteItem>,
) {
  // Create the non-existant user groups
  for user_group in to_create {
    // Create the user group
    if let Err(e) = crate::state::monitor_client()
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
    } else {
      info!(
        "{} user group '{}'",
        "created".green().bold(),
        user_group.name.bold(),
      );
    };

    set_users(user_group.name.clone(), user_group.users).await;
    run_update_permissions(user_group.name, user_group.permissions)
      .await;
  }

  // Update the existing user groups
  for UpdateItem {
    user_group,
    update_users,
    update_permissions,
  } in to_update
  {
    if update_users {
      set_users(user_group.name.clone(), user_group.users).await;
    }
    if update_permissions {
      run_update_permissions(user_group.name, user_group.permissions)
        .await;
    }
  }

  for user_group in to_delete {
    if let Err(e) = crate::state::monitor_client()
      .write(DeleteUserGroup { id: user_group.id })
      .await
    {
      warn!(
        "failed to delete user group {} | {e:#}",
        user_group.name
      );
    } else {
      info!(
        "{} user group '{}'",
        "deleted".red().bold(),
        user_group.name.bold(),
      );
    }
  }
}

async fn set_users(user_group: String, users: Vec<String>) {
  if let Err(e) = crate::state::monitor_client()
    .write(SetUsersInUserGroup {
      user_group: user_group.clone(),
      users,
    })
    .await
  {
    warn!("failed to set users in group {user_group} | {e:#}");
  } else {
    info!(
      "{} user group '{}' users",
      "updated".blue().bold(),
      user_group.bold(),
    );
  }
}

async fn run_update_permissions(
  user_group: String,
  permissions: Vec<PermissionToml>,
) {
  for PermissionToml { target, level } in permissions {
    if let Err(e) = crate::state::monitor_client()
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
    } else {
      info!(
        "{} user group '{}' permissions",
        "updated".blue().bold(),
        user_group.bold(),
      );
    }
  }
}
