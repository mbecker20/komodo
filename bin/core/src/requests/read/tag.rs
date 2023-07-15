use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::tag::CustomTag,
    requests::read::{GetTag, ListTags},
};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<GetTag, RequestUser> for State {
    async fn resolve(&self, GetTag { id }: GetTag, _: RequestUser) -> anyhow::Result<CustomTag> {
        self.get_tag(&id).await
    }
}

#[async_trait]
impl Resolve<ListTags, RequestUser> for State {
    async fn resolve(
        &self,
        ListTags { query }: ListTags,
        _: RequestUser,
    ) -> anyhow::Result<Vec<CustomTag>> {
        self.db
            .tags
            .get_some(query, None)
            .await
            .context("failed to get tags from db")
    }
}
