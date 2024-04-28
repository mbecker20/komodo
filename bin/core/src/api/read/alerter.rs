use std::str::FromStr;

use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::*,
  entities::{
    alerter::{Alerter, AlerterListItem},
    permission::PermissionLevel,
    update::ResourceTargetVariant,
    user::User,
  },
};
use mungos::mongodb::bson::{doc, oid::ObjectId};
use resolver_api::Resolve;

use crate::{
  helpers::resource::{
    get_resource_ids_for_non_admin, StateResource,
  },
  state::{db_client, State},
};

#[async_trait]
impl Resolve<GetAlerter, User> for State {
  async fn resolve(
    &self,
    GetAlerter { alerter }: GetAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    Alerter::get_resource_check_permissions(
      &alerter,
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
    Alerter::list_resources_for_user(query, &user).await
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
      let ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Alerter,
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
