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
  helpers::query::get_resource_ids_for_non_admin,
  resource,
  state::{db_client, State},
};

#[async_trait]
impl Resolve<GetAlerter, User> for State {
  async fn resolve(
    &self,
    GetAlerter { alerter }: GetAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    resource::get_check_permissions::<Alerter>(
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
    resource::list_for_user::<Alerter>(query, &user).await
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
