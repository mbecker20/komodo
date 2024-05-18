use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context};
use monitor_client::{
  api::write::{
    AddUserToUserGroup, CreateUserGroup, DeleteUserGroup,
    RemoveUserFromUserGroup, RenameUserGroup, SetUsersInUserGroup,
  },
  entities::{monitor_timestamp, user::User, user_group::UserGroup},
};
use mungos::{
  by_id::{delete_one_by_id, find_one_by_id, update_one_by_id},
  find::find_collect,
  mongodb::bson::{doc, oid::ObjectId},
};
use resolver_api::Resolve;

use crate::state::{db_client, State};

impl Resolve<CreateUserGroup, User> for State {
  async fn resolve(
    &self,
    CreateUserGroup { name }: CreateUserGroup,
    admin: User,
  ) -> anyhow::Result<UserGroup> {
    if !admin.admin {
      return Err(anyhow!("This call is admin-only"));
    }
    let user_group = UserGroup {
      id: Default::default(),
      users: Default::default(),
      updated_at: monitor_timestamp(),
      name,
    };
    let db = db_client().await;
    let id = db
      .user_groups
      .insert_one(user_group, None)
      .await
      .context("failed to create UserGroup on db")?
      .inserted_id
      .as_object_id()
      .context("inserted id is not ObjectId")?
      .to_string();
    find_one_by_id(&db.user_groups, &id)
      .await
      .context("failed to query db for user groups")?
      .context("user group at id not found")
  }
}

impl Resolve<RenameUserGroup, User> for State {
  async fn resolve(
    &self,
    RenameUserGroup { id, name }: RenameUserGroup,
    admin: User,
  ) -> anyhow::Result<UserGroup> {
    if !admin.admin {
      return Err(anyhow!("This call is admin-only"));
    }
    let db = db_client().await;
    update_one_by_id(
      &db.user_groups,
      &id,
      doc! { "$set": { "name": name } },
      None,
    )
    .await
    .context("failed to rename UserGroup on db")?;
    find_one_by_id(&db.user_groups, &id)
      .await
      .context("failed to query db for UserGroups")?
      .context("no user group with given id")
  }
}

impl Resolve<DeleteUserGroup, User> for State {
  async fn resolve(
    &self,
    DeleteUserGroup { id }: DeleteUserGroup,
    admin: User,
  ) -> anyhow::Result<UserGroup> {
    if !admin.admin {
      return Err(anyhow!("This call is admin-only"));
    }

    let db = db_client().await;

    let ug = find_one_by_id(&db.user_groups, &id)
      .await
      .context("failed to query db for UserGroups")?
      .context("no UserGroup found with given id")?;

    delete_one_by_id(&db.user_groups, &id, None)
      .await
      .context("failed to delete UserGroup from db")?;

    db.permissions
      .delete_many(doc! {
        "user_target.type": "UserGroup",
        "user_target.id": id,
      }, None)
      .await
      .context("failed to clean up UserGroups permissions. User Group has been deleted")?;

    Ok(ug)
  }
}

impl Resolve<AddUserToUserGroup, User> for State {
  async fn resolve(
    &self,
    AddUserToUserGroup { user_group, user }: AddUserToUserGroup,
    admin: User,
  ) -> anyhow::Result<UserGroup> {
    if !admin.admin {
      return Err(anyhow!("This call is admin-only"));
    }

    let db = db_client().await;

    let filter = match ObjectId::from_str(&user) {
      Ok(id) => doc! { "_id": id },
      Err(_) => doc! { "username": &user },
    };
    let user = db
      .users
      .find_one(filter, None)
      .await
      .context("failed to query mongo for users")?
      .context("no matching user found")?;

    let filter = match ObjectId::from_str(&user_group) {
      Ok(id) => doc! { "_id": id },
      Err(_) => doc! { "name": &user_group },
    };
    db.user_groups
      .update_one(
        filter.clone(),
        doc! { "$addToSet": { "users": &user.id } },
        None,
      )
      .await
      .context("failed to add user to group on db")?;
    db.user_groups
      .find_one(filter, None)
      .await
      .context("failed to query db for UserGroups")?
      .context("no user group with given id")
  }
}

impl Resolve<RemoveUserFromUserGroup, User> for State {
  async fn resolve(
    &self,
    RemoveUserFromUserGroup {
      user_group,
      user,
    }: RemoveUserFromUserGroup,
    admin: User,
  ) -> anyhow::Result<UserGroup> {
    if !admin.admin {
      return Err(anyhow!("This call is admin-only"));
    }

    let db = db_client().await;

    let filter = match ObjectId::from_str(&user) {
      Ok(id) => doc! { "_id": id },
      Err(_) => doc! { "username": &user },
    };
    let user = db
      .users
      .find_one(filter, None)
      .await
      .context("failed to query mongo for users")?
      .context("no matching user found")?;

    let filter = match ObjectId::from_str(&user_group) {
      Ok(id) => doc! { "_id": id },
      Err(_) => doc! { "name": &user_group },
    };
    db.user_groups
      .update_one(
        filter.clone(),
        doc! { "$pull": { "users": &user.id } },
        None,
      )
      .await
      .context("failed to add user to group on db")?;
    db.user_groups
      .find_one(filter, None)
      .await
      .context("failed to query db for UserGroups")?
      .context("no user group with given id")
  }
}

impl Resolve<SetUsersInUserGroup, User> for State {
  async fn resolve(
    &self,
    SetUsersInUserGroup { user_group, users }: SetUsersInUserGroup,
    admin: User,
  ) -> anyhow::Result<UserGroup> {
    if !admin.admin {
      return Err(anyhow!("This call is admin-only"));
    }

    let db = db_client().await;

    let all_users = find_collect(&db.users, None, None)
      .await
      .context("failed to query db for users")?
      .into_iter()
      .map(|u| (u.username, u.id))
      .collect::<HashMap<_, _>>();

    // Make sure all users are user ids
    let users = users
      .into_iter()
      .filter_map(|user| match ObjectId::from_str(&user) {
        Ok(_) => Some(user),
        Err(_) => all_users.get(&user).cloned(),
      })
      .collect::<Vec<_>>();

    let filter = match ObjectId::from_str(&user_group) {
      Ok(id) => doc! { "_id": id },
      Err(_) => doc! { "name": &user_group },
    };
    db.user_groups
      .update_one(
        filter.clone(),
        doc! { "$set": { "users": users } },
        None,
      )
      .await
      .context("failed to add user to group on db")?;
    db.user_groups
      .find_one(filter, None)
      .await
      .context("failed to query db for UserGroups")?
      .context("no user group with given id")
  }
}
