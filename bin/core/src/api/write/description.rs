use anyhow::anyhow;
use async_trait::async_trait;
use monitor_client::{
  api::write::{UpdateDescription, UpdateDescriptionResponse},
  entities::{
    alerter::Alerter, build::Build, builder::Builder,
    deployment::Deployment, procedure::Procedure, repo::Repo,
    server::Server, server_template::ServerTemplate,
    update::ResourceTarget, user::User,
  },
};
use resolver_api::Resolve;

use crate::{helpers::resource::StateResource, state::State};

#[async_trait]
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
        Server::update_description(&id, &description, &user).await?;
      }
      ResourceTarget::Deployment(id) => {
        Deployment::update_description(&id, &description, &user)
          .await?;
      }
      ResourceTarget::Build(id) => {
        Build::update_description(&id, &description, &user).await?;
      }
      ResourceTarget::Repo(id) => {
        Repo::update_description(&id, &description, &user).await?;
      }
      ResourceTarget::Builder(id) => {
        Builder::update_description(&id, &description, &user).await?;
      }
      ResourceTarget::Alerter(id) => {
        Alerter::update_description(&id, &description, &user).await?;
      }
      ResourceTarget::Procedure(id) => {
        Procedure::update_description(&id, &description, &user)
          .await?;
      }
      ResourceTarget::ServerTemplate(id) => {
        ServerTemplate::update_description(&id, &description, &user)
          .await?;
      }
    }
    Ok(UpdateDescriptionResponse {})
  }
}
