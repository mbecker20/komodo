use std::collections::VecDeque;

use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
  monitor_timestamp,
  requests::write::{
    PushRecentlyViewed, PushRecentlyViewedResponse,
    SetLastSeenUpdate, SetLastSeenUpdateResponse,
  },
};
use mungos::{
  by_id::update_one_by_id,
  mongodb::bson::{doc, to_bson},
};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

const RECENTLY_VIEWED_MAX: usize = 10;

#[async_trait]
impl Resolve<PushRecentlyViewed, RequestUser> for State {
  async fn resolve(
    &self,
    PushRecentlyViewed { resource }: PushRecentlyViewed,
    user: RequestUser,
  ) -> anyhow::Result<PushRecentlyViewedResponse> {
    let mut recently_viewed = self
      .get_user(&user.id)
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
      &self.db.users,
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
impl Resolve<SetLastSeenUpdate, RequestUser> for State {
  async fn resolve(
    &self,
    SetLastSeenUpdate {}: SetLastSeenUpdate,
    user: RequestUser,
  ) -> anyhow::Result<SetLastSeenUpdateResponse> {
    update_one_by_id(
      &self.db.users,
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
