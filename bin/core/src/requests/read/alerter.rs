use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{alerter::Alerter, PermissionLevel},
    requests::read::*,
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{auth::RequestUser, resource::Resource, state::State};

#[async_trait]
impl Resolve<GetAlerter, RequestUser> for State {
    async fn resolve(
        &self,
        GetAlerter { id }: GetAlerter,
        user: RequestUser,
    ) -> anyhow::Result<Alerter> {
        self.get_resource_check_permissions(&id, &user, PermissionLevel::Read)
            .await
    }
}

#[async_trait]
impl Resolve<ListAlerters, RequestUser> for State {
    async fn resolve(
        &self,
        ListAlerters { query }: ListAlerters,
        user: RequestUser,
    ) -> anyhow::Result<Vec<Alerter>> {
        <State as Resource<Alerter>>::list_resources_for_user(self, &user, query).await
    }
}

#[async_trait]
impl Resolve<GetAlertersSummary, RequestUser> for State {
    async fn resolve(
        &self,
        GetAlertersSummary {}: GetAlertersSummary,
        user: RequestUser,
    ) -> anyhow::Result<GetAlertersSummaryResponse> {
        let query = if user.is_admin {
            None
        } else {
            let query = doc! {
                format!("permissions.{}", user.id): { "$in": ["read", "execute", "update"] }
            };
            Some(query)
        };
        let total = self
            .db
            .alerters
            .collection
            .count_documents(query, None)
            .await
            .context("failed to count all alerter documents")?;
        let res = GetAlertersSummaryResponse {
            total: total as u32,
        };
        Ok(res)
    }
}
