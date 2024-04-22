use monitor_client::entities::user_group::UserGroup;

use crate::maps::name_to_user_group;

struct UserGroupToUpdate {
  pub old: UserGroup,
  pub new: UserGroup,
}

async fn get_updates(user_groups: Vec<UserGroup>) {
  let map = name_to_user_group();

  let mut to_update = Vec::<UserGroupToUpdate>::new();
  let mut to_create = Vec::<UserGroup>::new();

  for user_group in user_groups {
    match map.get(&user_group.name).cloned() {
      Some(old) => to_update.push(UserGroupToUpdate {
        old,
        new: user_group,
      }),
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
        .map(|item| item.new.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    );
  }
}

async fn run_updates(
  to_update: Vec<UserGroupToUpdate>,
  to_create: Vec<UserGroup>,
) {
  let log_after = !to_update.is_empty() || !to_create.is_empty();

  for user_group in to_create {
    
  }

  for UserGroupToUpdate { old, new } in to_update {
    //
  }
}
