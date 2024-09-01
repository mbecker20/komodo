use anyhow::Context;
use mongo_indexed::Document;
use komodo_client::{
  api::read::*,
  entities::{
    alerter::{Alerter, AlerterListItem},
    permission::PermissionLevel,
    user::User,
  },
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{
  resource,
  state::{db_client, State},
};

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

impl Resolve<ListAlerters, User> for State {
  async fn resolve(
    &self,
    ListAlerters { query }: ListAlerters,
    user: User,
  ) -> anyhow::Result<Vec<AlerterListItem>> {
    resource::list_for_user::<Alerter>(query, &user).await
  }
}

impl Resolve<ListFullAlerters, User> for State {
  async fn resolve(
    &self,
    ListFullAlerters { query }: ListFullAlerters,
    user: User,
  ) -> anyhow::Result<ListFullAlertersResponse> {
    resource::list_full_for_user::<Alerter>(query, &user).await
  }
}

impl Resolve<GetAlertersSummary, User> for State {
  async fn resolve(
    &self,
    GetAlertersSummary {}: GetAlertersSummary,
    user: User,
  ) -> anyhow::Result<GetAlertersSummaryResponse> {
    let query =
      match resource::get_resource_ids_for_user::<Alerter>(&user)
        .await?
      {
        Some(ids) => doc! {
          "_id": { "$in": ids }
        },
        None => Document::new(),
      };
    let total = db_client()
      .await
      .alerters
      .count_documents(query)
      .await
      .context("failed to count all alerter documents")?;
    let res = GetAlertersSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}
