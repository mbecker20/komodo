use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        repo::{Repo, RepoActionState},
        PermissionLevel,
    },
    requests::read::*,
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{auth::RequestUser, resource::Resource, state::State};

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
        let repos: Vec<Repo> = self.list_resources_for_user(&user, query).await?;

        let repos = repos
            .into_iter()
            .map(|repo| RepoListItem {
                id: repo.id,
                name: repo.name,
                last_pulled_at: repo.last_pulled_at,
                tags: repo.tags,
            })
            .collect();

        Ok(repos)
    }
}

#[async_trait]
impl Resolve<GetRepoActionState, RequestUser> for State {
    async fn resolve(
        &self,
        GetRepoActionState { id }: GetRepoActionState,
        user: RequestUser,
    ) -> anyhow::Result<RepoActionState> {
        let _: Repo = self.get_resource_check_permissions(&id, &user, PermissionLevel::Read)
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
