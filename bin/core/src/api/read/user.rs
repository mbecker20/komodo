use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::read::{
    GetUser, GetUsername, GetUsernameResponse, GetUsers, ListApiKeys,
    ListApiKeysResponse,
  },
  entities::user::User,
};
use mungos::{
  by_id::find_one_by_id, find::find_collect, mongodb::bson::doc,
};
use resolver_api::Resolve;

use crate::{auth::RequestUser, db_client, state::State};

#[async_trait]
impl Resolve<GetUser, RequestUser> for State {
  async fn resolve(
    &self,
    GetUser {}: GetUser,
    user: RequestUser,
  ) -> anyhow::Result<User> {
    let mut user = find_one_by_id(&db_client().await.users, &user.id)
      .await
      .context("failed at mongo query")?
      .context("no user found with id")?;
    user.sanitize();
    Ok(user)
  }
}

#[async_trait]
impl Resolve<GetUsername, RequestUser> for State {
  async fn resolve(
    &self,
    GetUsername { user_id }: GetUsername,
    _: RequestUser,
  ) -> anyhow::Result<GetUsernameResponse> {
    let user = find_one_by_id(&db_client().await.users, &user_id)
      .await
      .context("failed at mongo query for user")?
      .context("no user found with id")?;

    Ok(GetUsernameResponse {
      username: user.username,
    })
  }
}

#[async_trait]
impl Resolve<GetUsers, RequestUser> for State {
  async fn resolve(
    &self,
    GetUsers {}: GetUsers,
    user: RequestUser,
  ) -> anyhow::Result<Vec<User>> {
    if !user.is_admin {
      return Err(anyhow!("this route is only accessable by admins"));
    }
    let mut users =
      find_collect(&db_client().await.users, None, None)
        .await
        .context("failed to pull users from db")?;
    users.iter_mut().for_each(|user| user.sanitize());
    Ok(users)
  }
}

#[async_trait]
impl Resolve<ListApiKeys, RequestUser> for State {
  async fn resolve(
    &self,
    ListApiKeys {}: ListApiKeys,
    user: RequestUser,
  ) -> anyhow::Result<ListApiKeysResponse> {
    let api_keys = find_collect(
      &db_client().await.api_keys,
      doc! { "user_id": &user.id },
      None,
    )
    .await
    .context("failed to query db for api keys")?
    .into_iter()
    .map(|mut api_keys| {
      api_keys.sanitize();
      api_keys
    })
    .collect();
    Ok(api_keys)
  }
}
