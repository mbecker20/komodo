use std::{collections::VecDeque, time::Instant};

use anyhow::{anyhow, Context};
use axum::{middleware, routing::post, Extension, Json, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use komodo_client::{
  api::user::{
    CreateApiKey, CreateApiKeyResponse, DeleteApiKey,
    DeleteApiKeyResponse, PushRecentlyViewed,
    PushRecentlyViewedResponse, SetLastSeenUpdate,
    SetLastSeenUpdateResponse,
  },
  entities::{api_key::ApiKey, komodo_timestamp, user::User},
};
use mongo_indexed::doc;
use mungos::{by_id::update_one_by_id, mongodb::bson::to_bson};
use resolver_api::{derive::Resolver, Resolve, Resolver};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
  auth::auth_request,
  helpers::{query::get_user, random_string},
  state::{db_client, State},
};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(User)]
#[serde(tag = "type", content = "params")]
enum UserRequest {
  PushRecentlyViewed(PushRecentlyViewed),
  SetLastSeenUpdate(SetLastSeenUpdate),
  CreateApiKey(CreateApiKey),
  DeleteApiKey(DeleteApiKey),
}

pub fn router() -> Router {
  Router::new()
    .route("/", post(handler))
    .layer(middleware::from_fn(auth_request))
}

#[instrument(name = "UserHandler", level = "debug", skip(user))]
async fn handler(
  Extension(user): Extension<User>,
  Json(request): Json<UserRequest>,
) -> serror::Result<(TypedHeader<ContentType>, String)> {
  let timer = Instant::now();
  let req_id = Uuid::new_v4();
  debug!(
    "/user request {req_id} | user: {} ({})",
    user.username, user.id
  );
  let res =
    State
      .resolve_request(request, user)
      .await
      .map_err(|e| match e {
        resolver_api::Error::Serialization(e) => {
          anyhow!("{e:?}").context("response serialization error")
        }
        resolver_api::Error::Inner(e) => e,
      });
  if let Err(e) = &res {
    warn!("/user request {req_id} error: {e:#}");
  }
  let elapsed = timer.elapsed();
  debug!("/user request {req_id} | resolve time: {elapsed:?}");
  Ok((TypedHeader(ContentType::json()), res?))
}

const RECENTLY_VIEWED_MAX: usize = 10;

impl Resolve<PushRecentlyViewed, User> for State {
  #[instrument(
    name = "PushRecentlyViewed",
    level = "debug",
    skip(self, user)
  )]
  async fn resolve(
    &self,
    PushRecentlyViewed { resource }: PushRecentlyViewed,
    user: User,
  ) -> anyhow::Result<PushRecentlyViewedResponse> {
    let user = get_user(&user.id).await?;

    let (resource_type, id) = resource.extract_variant_id();
    let update = match user.recents.get(&resource_type) {
      Some(recents) => {
        let mut recents = recents
          .iter()
          .filter(|_id| !id.eq(*_id))
          .take(RECENTLY_VIEWED_MAX - 1)
          .collect::<VecDeque<_>>();
        recents.push_front(id);
        doc! { format!("recents.{resource_type}"): to_bson(&recents)? }
      }
      None => {
        doc! { format!("recents.{resource_type}"): [id] }
      }
    };
    update_one_by_id(
      &db_client().users,
      &user.id,
      mungos::update::Update::Set(update),
      None,
    )
    .await
    .with_context(|| {
      format!("failed to update recents.{resource_type}")
    })?;

    Ok(PushRecentlyViewedResponse {})
  }
}

impl Resolve<SetLastSeenUpdate, User> for State {
  #[instrument(
    name = "SetLastSeenUpdate",
    level = "debug",
    skip(self, user)
  )]
  async fn resolve(
    &self,
    SetLastSeenUpdate {}: SetLastSeenUpdate,
    user: User,
  ) -> anyhow::Result<SetLastSeenUpdateResponse> {
    update_one_by_id(
      &db_client().users,
      &user.id,
      mungos::update::Update::Set(doc! {
        "last_update_view": komodo_timestamp()
      }),
      None,
    )
    .await
    .context("failed to update user last_update_view")?;
    Ok(SetLastSeenUpdateResponse {})
  }
}

const SECRET_LENGTH: usize = 40;
const BCRYPT_COST: u32 = 10;

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
      created_at: komodo_timestamp(),
      expires,
    };
    db_client()
      .api_keys
      .insert_one(api_key)
      .await
      .context("failed to create api key on db")?;
    Ok(CreateApiKeyResponse { key, secret })
  }
}

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
    let client = db_client();
    let key = client
      .api_keys
      .find_one(doc! { "key": &key })
      .await
      .context("failed at db query")?
      .context("no api key with key found")?;
    if user.id != key.user_id {
      return Err(anyhow!("api key does not belong to user"));
    }
    client
      .api_keys
      .delete_one(doc! { "key": key.key })
      .await
      .context("failed to delete api key from db")?;
    Ok(DeleteApiKeyResponse {})
  }
}
