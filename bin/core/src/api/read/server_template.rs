use anyhow::Context;
use mongo_indexed::Document;
use monitor_client::{
  api::read::*,
  entities::{
    permission::PermissionLevel, server_template::ServerTemplate,
    update::ResourceTargetVariant, user::User,
  },
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{
  helpers::query::get_resource_ids_for_user,
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
    resource::list_for_user::<ServerTemplate>(query, &user).await
  }
}

impl Resolve<ListFullServerTemplates, User> for State {
  async fn resolve(
    &self,
    ListFullServerTemplates { query }: ListFullServerTemplates,
    user: User,
  ) -> anyhow::Result<ListFullServerTemplatesResponse> {
    resource::list_full_for_user::<ServerTemplate>(query, &user).await
  }
}

impl Resolve<GetServerTemplatesSummary, User> for State {
  async fn resolve(
    &self,
    GetServerTemplatesSummary {}: GetServerTemplatesSummary,
    user: User,
  ) -> anyhow::Result<GetServerTemplatesSummaryResponse> {
    let query = match get_resource_ids_for_user(
      &user,
      ResourceTargetVariant::ServerTemplate,
    )
    .await?
    {
      Some(ids) => doc! {
        "_id": { "$in": ids }
      },
      None => Document::new(),
    };
    let total = db_client()
      .await
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
