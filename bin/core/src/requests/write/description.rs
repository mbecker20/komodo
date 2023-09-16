use anyhow::anyhow;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        alerter::Alerter, build::Build, builder::Builder,
        deployment::Deployment, repo::Repo, server::Server,
        update::ResourceTarget,
    },
    requests::write::{UpdateDescription, UpdateDescriptionResponse},
};
use resolver_api::Resolve;

use crate::{
    auth::RequestUser, helpers::resource::StateResource, state::State,
};

#[async_trait]
impl Resolve<UpdateDescription, RequestUser> for State {
    async fn resolve(
        &self,
        UpdateDescription {
            target,
            description,
        }: UpdateDescription,
        user: RequestUser,
    ) -> anyhow::Result<UpdateDescriptionResponse> {
        match target {
            ResourceTarget::System(_) => {
                return Err(anyhow!(
                "cannot update description of System resource target"
            ))
            }
            ResourceTarget::Server(id) => {
                <State as StateResource<Server>>::update_description(
                    self,
                    &id,
                    &description,
                    &user,
                )
                .await?;
            }
            ResourceTarget::Deployment(id) => {
                <State as StateResource<Deployment>>::update_description(
                    self,
                    &id,
                    &description,
                    &user,
                )
                .await?;
            }
            ResourceTarget::Build(id) => {
                <State as StateResource<Build>>::update_description(
                    self,
                    &id,
                    &description,
                    &user,
                )
                .await?;
            }
            ResourceTarget::Repo(id) => {
                <State as StateResource<Repo>>::update_description(
                    self,
                    &id,
                    &description,
                    &user,
                )
                .await?;
            }
            ResourceTarget::Builder(id) => {
                <State as StateResource<Builder>>::update_description(
                    self,
                    &id,
                    &description,
                    &user,
                )
                .await?;
            }
            ResourceTarget::Alerter(id) => {
                <State as StateResource<Alerter>>::update_description(
                    self,
                    &id,
                    &description,
                    &user,
                )
                .await?;
            }
        }
        Ok(UpdateDescriptionResponse {})
    }
}
