use anyhow::anyhow;
use komodo_client::{
  api::write::{UpdateDescription, UpdateDescriptionResponse},
  entities::{
    alerter::Alerter, build::Build, builder::Builder,
    deployment::Deployment, procedure::Procedure, repo::Repo,
    server::Server, server_template::ServerTemplate, stack::Stack,
    sync::ResourceSync, user::User, ResourceTarget,
  },
};
use resolver_api::Resolve;

use crate::{resource, state::State};

impl Resolve<UpdateDescription, User> for State {
  #[instrument(name = "UpdateDescription", skip(self, user))]
  async fn resolve(
    &self,
    UpdateDescription {
      target,
      description,
    }: UpdateDescription,
    user: User,
  ) -> anyhow::Result<UpdateDescriptionResponse> {
    match target {
      ResourceTarget::System(_) => {
        return Err(anyhow!(
          "cannot update description of System resource target"
        ))
      }
      ResourceTarget::Server(id) => {
        resource::update_description::<Server>(
          &id,
          &description,
          &user,
        )
        .await?;
      }
      ResourceTarget::Deployment(id) => {
        resource::update_description::<Deployment>(
          &id,
          &description,
          &user,
        )
        .await?;
      }
      ResourceTarget::Build(id) => {
        resource::update_description::<Build>(
          &id,
          &description,
          &user,
        )
        .await?;
      }
      ResourceTarget::Repo(id) => {
        resource::update_description::<Repo>(
          &id,
          &description,
          &user,
        )
        .await?;
      }
      ResourceTarget::Builder(id) => {
        resource::update_description::<Builder>(
          &id,
          &description,
          &user,
        )
        .await?;
      }
      ResourceTarget::Alerter(id) => {
        resource::update_description::<Alerter>(
          &id,
          &description,
          &user,
        )
        .await?;
      }
      ResourceTarget::Procedure(id) => {
        resource::update_description::<Procedure>(
          &id,
          &description,
          &user,
        )
        .await?;
      }
      ResourceTarget::ServerTemplate(id) => {
        resource::update_description::<ServerTemplate>(
          &id,
          &description,
          &user,
        )
        .await?;
      }
      ResourceTarget::ResourceSync(id) => {
        resource::update_description::<ResourceSync>(
          &id,
          &description,
          &user,
        )
        .await?;
      }
      ResourceTarget::Stack(id) => {
        resource::update_description::<Stack>(
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
