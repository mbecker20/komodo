use std::{cmp::Ordering, collections::HashMap};

use anyhow::Context;
use formatting::{bold, colored, muted, Color};
use komodo_client::{
  api::{
    read::ListUserTargetPermissions,
    write::{
      CreateUserGroup, DeleteUserGroup, SetUsersInUserGroup,
      UpdatePermissionOnResourceType, UpdatePermissionOnTarget,
    },
  },
  entities::{
    permission::{PermissionLevel, UserTarget},
    sync::DiffData,
    toml::{PermissionToml, UserGroupToml},
    update::Log,
    user::sync_user,
    ResourceTarget, ResourceTargetVariant,
  },
};
use mungos::find::find_collect;
use regex::Regex;
use resolver_api::Resolve;

use crate::state::{db_client, State};

use super::{toml::TOML_PRETTY_OPTIONS, AllResourcesById};

pub struct UpdateItem {
  user_group: UserGroupToml,
  update_users: bool,
  all_diff: HashMap<ResourceTargetVariant, PermissionLevel>,
}

pub struct DeleteItem {
  id: String,
  name: String,
}

pub async fn get_updates_for_view(
  user_groups: Vec<UserGroupToml>,
  delete: bool,
  all_resources: &AllResourcesById,
) -> anyhow::Result<Vec<DiffData>> {
  let map = find_collect(&db_client().await.user_groups, None, None)
    .await
    .context("failed to query db for UserGroups")?
    .into_iter()
    .map(|ug| (ug.name.clone(), ug))
    .collect::<HashMap<_, _>>();

  let mut diffs = Vec::<DiffData>::new();

  if delete {
    for user_group in map.values() {
      if !user_groups.iter().any(|ug| ug.name == user_group.name) {
        diffs.push(DiffData::Delete {
          current: format!(
            "[[user_group]]\n{}",
            toml_pretty::to_string(user_group, TOML_PRETTY_OPTIONS)
              .context("failed to serialize user group to toml")?
          ),
        });
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
    user_group
      .permissions
      .retain(|p| p.level > PermissionLevel::None);

    user_group.permissions = expand_user_group_permissions(
      user_group.permissions,
      all_resources,
    )
    .await
    .with_context(|| {
      format!(
        "failed to expand user group {} permissions",
        user_group.name
      )
    })?;

    let original = match map.get(&user_group.name).cloned() {
      Some(original) => original,
      None => {
        diffs.push(DiffData::Create {
          proposed: format!(
            "[[user_group]]\n{}",
            toml_pretty::to_string(&user_group, TOML_PRETTY_OPTIONS)
              .context("failed to serialize user group to toml")?
          ),
        });
        continue;
      }
    };

    let mut original_users = original
      .users
      .clone()
      .into_iter()
      .filter_map(|user_id| {
        id_to_user.get(&user_id).map(|u| u.username.clone())
      })
      .collect::<Vec<_>>();

    let mut original_permissions = State
      .resolve(
        ListUserTargetPermissions {
          user_target: UserTarget::UserGroup(original.id.clone()),
        },
        sync_user().to_owned(),
      )
      .await
      .context("failed to query for existing UserGroup permissions")?
      .into_iter()
      .filter(|p| p.level > PermissionLevel::None)
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
          ResourceTarget::Stack(id) => {
            *id = all_resources
              .stacks
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

    let all_diff = diff_group_all(&original.all, &user_group.all);

    user_group.permissions.sort_by(sort_permissions);
    original_permissions.sort_by(sort_permissions);

    let update_users = user_group.users != original_users;
    let update_all = !all_diff.is_empty();
    let update_permissions =
      user_group.permissions != original_permissions;

    // only add log after diff detected
    if update_users || update_all || update_permissions {
      diffs.push(DiffData::Update {
        proposed: format!(
          "[[user_group]]\n{}",
          toml_pretty::to_string(&user_group, TOML_PRETTY_OPTIONS)
            .context("failed to serialize user group to toml")?
        ),
        current: format!(
          "[[user_group]]\n{}",
          toml_pretty::to_string(&original, TOML_PRETTY_OPTIONS)
            .context("failed to serialize user group to toml")?
        ),
      });
    }
  }

  Ok(diffs)
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
    user_group
      .permissions
      .retain(|p| p.level > PermissionLevel::None);

    user_group.permissions = expand_user_group_permissions(
      user_group.permissions,
      all_resources,
    )
    .await
    .with_context(|| {
      format!(
        "failed to expand user group {} permissions",
        user_group.name
      )
    })?;

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
      .filter(|p| p.level > PermissionLevel::None)
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
          ResourceTarget::Stack(id) => {
            *id = all_resources
              .stacks
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

    let all_diff = diff_group_all(&original.all, &user_group.all);

    user_group.permissions.sort_by(sort_permissions);
    original_permissions.sort_by(sort_permissions);

    let update_users = user_group.users != original_users;

    // Extend permissions with any existing that have no target in incoming
    let to_remove = original_permissions
      .iter()
      .filter(|permission| {
        !user_group
          .permissions
          .iter()
          .any(|p| p.target == permission.target)
      })
      .map(|permission| PermissionToml {
        target: permission.target.clone(),
        level: PermissionLevel::None,
      })
      .collect::<Vec<_>>();
    user_group.permissions.extend(to_remove);

    // remove any permissions that already exist on original
    user_group.permissions.retain(|permission| {
      let Some(level) = original_permissions
        .iter()
        .find(|p| p.target == permission.target)
        .map(|p| p.level)
      else {
        // not in original, keep it
        return true;
      };
      // keep it if level doesn't match
      level != permission.level
    });

    // only push update after diff detected
    if update_users
      || !all_diff.is_empty()
      || !user_group.permissions.is_empty()
    {
      to_update.push(UpdateItem {
        user_group,
        update_users,
        all_diff: all_diff
          .into_iter()
          .map(|(k, (_, v))| (k, v))
          .collect(),
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
    run_update_all(
      user_group.name.clone(),
      user_group.all,
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
    all_diff,
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
    if !all_diff.is_empty() {
      run_update_all(
        user_group.name.clone(),
        all_diff,
        &mut log,
        &mut has_error,
      )
      .await;
    }
    if !user_group.permissions.is_empty() {
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

async fn run_update_all(
  user_group: String,
  all_diff: HashMap<ResourceTargetVariant, PermissionLevel>,
  log: &mut String,
  has_error: &mut bool,
) {
  for (resource_type, permission) in all_diff {
    if let Err(e) = State
      .resolve(
        UpdatePermissionOnResourceType {
          user_target: UserTarget::UserGroup(user_group.clone()),
          resource_type,
          permission,
        },
        sync_user().to_owned(),
      )
      .await
    {
      *has_error = true;
      log.push_str(&format!(
        "\n{}: failed to set base permissions on {resource_type} in group {} | {e:#}",
        colored("ERROR", Color::Red),
        bold(&user_group)
      ))
    } else {
      log.push_str(&format!(
        "\n{}: {} user group '{}' base permissions on {resource_type}",
        muted("INFO"),
        colored("updated", Color::Blue),
        bold(&user_group)
      ))
    }
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
        "\n{}: failed to set permission in group {} | target: {target:?} | {e:#}",
        colored("ERROR", Color::Red),
        bold(&user_group)
      ))
    } else {
      log.push_str(&format!(
        "\n{}: {} user group '{}' permissions | {}: {target:?} | {}: {level}",
        muted("INFO"),
        colored("updated", Color::Blue),
        bold(&user_group),
        muted("target"),
        muted("level")
      ))
    }
  }
}

/// Expands any regex defined targets into the full list
async fn expand_user_group_permissions(
  permissions: Vec<PermissionToml>,
  all_resources: &AllResourcesById,
) -> anyhow::Result<Vec<PermissionToml>> {
  let mut expanded =
    Vec::<PermissionToml>::with_capacity(permissions.capacity());

  for permission in permissions {
    let (variant, id) = permission.target.extract_variant_id();
    if id.is_empty() {
      continue;
    }
    if id.starts_with('\\') && id.ends_with('\\') {
      let inner = &id[1..(id.len() - 1)];
      let regex = Regex::new(inner)
        .with_context(|| format!("invalid regex. got: {inner}"))?;
      match variant {
        ResourceTargetVariant::Build => {
          let permissions = all_resources
            .builds
            .values()
            .filter(|resource| regex.is_match(&resource.name))
            .map(|resource| PermissionToml {
              target: ResourceTarget::Build(resource.name.clone()),
              level: permission.level,
            });
          expanded.extend(permissions);
        }
        ResourceTargetVariant::Builder => {
          let permissions = all_resources
            .builders
            .values()
            .filter(|resource| regex.is_match(&resource.name))
            .map(|resource| PermissionToml {
              target: ResourceTarget::Builder(resource.name.clone()),
              level: permission.level,
            });
          expanded.extend(permissions);
        }
        ResourceTargetVariant::Deployment => {
          let permissions = all_resources
            .deployments
            .values()
            .filter(|resource| regex.is_match(&resource.name))
            .map(|resource| PermissionToml {
              target: ResourceTarget::Deployment(
                resource.name.clone(),
              ),
              level: permission.level,
            });
          expanded.extend(permissions);
        }
        ResourceTargetVariant::Server => {
          let permissions = all_resources
            .servers
            .values()
            .filter(|resource| regex.is_match(&resource.name))
            .map(|resource| PermissionToml {
              target: ResourceTarget::Server(resource.name.clone()),
              level: permission.level,
            });
          expanded.extend(permissions);
        }
        ResourceTargetVariant::Repo => {
          let permissions = all_resources
            .repos
            .values()
            .filter(|resource| regex.is_match(&resource.name))
            .map(|resource| PermissionToml {
              target: ResourceTarget::Repo(resource.name.clone()),
              level: permission.level,
            });
          expanded.extend(permissions);
        }
        ResourceTargetVariant::Alerter => {
          let permissions = all_resources
            .alerters
            .values()
            .filter(|resource| regex.is_match(&resource.name))
            .map(|resource| PermissionToml {
              target: ResourceTarget::Alerter(resource.name.clone()),
              level: permission.level,
            });
          expanded.extend(permissions);
        }
        ResourceTargetVariant::Procedure => {
          let permissions = all_resources
            .procedures
            .values()
            .filter(|resource| regex.is_match(&resource.name))
            .map(|resource| PermissionToml {
              target: ResourceTarget::Procedure(
                resource.name.clone(),
              ),
              level: permission.level,
            });
          expanded.extend(permissions);
        }
        ResourceTargetVariant::ServerTemplate => {
          let permissions = all_resources
            .templates
            .values()
            .filter(|resource| regex.is_match(&resource.name))
            .map(|resource| PermissionToml {
              target: ResourceTarget::ServerTemplate(
                resource.name.clone(),
              ),
              level: permission.level,
            });
          expanded.extend(permissions);
        }
        ResourceTargetVariant::ResourceSync => {
          let permissions = all_resources
            .syncs
            .values()
            .filter(|resource| regex.is_match(&resource.name))
            .map(|resource| PermissionToml {
              target: ResourceTarget::ResourceSync(
                resource.name.clone(),
              ),
              level: permission.level,
            });
          expanded.extend(permissions);
        }
        ResourceTargetVariant::Stack => {
          let permissions = all_resources
            .stacks
            .values()
            .filter(|resource| regex.is_match(&resource.name))
            .map(|resource| PermissionToml {
              target: ResourceTarget::Stack(resource.name.clone()),
              level: permission.level,
            });
          expanded.extend(permissions);
        }
        ResourceTargetVariant::System => {}
      }
    } else {
      // No regex
      expanded.push(permission);
    }
  }

  Ok(expanded)
}

type AllDiff =
  HashMap<ResourceTargetVariant, (PermissionLevel, PermissionLevel)>;

/// diffs user_group.all
fn diff_group_all(
  original: &HashMap<ResourceTargetVariant, PermissionLevel>,
  incoming: &HashMap<ResourceTargetVariant, PermissionLevel>,
) -> AllDiff {
  let mut to_update = HashMap::new();

  // need to compare both forward and backward because either hashmap could be sparse.

  // forward direction
  for (variant, level) in incoming {
    let original_level = original.get(variant).unwrap_or_default();
    if level == original_level {
      continue;
    }
    to_update.insert(*variant, (*original_level, *level));
  }

  // backward direction
  for (variant, level) in original {
    let incoming_level = incoming.get(variant).unwrap_or_default();
    if level == incoming_level {
      continue;
    }
    to_update.insert(*variant, (*level, *incoming_level));
  }

  to_update
}
