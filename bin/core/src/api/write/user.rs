use std::str::FromStr;

use anyhow::{anyhow, Context};
use komodo_client::{
  api::write::{
    DeleteUser, DeleteUserResponse, UpdateUserPassword,
    UpdateUserPasswordResponse, UpdateUserUsername,
    UpdateUserUsernameResponse,
  },
  entities::{
    user::{User, UserConfig},
    NoData,
  },
};
use mungos::mongodb::bson::{doc, oid::ObjectId};
use resolver_api::Resolve;

use crate::{
  helpers::hash_password,
  state::{db_client, State},
};

//

impl Resolve<UpdateUserUsername, User> for State {
  async fn resolve(
    &self,
    UpdateUserUsername { username }: UpdateUserUsername,
    user: User,
  ) -> anyhow::Result<UpdateUserUsernameResponse> {
    if username.is_empty() {
      return Err(anyhow!("Username cannot be empty."));
    }
    let db = db_client();
    if db
      .users
      .find_one(doc! { "username": &username })
      .await
      .context("Failed to query for existing users")?
      .is_some()
    {
      return Err(anyhow!("Username already taken."));
    }
    let id = ObjectId::from_str(&user.id)
      .context("User id not valid ObjectId.")?;
    db.users
      .update_one(
        doc! { "_id": id },
        doc! { "$set": { "username": username } },
      )
      .await
      .context("Failed to update user username on database.")?;
    Ok(NoData {})
  }
}

//

impl Resolve<UpdateUserPassword, User> for State {
  async fn resolve(
    &self,
    UpdateUserPassword { password }: UpdateUserPassword,
    user: User,
  ) -> anyhow::Result<UpdateUserPasswordResponse> {
    let UserConfig::Local { .. } = user.config else {
      return Err(anyhow!("User is not local user"));
    };
    if password.is_empty() {
      return Err(anyhow!("Password cannot be empty."));
    }
    let id = ObjectId::from_str(&user.id)
      .context("User id not valid ObjectId.")?;
    let hashed_password = hash_password(password)?;
    db_client()
      .users
      .update_one(
        doc! { "_id": id },
        doc! { "$set": {
          "config.data.password": hashed_password
        } },
      )
      .await
      .context("Failed to update user password on database.")?;
    Ok(NoData {})
  }
}

//

impl Resolve<DeleteUser, User> for State {
  async fn resolve(
    &self,
    DeleteUser { user }: DeleteUser,
    admin: User,
  ) -> anyhow::Result<DeleteUserResponse> {
    if !admin.admin {
      return Err(anyhow!("Calling user is not admin."));
    }
    if admin.username == user || admin.id == user {
      return Err(anyhow!("User cannot delete themselves."));
    }
    let query = if let Ok(id) = ObjectId::from_str(&user) {
      doc! { "_id": id }
    } else {
      doc! { "username": user }
    };
    let db = db_client();
    let Some(user) = db
      .users
      .find_one(query.clone())
      .await
      .context("Failed to query database for users.")?
    else {
      return Err(anyhow!("No user found with given id / username"));
    };
    if user.super_admin {
      return Err(anyhow!("Cannot delete a super admin user."));
    }
    if user.admin && !admin.super_admin {
      return Err(anyhow!(
        "Only a Super Admin can delete an admin user."
      ));
    }
    db.users
      .delete_one(query)
      .await
      .context("Failed to delete user from database")?;
    Ok(user)
  }
}
