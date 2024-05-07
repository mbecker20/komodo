use axum::async_trait;
use monitor_client::{
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

#[async_trait]
impl Resolve<CreateServerTemplate, User> for State {
  async fn resolve(
    &self,
    CreateServerTemplate { name, config }: CreateServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    resource::create::<ServerTemplate>(&name, config, &user).await
  }
}

#[async_trait]
impl Resolve<CopyServerTemplate, User> for State {
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

#[async_trait]
impl Resolve<DeleteServerTemplate, User> for State {
  async fn resolve(
    &self,
    DeleteServerTemplate { id }: DeleteServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    resource::delete::<ServerTemplate>(&id, &user).await
  }
}

#[async_trait]
impl Resolve<UpdateServerTemplate, User> for State {
  async fn resolve(
    &self,
    UpdateServerTemplate { id, config }: UpdateServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    resource::update::<ServerTemplate>(&id, config, &user).await
  }
}
