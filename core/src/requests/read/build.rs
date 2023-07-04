use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        build::{Build, BuildActionState},
        PermissionLevel,
    },
    requests::read::*, permissioned::Permissioned,
};
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
    ) -> anyhow::Result<Vec<Build>> {
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
