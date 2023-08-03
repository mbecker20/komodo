use std::collections::VecDeque;

use anyhow::Context;
use async_trait::async_trait;
use monitor_types::requests::write::{PushRecentlyViewed, PushRecentlyViewedResponse};
use mungos::mongodb::bson::{doc, to_bson};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<PushRecentlyViewed, RequestUser> for State {
    async fn resolve(
        &self,
        PushRecentlyViewed { resource }: PushRecentlyViewed,
        user: RequestUser,
    ) -> anyhow::Result<PushRecentlyViewedResponse> {
        let mut recently_viewed = self
            .db
            .users
            .find_one_by_id(&user.id)
            .await
            .context("failed at mongo query")?
            .context("no user found with id")?
            .recently_viewed
            .into_iter()
            .filter(|r| !resource.eq(r))
            .collect::<VecDeque<_>>();

        recently_viewed.push_front(resource);

        let recently_viewed =
            to_bson(&recently_viewed).context("failed to convert recently views to bson")?;

        self.db
            .users
            .update_one(
                &user.id,
                mungos::Update::Set(doc! {
                    "recently_viewed": recently_viewed
                }),
            )
            .await
            .context("context")?;

        Ok(PushRecentlyViewedResponse {})
    }
}
