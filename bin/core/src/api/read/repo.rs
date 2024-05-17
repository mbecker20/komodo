use std::str::FromStr;

use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::*,
  entities::{
    permission::PermissionLevel,
    repo::{Repo, RepoActionState, RepoListItem, RepoState},
    update::ResourceTargetVariant,
    user::User,
  },
};
use mungos::{
  find::find_collect,
  mongodb::bson::{doc, oid::ObjectId},
};
use resolver_api::Resolve;

use crate::{
  helpers::query::get_resource_ids_for_non_admin,
  resource,
  state::{action_states, db_client, repo_state_cache, State},
};

#[async_trait]
impl Resolve<GetRepo, User> for State {
  async fn resolve(
    &self,
    GetRepo { repo }: GetRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    resource::get_check_permissions::<Repo>(
      &repo,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

#[async_trait]
impl Resolve<ListRepos, User> for State {
  async fn resolve(
    &self,
    ListRepos { query }: ListRepos,
    user: User,
  ) -> anyhow::Result<Vec<RepoListItem>> {
    resource::list_for_user::<Repo>(query, &user).await
  }
}

#[async_trait]
impl Resolve<GetRepoActionState, User> for State {
  async fn resolve(
    &self,
    GetRepoActionState { repo }: GetRepoActionState,
    user: User,
  ) -> anyhow::Result<RepoActionState> {
    let repo = resource::get_check_permissions::<Repo>(
      &repo,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .repo
      .get(&repo.id)
      .await
      .unwrap_or_default()
      .get()?;
    Ok(action_state)
  }
}

#[async_trait]
impl Resolve<GetReposSummary, User> for State {
  async fn resolve(
    &self,
    GetReposSummary {}: GetReposSummary,
    user: User,
  ) -> anyhow::Result<GetReposSummaryResponse> {
    let query = if user.admin {
      None
    } else {
      let ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Alerter,
      )
      .await?
      .into_iter()
      .flat_map(|id| ObjectId::from_str(&id))
      .collect::<Vec<_>>();
      let query = doc! {
        "_id": { "$in": ids }
      };
      Some(query)
    };

    let repos = find_collect(&db_client().await.repos, query, None)
      .await
      .context("failed to find all repo documents")?;
    let mut res = GetReposSummaryResponse::default();

    let cache = repo_state_cache();
    let action_states = action_states();

    for repo in repos {
      res.total += 1;

      match (
        cache.get(&repo.id).await.unwrap_or_default(),
        action_states
          .repo
          .get(&repo.id)
          .await
          .unwrap_or_default()
          .get()?,
      ) {
        (_, action_states) if action_states.cloning => {
          res.cloning += 1;
        }
        (_, action_states) if action_states.pulling => {
          res.pulling += 1;
        }
        (RepoState::Ok, _) => res.ok += 1,
        (RepoState::Failed, _) => res.failed += 1,
        (RepoState::Unknown, _) => res.unknown += 1,
        // will never come off the cache in the building state, since that comes from action states
        (RepoState::Cloning, _) | (RepoState::Pulling, _) => {
          unreachable!()
        }
      }
    }

    Ok(res)
  }
}
