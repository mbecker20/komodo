use monitor_client::{
  api::write::*,
  entities::{permission::PermissionLevel, repo::Repo, user::User},
};
use resolver_api::Resolve;

use crate::{resource, state::State};

impl Resolve<CreateRepo, User> for State {
  #[instrument(name = "CreateRepo", skip(self, user))]
  async fn resolve(
    &self,
    CreateRepo { name, config }: CreateRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    resource::create::<Repo>(&name, config, &user).await
  }
}

impl Resolve<CopyRepo, User> for State {
  #[instrument(name = "CopyRepo", skip(self, user))]
  async fn resolve(
    &self,
    CopyRepo { name, id }: CopyRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    let Repo { config, .. } =
      resource::get_check_permissions::<Repo>(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;
    resource::create::<Repo>(&name, config.into(), &user).await
  }
}

impl Resolve<DeleteRepo, User> for State {
  #[instrument(name = "DeleteRepo", skip(self, user))]
  async fn resolve(
    &self,
    DeleteRepo { id }: DeleteRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    resource::delete::<Repo>(&id, &user).await
  }
}

impl Resolve<UpdateRepo, User> for State {
  #[instrument(name = "UpdateRepo", skip(self, user))]
  async fn resolve(
    &self,
    UpdateRepo { id, config }: UpdateRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    resource::update::<Repo>(&id, config, &user).await
  }
}
