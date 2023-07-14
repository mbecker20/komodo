use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{builder::Builder, PermissionLevel},
    permissioned::Permissioned,
    requests::read::*,
};
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
        let builders = self
            .db
            .builders
            .get_some(query, None)
            .await
            .context("failed to pull builders from mongo")?;

        let builders = if user.is_admin {
            builders
        } else {
            builders
                .into_iter()
                .filter(|builder| builder.get_user_permissions(&user.id) > PermissionLevel::None)
                .collect()
        };

        Ok(builders)
    }
}
