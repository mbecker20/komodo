use anyhow::Context;
use komodo_client::{
  api::read::*,
  entities::{
    permission::PermissionLevel, server_template::ServerTemplate,
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

impl Resolve<GetServerTemplate, User> for State {
  async fn resolve(
    &self,
    GetServerTemplate { server_template }: GetServerTemplate,
    user: User,
  ) -> anyhow::Result<GetServerTemplateResponse> {
    resource::get_check_permissions::<ServerTemplate>(
      &server_template,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

impl Resolve<ListServerTemplates, User> for State {
  async fn resolve(
    &self,
    ListServerTemplates { query }: ListServerTemplates,
    user: User,
  ) -> anyhow::Result<ListServerTemplatesResponse> {
    let all_tags = if query.tags.is_empty() {
      vec![]
    } else {
      get_all_tags(None).await?
    };
    resource::list_for_user::<ServerTemplate>(query, &user, &all_tags)
      .await
  }
}

impl Resolve<ListFullServerTemplates, User> for State {
  async fn resolve(
    &self,
    ListFullServerTemplates { query }: ListFullServerTemplates,
    user: User,
  ) -> anyhow::Result<ListFullServerTemplatesResponse> {
    let all_tags = if query.tags.is_empty() {
      vec![]
    } else {
      get_all_tags(None).await?
    };
    resource::list_full_for_user::<ServerTemplate>(
      query, &user, &all_tags,
    )
    .await
  }
}

impl Resolve<GetServerTemplatesSummary, User> for State {
  async fn resolve(
    &self,
    GetServerTemplatesSummary {}: GetServerTemplatesSummary,
    user: User,
  ) -> anyhow::Result<GetServerTemplatesSummaryResponse> {
    let query = match resource::get_resource_ids_for_user::<
      ServerTemplate,
    >(&user)
    .await?
    {
      Some(ids) => doc! {
        "_id": { "$in": ids }
      },
      None => Document::new(),
    };
    let total = db_client()
      .server_templates
      .count_documents(query)
      .await
      .context("failed to count all server template documents")?;
    let res = GetServerTemplatesSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}
