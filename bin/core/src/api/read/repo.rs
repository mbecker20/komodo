use std::str::FromStr;

use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::*,
  entities::{
    permission::PermissionLevel,
    repo::{Repo, RepoActionState, RepoListItem},
    update::ResourceTargetVariant,
    user::User,
  },
};
use mungos::mongodb::bson::{doc, oid::ObjectId};
use resolver_api::Resolve;

use crate::{
  helpers::resource::{
    get_resource_ids_for_non_admin, StateResource,
  },
  state::{action_states, db_client, State},
};

#[async_trait]
impl Resolve<GetRepo, User> for State {
  async fn resolve(
    &self,
    GetRepo { repo }: GetRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    Repo::get_resource_check_permissions(
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
    Repo::list_resources_for_user(query, &user).await
  }
}

#[async_trait]
impl Resolve<GetRepoActionState, User> for State {
  async fn resolve(
    &self,
    GetRepoActionState { repo }: GetRepoActionState,
    user: User,
  ) -> anyhow::Result<RepoActionState> {
    let repo = Repo::get_resource_check_permissions(
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
    let total = db_client()
      .await
      .repos
      .count_documents(query, None)
      .await
      .context("failed to count all build documents")?;
    let res = GetReposSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}
