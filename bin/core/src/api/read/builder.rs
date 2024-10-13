use anyhow::Context;
use komodo_client::{
  api::read::*,
  entities::{
    builder::{Builder, BuilderListItem},
    permission::PermissionLevel,
    user::User,
  },
};
use mongo_indexed::Document;
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{
  helpers::query::get_all_tags,
  resource,
  state::{db_client, State},
};

impl Resolve<GetBuilder, User> for State {
  async fn resolve(
    &self,
    GetBuilder { builder }: GetBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    resource::get_check_permissions::<Builder>(
      &builder,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

impl Resolve<ListBuilders, User> for State {
  async fn resolve(
    &self,
    ListBuilders { query }: ListBuilders,
    user: User,
  ) -> anyhow::Result<Vec<BuilderListItem>> {
    let all_tags = if query.tags.is_empty() {
      vec![]
    } else {
      get_all_tags(None).await?
    };
    resource::list_for_user::<Builder>(query, &user, &all_tags).await
  }
}

impl Resolve<ListFullBuilders, User> for State {
  async fn resolve(
    &self,
    ListFullBuilders { query }: ListFullBuilders,
    user: User,
  ) -> anyhow::Result<ListFullBuildersResponse> {
    let all_tags = if query.tags.is_empty() {
      vec![]
    } else {
      get_all_tags(None).await?
    };
    resource::list_full_for_user::<Builder>(query, &user, &all_tags)
      .await
  }
}

impl Resolve<GetBuildersSummary, User> for State {
  async fn resolve(
    &self,
    GetBuildersSummary {}: GetBuildersSummary,
    user: User,
  ) -> anyhow::Result<GetBuildersSummaryResponse> {
    let query =
      match resource::get_resource_ids_for_user::<Builder>(&user)
        .await?
      {
        Some(ids) => doc! {
          "_id": { "$in": ids }
        },
        None => Document::new(),
      };
    let total = db_client()
      .builders
      .count_documents(query)
      .await
      .context("failed to count all builder documents")?;
    let res = GetBuildersSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}
