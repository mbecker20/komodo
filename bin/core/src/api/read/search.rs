use async_trait::async_trait;
use monitor_client::{
  api::read::{FindResources, FindResourcesResponse},
  entities::{
    build, deployment, procedure, repo, server,
    update::ResourceTargetVariant::{self, *},
    user::User,
  },
};
use resolver_api::Resolve;

use crate::{helpers::resource::StateResource, state::State};

const FIND_RESOURCE_TYPES: [ResourceTargetVariant; 5] =
  [Server, Build, Deployment, Repo, Procedure];

#[async_trait]
impl Resolve<FindResources, User> for State {
  async fn resolve(
    &self,
    FindResources { query, resources }: FindResources,
    user: User,
  ) -> anyhow::Result<FindResourcesResponse> {
    let mut res = FindResourcesResponse::default();
    let resource_types = if resources.is_empty() {
      FIND_RESOURCE_TYPES.to_vec()
    } else {
      resources
        .into_iter()
        .filter(|r| !matches!(r, System | Builder | Alerter))
        .collect()
    };
    for resource_type in resource_types {
      match resource_type {
        Server => {
          res.servers =
            server::Server::query_resource_list_items_for_user(
              query.clone(),
              &user,
            )
            .await?;
        }
        Deployment => {
          res.deployments =
            deployment::Deployment::query_resource_list_items_for_user(
              query.clone(),
              &user,
            )
            .await?;
        }
        Build => {
          res.builds =
            build::Build::query_resource_list_items_for_user(
              query.clone(),
              &user,
            )
            .await?;
        }
        Repo => {
          res.repos = repo::Repo::query_resource_list_items_for_user(
            query.clone(),
            &user,
          )
          .await?;
        }
        Procedure => {
          res.procedures =
            procedure::Procedure::query_resource_list_items_for_user(
              query.clone(),
              &user,
            )
            .await?;
        }
        _ => {}
      }
    }
    Ok(res)
  }
}
