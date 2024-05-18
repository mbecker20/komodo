use anyhow::Context;
use monitor_client::{
  api::read::{GetTag, ListTags},
  entities::{tag::Tag, user::User},
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::{
  helpers::query::get_tag,
  state::{db_client, State},
};

impl Resolve<GetTag, User> for State {
  async fn resolve(
    &self,
    GetTag { tag }: GetTag,
    _: User,
  ) -> anyhow::Result<Tag> {
    get_tag(&tag).await
  }
}

impl Resolve<ListTags, User> for State {
  async fn resolve(
    &self,
    ListTags { query }: ListTags,
    _: User,
  ) -> anyhow::Result<Vec<Tag>> {
    find_collect(&db_client().await.tags, query, None)
      .await
      .context("failed to get tags from db")
  }
}
