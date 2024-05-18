use std::str::FromStr;

use anyhow::Context;
use monitor_client::{
  api::read::{
    GetServerTemplate, GetServerTemplateResponse,
    GetServerTemplatesSummary, GetServerTemplatesSummaryResponse,
    ListServerTemplates, ListServerTemplatesResponse,
  },
  entities::{
    permission::PermissionLevel, server_template::ServerTemplate,
    update::ResourceTargetVariant, user::User,
  },
};
use mungos::mongodb::bson::{doc, oid::ObjectId};
use resolver_api::Resolve;

use crate::{
  helpers::query::get_resource_ids_for_non_admin, resource, state::{db_client, State}
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

impl Resolve<GetServerTemplatesSummary, User> for State {
  async fn resolve(
    &self,
    GetServerTemplatesSummary {}: GetServerTemplatesSummary,
    user: User,
  ) -> anyhow::Result<GetServerTemplatesSummaryResponse> {
    let query = if user.admin {
      None
    } else {
      let ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::ServerTemplate,
      )
      .await?
      .into_iter()
      .flat_map(|id| ObjectId::from_str(&id))
      .collect::<Vec<_>>();
      let query = doc! {
        "_id": { "$in": ids }
      };
      Some(query)
    };
    let total = db_client()
      .await
      .builders
      .count_documents(query, None)
      .await
      .context("failed to count all builder documents")?;
    let res = GetServerTemplatesSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}
