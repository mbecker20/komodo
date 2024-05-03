use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::{
    CreateServiceUser, CreateServiceUserResponse, PushRecentlyViewed,
    PushRecentlyViewedResponse, SetLastSeenUpdate,
    SetLastSeenUpdateResponse, UpdateServiceUserDescription,
    UpdateServiceUserDescriptionResponse,
  },
  entities::{
    monitor_timestamp,
    user::{User, UserConfig},
  },
};
use mungos::{
  by_id::update_one_by_id,
  mongodb::bson::{doc, oid::ObjectId, to_bson},
};
use resolver_api::Resolve;

use crate::{
  helpers::query::get_user,
  state::{db_client, State},
};

const RECENTLY_VIEWED_MAX: usize = 10;

#[async_trait]
impl Resolve<PushRecentlyViewed, User> for State {
  #[instrument(name = "PushRecentlyViewed", skip(self, user))]
  async fn resolve(
    &self,
    PushRecentlyViewed { resource }: PushRecentlyViewed,
    user: User,
  ) -> anyhow::Result<PushRecentlyViewedResponse> {
    let mut recently_viewed = get_user(&user.id)
      .await?
      .recently_viewed
      .into_iter()
      .filter(|r| !resource.eq(r))
      .take(RECENTLY_VIEWED_MAX - 1)
      .collect::<VecDeque<_>>();

    recently_viewed.push_front(resource);

    let recently_viewed = to_bson(&recently_viewed)
      .context("failed to convert recently views to bson")?;

    update_one_by_id(
      &db_client().await.users,
      &user.id,
      mungos::update::Update::Set(doc! {
        "recently_viewed": recently_viewed
      }),
      None,
    )
    .await
    .context("context")?;

    Ok(PushRecentlyViewedResponse {})
  }
}

#[async_trait]
impl Resolve<SetLastSeenUpdate, User> for State {
  #[instrument(name = "SetLastSeenUpdate", skip(self, user))]
  async fn resolve(
    &self,
    SetLastSeenUpdate {}: SetLastSeenUpdate,
    user: User,
  ) -> anyhow::Result<SetLastSeenUpdateResponse> {
    update_one_by_id(
      &db_client().await.users,
      &user.id,
      mungos::update::Update::Set(doc! {
        "last_update_view": monitor_timestamp()
      }),
      None,
    )
    .await
    .context("failed to update user last_update_view")?;
    Ok(SetLastSeenUpdateResponse {})
  }
}

#[async_trait]
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
      recently_viewed: Vec::new(),
      updated_at: monitor_timestamp(),
    };
    user.id = db_client()
      .await
      .users
      .insert_one(&user, None)
      .await
      .context("failed to create service user on db")?
      .inserted_id
      .as_object_id()
      .context("inserted id is not object id")?
      .to_string();
    Ok(user)
  }
}

#[async_trait]
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
      .find_one(doc! { "username": &username }, None)
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
        None,
      )
      .await
      .context("failed to update user on db")?;
    db.users
      .find_one(doc! { "username": &username }, None)
      .await
      .context("failed to query db for user")?
      .context("user with username not found")
  }
}
