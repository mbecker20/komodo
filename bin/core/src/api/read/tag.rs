use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::{GetTag, ListTags},
  entities::tag::CustomTag,
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::{
  auth::RequestUser, db_client, helpers::get_tag, state::State,
};

#[async_trait]
impl Resolve<GetTag, RequestUser> for State {
  async fn resolve(
    &self,
    GetTag { id }: GetTag,
    _: RequestUser,
  ) -> anyhow::Result<CustomTag> {
    get_tag(&id).await
  }
}

#[async_trait]
impl Resolve<ListTags, RequestUser> for State {
  async fn resolve(
    &self,
    ListTags { query }: ListTags,
    _: RequestUser,
  ) -> anyhow::Result<Vec<CustomTag>> {
    find_collect(&db_client().await.tags, query, None)
      .await
      .context("failed to get tags from db")
  }
}
