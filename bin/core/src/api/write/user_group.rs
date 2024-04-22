use std::str::FromStr;

use anyhow::{anyhow, Context};
use axum::async_trait;
use monitor_client::{
  api::write::{
    AddUserToUserGroup, CreateUserGroup, DeleteUserGroup,
    RemoveUserFromUserGroup, RenameUserGroup,
  },
  entities::{monitor_timestamp, user::User, user_group::UserGroup},
};
use mungos::{
  by_id::{delete_one_by_id, find_one_by_id, update_one_by_id},
  mongodb::bson::{doc, oid::ObjectId},
};
use resolver_api::Resolve;

use crate::{db::db_client, state::State};

#[async_trait]
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

#[async_trait]
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

#[async_trait]
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

#[async_trait]
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
        doc! { "$push": { "users": &user.id } },
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

#[async_trait]
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
