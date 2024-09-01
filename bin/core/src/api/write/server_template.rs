use komodo_client::{
  api::write::{
    CopyServerTemplate, CreateServerTemplate, DeleteServerTemplate,
    UpdateServerTemplate,
  },
  entities::{
    permission::PermissionLevel, server_template::ServerTemplate,
    user::User,
  },
};
use resolver_api::Resolve;

use crate::{resource, state::State};

impl Resolve<CreateServerTemplate, User> for State {
  #[instrument(name = "CreateServerTemplate", skip(self, user))]
  async fn resolve(
    &self,
    CreateServerTemplate { name, config }: CreateServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    resource::create::<ServerTemplate>(&name, config, &user).await
  }
}

impl Resolve<CopyServerTemplate, User> for State {
  #[instrument(name = "CopyServerTemplate", skip(self, user))]
  async fn resolve(
    &self,
    CopyServerTemplate { name, id }: CopyServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    let ServerTemplate { config, .. } =
      resource::get_check_permissions::<ServerTemplate>(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;
    resource::create::<ServerTemplate>(&name, config.into(), &user)
      .await
  }
}

impl Resolve<DeleteServerTemplate, User> for State {
  #[instrument(name = "DeleteServerTemplate", skip(self, user))]
  async fn resolve(
    &self,
    DeleteServerTemplate { id }: DeleteServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    resource::delete::<ServerTemplate>(&id, &user).await
  }
}

impl Resolve<UpdateServerTemplate, User> for State {
  #[instrument(name = "UpdateServerTemplate", skip(self, user))]
  async fn resolve(
    &self,
    UpdateServerTemplate { id, config }: UpdateServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    resource::update::<ServerTemplate>(&id, config, &user).await
  }
}
