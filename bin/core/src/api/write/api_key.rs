use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::*,
  entities::{
    api_key::ApiKey,
    monitor_timestamp,
    user::{User, UserConfig},
  },
};
use mungos::{by_id::find_one_by_id, mongodb::bson::doc};
use resolver_api::Resolve;

use crate::{
  auth::random_string, helpers::query::get_user,
  state::{State, db_client},
};

const SECRET_LENGTH: usize = 40;
const BCRYPT_COST: u32 = 10;

#[async_trait]
impl Resolve<CreateApiKey, User> for State {
  #[instrument(
    name = "CreateApiKey",
    level = "debug",
    skip(self, user)
  )]
  async fn resolve(
    &self,
    CreateApiKey { name, expires }: CreateApiKey,
    user: User,
  ) -> anyhow::Result<CreateApiKeyResponse> {
    let user = get_user(&user.id).await?;

    let key = format!("K-{}", random_string(SECRET_LENGTH));
    let secret = format!("S-{}", random_string(SECRET_LENGTH));
    let secret_hash = bcrypt::hash(&secret, BCRYPT_COST)
      .context("failed at hashing secret string")?;

    let api_key = ApiKey {
      name,
      key: key.clone(),
      secret: secret_hash,
      user_id: user.id.clone(),
      created_at: monitor_timestamp(),
      expires,
    };
    db_client()
      .await
      .api_keys
      .insert_one(api_key, None)
      .await
      .context("failed to create api key on db")?;
    Ok(CreateApiKeyResponse { key, secret })
  }
}

#[async_trait]
impl Resolve<DeleteApiKey, User> for State {
  #[instrument(
    name = "DeleteApiKey",
    level = "debug",
    skip(self, user)
  )]
  async fn resolve(
    &self,
    DeleteApiKey { key }: DeleteApiKey,
    user: User,
  ) -> anyhow::Result<DeleteApiKeyResponse> {
    let client = db_client().await;
    let key = client
      .api_keys
      .find_one(doc! { "key": &key }, None)
      .await
      .context("failed at db query")?
      .context("no api key with key found")?;
    if user.id != key.user_id {
      return Err(anyhow!("api key does not belong to user"));
    }
    client
      .api_keys
      .delete_one(doc! { "key": key.key }, None)
      .await
      .context("failed to delete api key from db")?;
    Ok(DeleteApiKeyResponse {})
  }
}

#[async_trait]
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

#[async_trait]
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
      .find_one(doc! { "key": &key }, None)
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
      .delete_one(doc! { "key": key }, None)
      .await
      .context("failed to delete api key on db")?;
    Ok(DeleteApiKeyForServiceUserResponse {})
  }
}
