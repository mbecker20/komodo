use komodo_client::{
  api::write::*,
  entities::{
    builder::Builder, permission::PermissionLevel, user::User,
  },
};
use resolver_api::Resolve;

use crate::{resource, state::State};

impl Resolve<CreateBuilder, User> for State {
  #[instrument(name = "CreateBuilder", skip(self, user))]
  async fn resolve(
    &self,
    CreateBuilder { name, config }: CreateBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    resource::create::<Builder>(&name, config, &user).await
  }
}

impl Resolve<CopyBuilder, User> for State {
  #[instrument(name = "CopyBuilder", skip(self, user))]
  async fn resolve(
    &self,
    CopyBuilder { name, id }: CopyBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    let Builder { config, .. } = resource::get_check_permissions::<
      Builder,
    >(
      &id, &user, PermissionLevel::Write
    )
    .await?;
    resource::create::<Builder>(&name, config.into(), &user).await
  }
}

impl Resolve<DeleteBuilder, User> for State {
  #[instrument(name = "DeleteBuilder", skip(self, user))]
  async fn resolve(
    &self,
    DeleteBuilder { id }: DeleteBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    resource::delete::<Builder>(&id, &user).await
  }
}

impl Resolve<UpdateBuilder, User> for State {
  #[instrument(name = "UpdateBuilder", skip(self, user))]
  async fn resolve(
    &self,
    UpdateBuilder { id, config }: UpdateBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    resource::update::<Builder>(&id, config, &user).await
  }
}
