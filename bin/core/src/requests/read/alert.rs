use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::server::Server,
    requests::read::{ListAlerts, ListAlertsResponse},
};
use mungos::mongodb::{
    bson::{doc, Document},
    options::FindOptions,
};
use resolver_api::Resolve;

use crate::{auth::RequestUser, helpers::resource::StateResource, state::State};

const NUM_ALERTS_PER_PAGE: u64 = 10;

#[async_trait]
impl Resolve<ListAlerts, RequestUser> for State {
    async fn resolve(
        &self,
        ListAlerts {
            page,
            include_resolved,
        }: ListAlerts,
        user: RequestUser,
    ) -> anyhow::Result<ListAlertsResponse> {
        let mut query = Document::new();
        if !include_resolved {
            query.insert("resolved", false);
        }
        if !user.is_admin {
            let server_ids =
                <State as StateResource<Server>>::get_resource_ids_for_non_admin(self, &user.id)
                    .await?;
            query.insert("target.type", "Server");
            query.insert("target.id", doc! { "$in": server_ids });
        }
        let alerts = self
            .db
            .alerts
            .get_some(
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
