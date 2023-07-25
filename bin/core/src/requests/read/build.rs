use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        build::{Build, BuildActionState},
        PermissionLevel,
    },
    permissioned::Permissioned,
    requests::read::*,
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<GetBuild, RequestUser> for State {
    async fn resolve(&self, GetBuild { id }: GetBuild, user: RequestUser) -> anyhow::Result<Build> {
        self.get_build_check_permissions(&id, &user, PermissionLevel::Read)
            .await
    }
}

#[async_trait]
impl Resolve<ListBuilds, RequestUser> for State {
    async fn resolve(
        &self,
        ListBuilds { query }: ListBuilds,
        user: RequestUser,
    ) -> anyhow::Result<Vec<BuildListItem>> {
        let builds = self
            .db
            .builds
            .get_some(query, None)
            .await
            .context("failed to pull builds from mongo")?;

        let builds = if user.is_admin {
            builds
        } else {
            builds
                .into_iter()
                .filter(|build| build.get_user_permissions(&user.id) > PermissionLevel::None)
                .collect()
        };

        let builds = builds
            .into_iter()
            .map(|build| BuildListItem {
                id: build.id,
                name: build.name,
                last_built_at: build.last_built_at,
                version: build.config.version,
                tags: build.tags,
            })
            .collect();

        Ok(builds)
    }
}

#[async_trait]
impl Resolve<GetBuildActionState, RequestUser> for State {
    async fn resolve(
        &self,
        GetBuildActionState { id }: GetBuildActionState,
        user: RequestUser,
    ) -> anyhow::Result<BuildActionState> {
        self.get_build_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let action_state = self.action_states.build.get(&id).await.unwrap_or_default();
        Ok(action_state)
    }
}

#[async_trait]
impl Resolve<GetBuildsSummary, RequestUser> for State {
    async fn resolve(
        &self,
        GetBuildsSummary {}: GetBuildsSummary,
        user: RequestUser,
    ) -> anyhow::Result<GetBuildsSummaryResponse> {
        let query = if user.is_admin {
            None
        } else {
            let doc = doc! {
                
            };
            Some(doc)
        };
        let total = self.db.builds.collection.count_documents(query, None).await.context("failed to count all build documents")?;

        let res = GetBuildsSummaryResponse {
            total: total as u32
        };
        Ok(res)
    }
}
