use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{builder::Builder, PermissionLevel},
    requests::read::*,
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<GetBuilder, RequestUser> for State {
    async fn resolve(
        &self,
        GetBuilder { id }: GetBuilder,
        user: RequestUser,
    ) -> anyhow::Result<Builder> {
        self.get_builder_check_permissions(&id, &user, PermissionLevel::Read)
            .await
    }
}

#[async_trait]
impl Resolve<ListBuilders, RequestUser> for State {
    async fn resolve(
        &self,
        ListBuilders { query }: ListBuilders,
        user: RequestUser,
    ) -> anyhow::Result<Vec<Builder>> {
        let mut query = query.unwrap_or_default();
        if !user.is_admin {
            query.insert(
                format!("permissions.{}", user.id),
                doc! { "$in": ["read", "execute", "update"] },
            );
        }

        let builders = self
            .db
            .builders
            .get_some(query, None)
            .await
            .context("failed to pull builders from mongo")?;

        Ok(builders)
    }
}

#[async_trait]
impl Resolve<GetBuildersSummary, RequestUser> for State {
    async fn resolve(
        &self,
        GetBuildersSummary {}: GetBuildersSummary,
        user: RequestUser,
    ) -> anyhow::Result<GetBuildersSummaryResponse> {
        todo!()
    }
}
