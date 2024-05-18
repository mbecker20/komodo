use monitor_client::{
  api::write::*,
  entities::{build::Build, permission::PermissionLevel, user::User},
};
use resolver_api::Resolve;

use crate::{resource, state::State};

impl Resolve<CreateBuild, User> for State {
  #[instrument(name = "CreateBuild", skip(self, user))]
  async fn resolve(
    &self,
    CreateBuild { name, config }: CreateBuild,
    user: User,
  ) -> anyhow::Result<Build> {
    resource::create::<Build>(&name, config, &user).await
  }
}

impl Resolve<CopyBuild, User> for State {
  #[instrument(name = "CopyBuild", skip(self, user))]
  async fn resolve(
    &self,
    CopyBuild { name, id }: CopyBuild,
    user: User,
  ) -> anyhow::Result<Build> {
    let Build {
      config,
      ..
    } = resource::get_check_permissions::<Build>(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;
    resource::create::<Build>(&name, config.into(), &user).await
  }
}

impl Resolve<DeleteBuild, User> for State {
  #[instrument(name = "DeleteBuild", skip(self, user))]
  async fn resolve(
    &self,
    DeleteBuild { id }: DeleteBuild,
    user: User,
  ) -> anyhow::Result<Build> {
    resource::delete::<Build>(&id, &user).await
  }
}

impl Resolve<UpdateBuild, User> for State {
  #[instrument(name = "UpdateBuild", skip(self, user))]
  async fn resolve(
    &self,
    UpdateBuild { id, config }: UpdateBuild,
    user: User,
  ) -> anyhow::Result<Build> {
    resource::update::<Build>(&id, config, &user).await
  }
}
