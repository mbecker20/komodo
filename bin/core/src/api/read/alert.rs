use anyhow::Context;
use komodo_client::{
  api::read::{
    GetAlert, GetAlertResponse, ListAlerts, ListAlertsResponse,
  },
  entities::{deployment::Deployment, server::Server, user::User},
};
use mungos::{
  by_id::find_one_by_id,
  find::find_collect,
  mongodb::{bson::doc, options::FindOptions},
};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  resource::get_resource_ids_for_user,
  state::{db_client, State},
};

const NUM_ALERTS_PER_PAGE: u64 = 100;

impl Resolve<ListAlerts, User> for State {
  async fn resolve(
    &self,
    ListAlerts { query, page }: ListAlerts,
    user: User,
  ) -> anyhow::Result<ListAlertsResponse> {
    let mut query = query.unwrap_or_default();
    if !user.admin && !core_config().transparent_mode {
      let server_ids =
        get_resource_ids_for_user::<Server>(&user).await?;
      let deployment_ids =
        get_resource_ids_for_user::<Deployment>(&user).await?;
      query.extend(doc! {
        "$or": [
          { "target.type": "Server", "target.id": { "$in": &server_ids } },
          { "target.type": "Deployment", "target.id": { "$in": &deployment_ids } },
        ]
      });
    }

    let alerts = find_collect(
      &db_client().await.alerts,
      query,
      FindOptions::builder()
        .sort(doc! { "ts": -1 })
        .limit(NUM_ALERTS_PER_PAGE as i64)
        .skip(page * NUM_ALERTS_PER_PAGE)
        .build(),
    )
    .await
    .context("failed to get alerts from db")?;

    let next_page = if alerts.len() < NUM_ALERTS_PER_PAGE as usize {
      None
    } else {
      Some((page + 1) as i64)
    };

    let res = ListAlertsResponse { next_page, alerts };

    Ok(res)
  }
}

impl Resolve<GetAlert, User> for State {
  async fn resolve(
    &self,
    GetAlert { id }: GetAlert,
    _: User,
  ) -> anyhow::Result<GetAlertResponse> {
    find_one_by_id(&db_client().await.alerts, &id)
      .await
      .context("failed to query db for alert")?
      .context("no alert found with given id")
  }
}
