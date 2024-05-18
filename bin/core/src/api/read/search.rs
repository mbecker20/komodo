use monitor_client::{
  api::read::{FindResources, FindResourcesResponse},
  entities::{
    build::Build, deployment::Deployment, procedure::Procedure,
    repo::Repo, server::Server, update::ResourceTargetVariant,
    user::User,
  },
};
use resolver_api::Resolve;

use crate::{resource, state::State};

const FIND_RESOURCE_TYPES: [ResourceTargetVariant; 5] = [
  ResourceTargetVariant::Server,
  ResourceTargetVariant::Build,
  ResourceTargetVariant::Deployment,
  ResourceTargetVariant::Repo,
  ResourceTargetVariant::Procedure,
];

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
        .filter(|r| {
          !matches!(
            r,
            ResourceTargetVariant::System
              | ResourceTargetVariant::Builder
              | ResourceTargetVariant::Alerter
          )
        })
        .collect()
    };
    for resource_type in resource_types {
      match resource_type {
        ResourceTargetVariant::Server => {
          res.servers = resource::list_for_user_using_document::<
            Server,
          >(query.clone(), &user)
          .await?;
        }
        ResourceTargetVariant::Deployment => {
          res.deployments = resource::list_for_user_using_document::<
            Deployment,
          >(query.clone(), &user)
          .await?;
        }
        ResourceTargetVariant::Build => {
          res.builds =
            resource::list_for_user_using_document::<Build>(
              query.clone(),
              &user,
            )
            .await?;
        }
        ResourceTargetVariant::Repo => {
          res.repos = resource::list_for_user_using_document::<Repo>(
            query.clone(),
            &user,
          )
          .await?;
        }
        ResourceTargetVariant::Procedure => {
          res.procedures = resource::list_for_user_using_document::<
            Procedure,
          >(query.clone(), &user)
          .await?;
        }
        _ => {}
      }
    }
    Ok(res)
  }
}
