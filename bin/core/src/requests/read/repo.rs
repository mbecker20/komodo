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

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<GetRepo, RequestUser> for State {
    async fn resolve(&self, GetRepo { id }: GetRepo, user: RequestUser) -> anyhow::Result<Repo> {
        self.get_repo_check_permissions(&id, &user, PermissionLevel::Read)
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
        let mut query = query.unwrap_or_default();
        if !user.is_admin {
            query.insert(
                format!("permissions.{}", user.id),
                doc! { "$in": ["read", "execute", "update"] },
            );
        }

        let repos = self
            .db
            .repos
            .get_some(query, None)
            .await
            .context("failed to pull repos from mongo")?;

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
        self.get_repo_check_permissions(&id, &user, PermissionLevel::Read)
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
        todo!()
    }
}
