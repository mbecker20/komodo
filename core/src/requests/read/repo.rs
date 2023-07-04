use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        repo::{Repo, RepoActionState},
        PermissionLevel,
    },
    permissioned::Permissioned,
    requests::read::*,
};
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
    ) -> anyhow::Result<Vec<Repo>> {
        let repos = self
            .db
            .repos
            .get_some(query, None)
            .await
            .context("failed to pull repos from mongo")?;

        let repos = if user.is_admin {
            repos
        } else {
            repos
                .into_iter()
                .filter(|repo| repo.get_user_permissions(&user.id) > PermissionLevel::None)
                .collect()
        };

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
