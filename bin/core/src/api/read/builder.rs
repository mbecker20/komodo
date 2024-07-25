use std::collections::HashSet;

use anyhow::Context;
use mongo_indexed::Document;
use monitor_client::{
  api::read::{self, *},
  entities::{
    builder::{Builder, BuilderConfig, BuilderListItem},
    config::{DockerAccount, GitAccount},
    permission::PermissionLevel,
    update::ResourceTargetVariant,
    user::User,
  },
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::query::get_resource_ids_for_user,
  resource,
  state::{db_client, State},
};

impl Resolve<GetBuilder, User> for State {
  async fn resolve(
    &self,
    GetBuilder { builder }: GetBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    resource::get_check_permissions::<Builder>(
      &builder,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

impl Resolve<ListBuilders, User> for State {
  async fn resolve(
    &self,
    ListBuilders { query }: ListBuilders,
    user: User,
  ) -> anyhow::Result<Vec<BuilderListItem>> {
    resource::list_for_user::<Builder>(query, &user).await
  }
}

impl Resolve<ListFullBuilders, User> for State {
  async fn resolve(
    &self,
    ListFullBuilders { query }: ListFullBuilders,
    user: User,
  ) -> anyhow::Result<ListFullBuildersResponse> {
    resource::list_full_for_user::<Builder>(query, &user).await
  }
}

impl Resolve<GetBuildersSummary, User> for State {
  async fn resolve(
    &self,
    GetBuildersSummary {}: GetBuildersSummary,
    user: User,
  ) -> anyhow::Result<GetBuildersSummaryResponse> {
    let query = match get_resource_ids_for_user(
      &user,
      ResourceTargetVariant::Builder,
    )
    .await?
    {
      Some(ids) => doc! {
        "_id": { "$in": ids }
      },
      None => Document::new(),
    };
    let total = db_client()
      .await
      .builders
      .count_documents(query)
      .await
      .context("failed to count all builder documents")?;
    let res = GetBuildersSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}

impl Resolve<GetBuilderAvailableAccounts, User> for State {
  async fn resolve(
    &self,
    GetBuilderAvailableAccounts { builder }: GetBuilderAvailableAccounts,
    user: User,
  ) -> anyhow::Result<GetBuilderAvailableAccountsResponse> {
    let builder = resource::get_check_permissions::<Builder>(
      &builder,
      &user,
      PermissionLevel::Read,
    )
    .await?;
  
    let (git, docker) = match builder.config {
      BuilderConfig::Aws(config) => {
        (config.git_accounts, config.docker_accounts)
      }
      BuilderConfig::Server(config) => {
        let res = self
          .resolve(
            read::GetAvailableAccounts {
              server: Some(config.server_id),
            },
            user,
          )
          .await?;
        (res.git, res.docker)
      }
    };

    let mut git_set = HashSet::<GitAccount>::new();

    git_set.extend(core_config().git_accounts.clone());
    git_set.extend(git);

    let mut git = git_set.into_iter().collect::<Vec<_>>();
    git.sort();

    let mut docker_set = HashSet::<DockerAccount>::new();

    docker_set.extend(core_config().docker_accounts.clone());
    docker_set.extend(docker);

    let mut docker = docker_set.into_iter().collect::<Vec<_>>();
    docker.sort();

    Ok(GetBuilderAvailableAccountsResponse { git, docker })
  }
}
