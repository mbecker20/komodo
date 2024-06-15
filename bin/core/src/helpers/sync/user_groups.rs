use std::{cmp::Ordering, collections::HashMap};

use anyhow::Context;
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
    sync::SyncUpdate,
    toml::{PermissionToml, UserGroupToml},
    update::{Log, ResourceTarget},
    user::sync_user,
  },
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::{
  helpers::formatting::{bold, colored, muted, Color},
  state::{db_client, State},
};

use super::resource::AllResourcesById;

pub struct UpdateItem {
  user_group: UserGroupToml,
  update_users: bool,
  update_permissions: bool,
}

pub struct DeleteItem {
  id: String,
  name: String,
}

pub async fn get_updates_for_view(
  user_groups: Vec<UserGroupToml>,
  delete: bool,
  all_resources: &AllResourcesById,
) -> anyhow::Result<Option<SyncUpdate>> {
  let map = find_collect(&db_client().await.user_groups, None, None)
    .await
    .context("failed to query db for UserGroups")?
    .into_iter()
    .map(|ug| (ug.name.clone(), ug))
    .collect::<HashMap<_, _>>();

  let mut update = SyncUpdate {
    log: String::from("User Group Updates"),
    ..Default::default()
  };

  let mut to_delete = Vec::<String>::new();

  if delete {
    for user_group in map.values() {
      if !user_groups.iter().any(|ug| ug.name == user_group.name) {
        update.to_delete += 1;
        to_delete.push(user_group.name.clone());
      }
    }
  }

  let id_to_user = find_collect(&db_client().await.users, None, None)
    .await
    .context("failed to query db for Users")?
    .into_iter()
    .map(|user| (user.id.clone(), user))
    .collect::<HashMap<_, _>>();

  for mut user_group in user_groups {
    let original = match map.get(&user_group.name).cloned() {
      Some(original) => original,
      None => {
        update.to_create += 1;
        update.log.push_str(&format!(
          "\n\n{}: user group: {}\n{}: {:?}\n{}: {:?}",
          colored("CREATE", Color::Green),
          colored(&user_group.name, Color::Green),
          muted("users"),
          user_group.users,
          muted("permissions"),
          user_group.permissions,
        ));
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

    let mut original_permissions = State
      .resolve(
        ListUserTargetPermissions {
          user_target: UserTarget::UserGroup(original.id),
        },
        sync_user().to_owned(),
      )
      .await
      .context("failed to query for existing UserGroup permissions")?
      .into_iter()
      .map(|mut p| {
        // replace the ids with names
        match &mut p.resource_target {
          ResourceTarget::System(_) => {}
          ResourceTarget::Build(id) => {
            *id = all_resources
              .builds
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Builder(id) => {
            *id = all_resources
              .builders
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Deployment(id) => {
            *id = all_resources
              .deployments
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Server(id) => {
            *id = all_resources
              .servers
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Repo(id) => {
            *id = all_resources
              .repos
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Alerter(id) => {
            *id = all_resources
              .alerters
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Procedure(id) => {
            *id = all_resources
              .procedures
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::ServerTemplate(id) => {
            *id = all_resources
              .templates
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::ResourceSync(id) => {
            *id = all_resources
              .syncs
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

    // only add log after diff detected
    if update_users || update_permissions {
      update.to_update += 1;
      update.log.push_str(&format!(
        "\n\n{}: user group: '{}'\n-------------------",
        colored("UPDATE", Color::Blue),
        bold(&user_group.name),
      ));
      let mut lines = Vec::<String>::new();
      if update_users {
        let adding = user_group
          .users
          .iter()
          .filter(|user| !original_users.contains(user))
          .map(|user| user.as_str())
          .collect::<Vec<_>>();
        let adding = if adding.is_empty() {
          String::from("None")
        } else {
          colored(&adding.join(", "), Color::Green)
        };
        let removing = original_users
          .iter()
          .filter(|user| !user_group.users.contains(user))
          .map(|user| user.as_str())
          .collect::<Vec<_>>();
        let removing = if removing.is_empty() {
          String::from("None")
        } else {
          colored(&removing.join(", "), Color::Red)
        };
        lines.push(format!(
          "{}:    'users'\n{}: {removing}\n{}:   {adding}",
          muted("field"),
          muted("removing"),
          muted("adding"),
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
          String::from("None")
        } else {
          colored(&adding.join(", "), Color::Green)
        };
        let removing = original_permissions
          .iter()
          .filter(|permission| {
            !user_group.permissions.contains(permission)
          })
          .map(|permission| format!("{permission:?}"))
          .collect::<Vec<_>>();
        let removing = if removing.is_empty() {
          String::from("None")
        } else {
          colored(&removing.join(", "), Color::Red)
        };
        lines.push(format!(
          "{}:    'permissions'\n{}: {removing}\n{}:   {adding}",
          muted("field"),
          muted("removing"),
          muted("adding"),
        ))
      }
      update.log.push('\n');
      update.log.push_str(&lines.join("\n-------------------\n"));
    }
  }

  for name in &to_delete {
    update.log.push_str(&format!(
      "\n\n{}: user group: '{}'\n-------------------",
      colored("DELETE", Color::Red),
      bold(name),
    ));
  }

  let any_change = update.to_create > 0
    || update.to_update > 0
    || update.to_delete > 0;

  Ok(any_change.then_some(update))
}

pub async fn get_updates_for_execution(
  user_groups: Vec<UserGroupToml>,
  delete: bool,
  all_resources: &AllResourcesById,
) -> anyhow::Result<(
  Vec<UserGroupToml>,
  Vec<UpdateItem>,
  Vec<DeleteItem>,
)> {
  let map = find_collect(&db_client().await.user_groups, None, None)
    .await
    .context("failed to query db for UserGroups")?
    .into_iter()
    .map(|ug| (ug.name.clone(), ug))
    .collect::<HashMap<_, _>>();

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

  if user_groups.is_empty() {
    return Ok((to_create, to_update, to_delete));
  }

  let id_to_user = find_collect(&db_client().await.users, None, None)
    .await
    .context("failed to query db for Users")?
    .into_iter()
    .map(|user| (user.id.clone(), user))
    .collect::<HashMap<_, _>>();

  for mut user_group in user_groups {
    let original = match map.get(&user_group.name).cloned() {
      Some(original) => original,
      None => {
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

    let mut original_permissions = State
      .resolve(
        ListUserTargetPermissions {
          user_target: UserTarget::UserGroup(original.id),
        },
        sync_user().to_owned(),
      )
      .await
      .context("failed to query for existing UserGroup permissions")?
      .into_iter()
      .map(|mut p| {
        // replace the ids with names
        match &mut p.resource_target {
          ResourceTarget::System(_) => {}
          ResourceTarget::Build(id) => {
            *id = all_resources
              .builds
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Builder(id) => {
            *id = all_resources
              .builders
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Deployment(id) => {
            *id = all_resources
              .deployments
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Server(id) => {
            *id = all_resources
              .servers
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Repo(id) => {
            *id = all_resources
              .repos
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Alerter(id) => {
            *id = all_resources
              .alerters
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::Procedure(id) => {
            *id = all_resources
              .procedures
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::ServerTemplate(id) => {
            *id = all_resources
              .templates
              .get(id)
              .map(|b| b.name.clone())
              .unwrap_or_default()
          }
          ResourceTarget::ResourceSync(id) => {
            *id = all_resources
              .syncs
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
      to_update.push(UpdateItem {
        user_group,
        update_users,
        update_permissions,
      });
    }
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
) -> Option<Log> {
  if to_create.is_empty()
    && to_update.is_empty()
    && to_delete.is_empty()
  {
    return None;
  }

  let mut has_error = false;
  let mut log = String::from("running updates on UserGroups");

  // Create the non-existant user groups
  for user_group in to_create {
    // Create the user group
    if let Err(e) = State
      .resolve(
        CreateUserGroup {
          name: user_group.name.clone(),
        },
        sync_user().to_owned(),
      )
      .await
    {
      has_error = true;
      log.push_str(&format!(
        "\n{}: failed to create user group '{}' | {e:#}",
        colored("ERROR", Color::Red),
        bold(&user_group.name)
      ));
      continue;
    } else {
      log.push_str(&format!(
        "\n{}: {} user group '{}'",
        muted("INFO"),
        colored("created", Color::Green),
        bold(&user_group.name)
      ))
    };

    set_users(
      user_group.name.clone(),
      user_group.users,
      &mut log,
      &mut has_error,
    )
    .await;
    run_update_permissions(
      user_group.name,
      user_group.permissions,
      &mut log,
      &mut has_error,
    )
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
      set_users(
        user_group.name.clone(),
        user_group.users,
        &mut log,
        &mut has_error,
      )
      .await;
    }
    if update_permissions {
      run_update_permissions(
        user_group.name,
        user_group.permissions,
        &mut log,
        &mut has_error,
      )
      .await;
    }
  }

  for user_group in to_delete {
    if let Err(e) = State
      .resolve(
        DeleteUserGroup { id: user_group.id },
        sync_user().to_owned(),
      )
      .await
    {
      has_error = true;
      log.push_str(&format!(
        "\n{}: failed to delete user group '{}' | {e:#}",
        colored("ERROR", Color::Red),
        bold(&user_group.name)
      ))
    } else {
      log.push_str(&format!(
        "\n{}: {} user group '{}'",
        muted("INFO"),
        colored("deleted", Color::Red),
        bold(&user_group.name)
      ))
    }
  }

  let stage = "Update UserGroups";
  Some(if has_error {
    Log::error(stage, log)
  } else {
    Log::simple(stage, log)
  })
}

async fn set_users(
  user_group: String,
  users: Vec<String>,
  log: &mut String,
  has_error: &mut bool,
) {
  if let Err(e) = State
    .resolve(
      SetUsersInUserGroup {
        user_group: user_group.clone(),
        users,
      },
      sync_user().to_owned(),
    )
    .await
  {
    *has_error = true;
    log.push_str(&format!(
      "\n{}: failed to set users in group {} | {e:#}",
      colored("ERROR", Color::Red),
      bold(&user_group)
    ))
  } else {
    log.push_str(&format!(
      "\n{}: {} user group '{}' users",
      muted("INFO"),
      colored("updated", Color::Blue),
      bold(&user_group)
    ))
  }
}

async fn run_update_permissions(
  user_group: String,
  permissions: Vec<PermissionToml>,
  log: &mut String,
  has_error: &mut bool,
) {
  for PermissionToml { target, level } in permissions {
    if let Err(e) = State
      .resolve(
        UpdatePermissionOnTarget {
          user_target: UserTarget::UserGroup(user_group.clone()),
          resource_target: target.clone(),
          permission: level,
        },
        sync_user().to_owned(),
      )
      .await
    {
      *has_error = true;
      log.push_str(&format!(
        "\n{}: failed to set permssion in group {} | target: {target:?} | {e:#}",
        colored("ERROR", Color::Red),
        bold(&user_group)
      ))
    } else {
      log.push_str(&format!(
        "\n{}: {} user group '{}' permissions",
        muted("INFO"),
        colored("updated", Color::Blue),
        bold(&user_group)
      ))
    }
  }
}
