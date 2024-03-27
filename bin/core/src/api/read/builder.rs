use std::str::FromStr;

use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::{self, *},
  entities::{
    builder::{Builder, BuilderConfig, BuilderListItem},
    permission::PermissionLevel,
    resource::AddFilters,
    update::ResourceTargetVariant,
    user::User,
  },
};
use mungos::mongodb::bson::{doc, oid::ObjectId, Document};
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::resource::{
    get_resource_ids_for_non_admin, StateResource,
  },
  state::State,
};

#[async_trait]
impl Resolve<GetBuilder, User> for State {
  async fn resolve(
    &self,
    GetBuilder { builder }: GetBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    Builder::get_resource_check_permissions(
      &builder,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

#[async_trait]
impl Resolve<ListBuilders, User> for State {
  async fn resolve(
    &self,
    ListBuilders { query }: ListBuilders,
    user: User,
  ) -> anyhow::Result<Vec<BuilderListItem>> {
    let mut filters = Document::new();
    query.add_filters(&mut filters);
    Builder::list_resources_for_user(filters, &user).await
  }
}

#[async_trait]
impl Resolve<GetBuildersSummary, User> for State {
  async fn resolve(
    &self,
    GetBuildersSummary {}: GetBuildersSummary,
    user: User,
  ) -> anyhow::Result<GetBuildersSummaryResponse> {
    let query = if user.admin {
      None
    } else {
      let ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Builder,
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
      .builders
      .count_documents(query, None)
      .await
      .context("failed to count all builder documents")?;
    let res = GetBuildersSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}

#[async_trait]
impl Resolve<GetBuilderAvailableAccounts, User> for State {
  async fn resolve(
    &self,
    GetBuilderAvailableAccounts { builder }: GetBuilderAvailableAccounts,
    user: User,
  ) -> anyhow::Result<GetBuilderAvailableAccountsResponse> {
    let builder = Builder::get_resource_check_permissions(
      &builder,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    match builder.config {
      BuilderConfig::Aws(config) => {
        Ok(GetBuilderAvailableAccountsResponse {
          github: config.github_accounts,
          docker: config.docker_accounts,
        })
      }
      BuilderConfig::Server(config) => {
        let res = self
          .resolve(
            read::GetAvailableAccounts { server: config.id },
            user,
          )
          .await?;
        Ok(GetBuilderAvailableAccountsResponse {
          github: res.github,
          docker: res.docker,
        })
      }
    }
  }
}
