use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  entities::{deployment::Deployment, server::Server},
  api::read::{ListAlerts, ListAlertsResponse},
};
use mungos::{
  find::find_collect,
  mongodb::{bson::doc, options::FindOptions},
};
use resolver_api::Resolve;

use crate::{
  auth::RequestUser, helpers::resource::StateResource, state::State,
};

const NUM_ALERTS_PER_PAGE: u64 = 10;

#[async_trait]
impl Resolve<ListAlerts, RequestUser> for State {
  async fn resolve(
    &self,
    ListAlerts { query, page }: ListAlerts,
    user: RequestUser,
  ) -> anyhow::Result<ListAlertsResponse> {
    let mut query = query.unwrap_or_default();
    if !user.is_admin {
      let server_ids =
                <State as StateResource<Server>>::get_resource_ids_for_non_admin(self, &user.id)
                    .await?;
      let deployment_ids = <State as StateResource<
                Deployment,
            >>::get_resource_ids_for_non_admin(
                self, &user.id
            )
            .await?;
      query.extend(doc! {
                "$or": [
                   { "target.type": "Server", "target.id": { "$in": &server_ids } },
                   { "target.type": "Deployment", "target.id": { "$in": &deployment_ids } },
                ]
            });
    }

    let alerts = find_collect(
      &self.db.alerts,
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
