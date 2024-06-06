use monitor_client::{
  api::write::*,
  entities::{
    permission::PermissionLevel, sync::ResourceSync, user::User,
  },
};
use resolver_api::Resolve;

use crate::{resource, state::State};

impl Resolve<CreateResourceSync, User> for State {
  #[instrument(name = "CreateResourceSync", skip(self, user))]
  async fn resolve(
    &self,
    CreateResourceSync { name, config }: CreateResourceSync,
    user: User,
  ) -> anyhow::Result<ResourceSync> {
    resource::create::<ResourceSync>(&name, config, &user).await
  }
}

impl Resolve<CopyResourceSync, User> for State {
  #[instrument(name = "CopyResourceSync", skip(self, user))]
  async fn resolve(
    &self,
    CopyResourceSync { name, id }: CopyResourceSync,
    user: User,
  ) -> anyhow::Result<ResourceSync> {
    let ResourceSync { config, .. } =
      resource::get_check_permissions::<ResourceSync>(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;
    resource::create::<ResourceSync>(&name, config.into(), &user)
      .await
  }
}

impl Resolve<DeleteResourceSync, User> for State {
  #[instrument(name = "DeleteResourceSync", skip(self, user))]
  async fn resolve(
    &self,
    DeleteResourceSync { id }: DeleteResourceSync,
    user: User,
  ) -> anyhow::Result<ResourceSync> {
    resource::delete::<ResourceSync>(&id, &user).await
  }
}

impl Resolve<UpdateResourceSync, User> for State {
  #[instrument(name = "UpdateResourceSync", skip(self, user))]
  async fn resolve(
    &self,
    UpdateResourceSync { id, config }: UpdateResourceSync,
    user: User,
  ) -> anyhow::Result<ResourceSync> {
    resource::update::<ResourceSync>(&id, config, &user).await
  }
}
