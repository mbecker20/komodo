use std::str::FromStr;

use anyhow::{anyhow, Context};
use monitor_client::{
  api::{
    user::CreateApiKey,
    write::{
      CreateApiKeyForServiceUser, CreateApiKeyForServiceUserResponse,
      CreateServiceUser, CreateServiceUserResponse,
      DeleteApiKeyForServiceUser, DeleteApiKeyForServiceUserResponse,
      UpdateServiceUserDescription,
      UpdateServiceUserDescriptionResponse,
    },
  },
  entities::{
    monitor_timestamp,
    user::{User, UserConfig},
  },
};
use mungos::{
  by_id::find_one_by_id,
  mongodb::bson::{doc, oid::ObjectId},
};
use resolver_api::Resolve;

use crate::state::{db_client, State};

impl Resolve<CreateServiceUser, User> for State {
  #[instrument(name = "CreateServiceUser", skip(self, user))]
  async fn resolve(
    &self,
    CreateServiceUser {
      username,
      description,
    }: CreateServiceUser,
    user: User,
  ) -> anyhow::Result<CreateServiceUserResponse> {
    if !user.admin {
      return Err(anyhow!("user not admin"));
    }
    if ObjectId::from_str(&username).is_ok() {
      return Err(anyhow!("username cannot be valid ObjectId"));
    }
    let config = UserConfig::Service { description };
    let mut user = User {
      id: Default::default(),
      username,
      config,
      enabled: true,
      admin: false,
      create_server_permissions: false,
      create_build_permissions: false,
      last_update_view: 0,
      recents: Default::default(),
      all: Default::default(),
      updated_at: monitor_timestamp(),
    };
    user.id = db_client()
      .await
      .users
      .insert_one(&user)
      .await
      .context("failed to create service user on db")?
      .inserted_id
      .as_object_id()
      .context("inserted id is not object id")?
      .to_string();
    Ok(user)
  }
}

impl Resolve<UpdateServiceUserDescription, User> for State {
  #[instrument(
    name = "UpdateServiceUserDescription",
    skip(self, user)
  )]
  async fn resolve(
    &self,
    UpdateServiceUserDescription {
      username,
      description,
    }: UpdateServiceUserDescription,
    user: User,
  ) -> anyhow::Result<UpdateServiceUserDescriptionResponse> {
    if !user.admin {
      return Err(anyhow!("user not admin"));
    }
    let db = db_client().await;
    let service_user = db
      .users
      .find_one(doc! { "username": &username })
      .await
      .context("failed to query db for user")?
      .context("no user with given username")?;
    let UserConfig::Service { .. } = &service_user.config else {
      return Err(anyhow!("user is not service user"));
    };
    db.users
      .update_one(
        doc! { "username": &username },
        doc! { "$set": { "config.data.description": description } },
      )
      .await
      .context("failed to update user on db")?;
    db.users
      .find_one(doc! { "username": &username })
      .await
      .context("failed to query db for user")?
      .context("user with username not found")
  }
}

impl Resolve<CreateApiKeyForServiceUser, User> for State {
  #[instrument(name = "CreateApiKeyForServiceUser", skip(self, user))]
  async fn resolve(
    &self,
    CreateApiKeyForServiceUser {
      user_id,
      name,
      expires,
    }: CreateApiKeyForServiceUser,
    user: User,
  ) -> anyhow::Result<CreateApiKeyForServiceUserResponse> {
    if !user.admin {
      return Err(anyhow!("user not admin"));
    }
    let service_user =
      find_one_by_id(&db_client().await.users, &user_id)
        .await
        .context("failed to query db for user")?
        .context("no user found with id")?;
    let UserConfig::Service { .. } = &service_user.config else {
      return Err(anyhow!("user is not service user"));
    };
    self
      .resolve(CreateApiKey { name, expires }, service_user)
      .await
  }
}

impl Resolve<DeleteApiKeyForServiceUser, User> for State {
  #[instrument(name = "DeleteApiKeyForServiceUser", skip(self, user))]
  async fn resolve(
    &self,
    DeleteApiKeyForServiceUser { key }: DeleteApiKeyForServiceUser,
    user: User,
  ) -> anyhow::Result<DeleteApiKeyForServiceUserResponse> {
    if !user.admin {
      return Err(anyhow!("user not admin"));
    }
    let db = db_client().await;
    let api_key = db
      .api_keys
      .find_one(doc! { "key": &key })
      .await
      .context("failed to query db for api key")?
      .context("did not find matching api key")?;
    let service_user =
      find_one_by_id(&db_client().await.users, &api_key.user_id)
        .await
        .context("failed to query db for user")?
        .context("no user found with id")?;
    let UserConfig::Service { .. } = &service_user.config else {
      return Err(anyhow!("user is not service user"));
    };
    db.api_keys
      .delete_one(doc! { "key": key })
      .await
      .context("failed to delete api key on db")?;
    Ok(DeleteApiKeyForServiceUserResponse {})
  }
}
