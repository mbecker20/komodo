use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::*,
  entities::{api_key::ApiKey, monitor_timestamp},
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{
  auth::{random_string, RequestUser},
  db_client,
  helpers::get_user,
  state::State,
};

const SECRET_LENGTH: usize = 40;
const BCRYPT_COST: u32 = 10;

#[async_trait]
impl Resolve<CreateApiKey, RequestUser> for State {
  async fn resolve(
    &self,
    CreateApiKey { name, expires }: CreateApiKey,
    user: RequestUser,
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
impl Resolve<DeleteApiKey, RequestUser> for State {
  async fn resolve(
    &self,
    DeleteApiKey { key }: DeleteApiKey,
    user: RequestUser,
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
