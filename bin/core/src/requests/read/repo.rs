use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        repo::{Repo, RepoActionState, RepoListItem},
        PermissionLevel,
    },
    requests::read::*,
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{auth::RequestUser, helpers::resource::StateResource, state::State};

#[async_trait]
impl Resolve<GetRepo, RequestUser> for State {
    async fn resolve(&self, GetRepo { id }: GetRepo, user: RequestUser) -> anyhow::Result<Repo> {
        self.get_resource_check_permissions(&id, &user, PermissionLevel::Read)
            .await
    }
}

#[async_trait]
impl Resolve<ListRepos, RequestUser> for State {
    async fn resolve(
        &self,
        ListRepos { query }: ListRepos,
        user: RequestUser,
    ) -> anyhow::Result<Vec<RepoListItem>> {
        <State as StateResource<Repo>>::list_resources_for_user(self, query, &user).await
    }
}

#[async_trait]
impl Resolve<GetRepoActionState, RequestUser> for State {
    async fn resolve(
        &self,
        GetRepoActionState { id }: GetRepoActionState,
        user: RequestUser,
    ) -> anyhow::Result<RepoActionState> {
        let _: Repo = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let action_state = self.action_states.repo.get(&id).await.unwrap_or_default();
        Ok(action_state)
    }
}

#[async_trait]
impl Resolve<GetReposSummary, RequestUser> for State {
    async fn resolve(
        &self,
        GetReposSummary {}: GetReposSummary,
        user: RequestUser,
    ) -> anyhow::Result<GetReposSummaryResponse> {
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
            .repos
            .collection
            .count_documents(query, None)
            .await
            .context("failed to count all build documents")?;
        let res = GetReposSummaryResponse {
            total: total as u32,
        };
        Ok(res)
    }
}
