use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::*,
  entities::{
    alerter::{Alerter, AlerterListItem},
    resource::AddFilters,
    user::User,
    PermissionLevel,
  },
};
use mungos::mongodb::bson::{doc, Document};
use resolver_api::Resolve;

use crate::{
  db::db_client, helpers::resource::StateResource, state::State,
};

#[async_trait]
impl Resolve<GetAlerter, User> for State {
  async fn resolve(
    &self,
    GetAlerter { id }: GetAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Read,
      )
      .await
  }
}

#[async_trait]
impl Resolve<ListAlerters, User> for State {
  async fn resolve(
    &self,
    ListAlerters { query }: ListAlerters,
    user: User,
  ) -> anyhow::Result<Vec<AlerterListItem>> {
    let mut filters = Document::new();
    query.add_filters(&mut filters);
    <State as StateResource<Alerter>>::list_resources_for_user(
      self, filters, &user,
    )
    .await
  }
}

#[async_trait]
impl Resolve<GetAlertersSummary, User> for State {
  async fn resolve(
    &self,
    GetAlertersSummary {}: GetAlertersSummary,
    user: User,
  ) -> anyhow::Result<GetAlertersSummaryResponse> {
    let query = if user.admin {
      None
    } else {
      let query = doc! {
          format!("permissions.{}", user.id): { "$in": ["read", "execute", "update"] }
      };
      Some(query)
    };
    let total = db_client()
      .await
      .alerters
      .count_documents(query, None)
      .await
      .context("failed to count all alerter documents")?;
    let res = GetAlertersSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}
