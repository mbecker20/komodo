use monitor_client::{
  api::write::{
    CopyAlerter, CreateAlerter, DeleteAlerter, UpdateAlerter,
  },
  entities::{
    alerter::Alerter, permission::PermissionLevel, user::User,
  },
};
use resolver_api::Resolve;

use crate::{resource, state::State};

impl Resolve<CreateAlerter, User> for State {
  #[instrument(name = "CreateAlerter", skip(self, user))]
  async fn resolve(
    &self,
    CreateAlerter { name, config }: CreateAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    resource::create::<Alerter>(&name, config, &user).await
  }
}

impl Resolve<CopyAlerter, User> for State {
  #[instrument(name = "CopyAlerter", skip(self, user))]
  async fn resolve(
    &self,
    CopyAlerter { name, id }: CopyAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    let Alerter {
      config,
      ..
    } = resource::get_check_permissions::<Alerter>(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;
    resource::create::<Alerter>(&name, config.into(), &user).await
  }
}

impl Resolve<DeleteAlerter, User> for State {
  #[instrument(name = "DeleteAlerter", skip(self, user))]
  async fn resolve(
    &self,
    DeleteAlerter { id }: DeleteAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    resource::delete::<Alerter>(&id, &user).await
  }
}

impl Resolve<UpdateAlerter, User> for State {
  #[instrument(name = "UpdateAlerter", skip(self, user))]
  async fn resolve(
    &self,
    UpdateAlerter { id, config }: UpdateAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    resource::update::<Alerter>(&id, config, &user).await
  }
}
