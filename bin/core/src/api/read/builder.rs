use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::{self, *},
  entities::{
    builder::{Builder, BuilderConfig, BuilderListItem},
    resource::AddFilters,
    user::User,
    PermissionLevel,
  },
};
use mungos::mongodb::bson::{doc, Document};
use resolver_api::Resolve;

use crate::{
  db::db_client, helpers::resource::StateResource, state::State,
};

#[async_trait]
impl Resolve<GetBuilder, User> for State {
  async fn resolve(
    &self,
    GetBuilder { id }: GetBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    self
      .get_resource_check_permissions(
        &id,
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
    <State as StateResource<Builder>>::list_resources_for_user(
      self, filters, &user,
    )
    .await
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
      let query = doc! {
          format!("permissions.{}", user.id): { "$in": ["read", "execute", "update"] }
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
    GetBuilderAvailableAccounts { id }: GetBuilderAvailableAccounts,
    user: User,
  ) -> anyhow::Result<GetBuilderAvailableAccountsResponse> {
    let builder: Builder = self
      .get_resource_check_permissions(
        &id,
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
            read::GetAvailableAccounts {
              server_id: config.id,
            },
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
