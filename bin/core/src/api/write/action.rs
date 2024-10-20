use komodo_client::{
  api::write::*,
  entities::{
    action::Action, permission::PermissionLevel, user::User,
  },
};
use resolver_api::Resolve;

use crate::{resource, state::State};

impl Resolve<CreateAction, User> for State {
  #[instrument(name = "CreateAction", skip(self, user))]
  async fn resolve(
    &self,
    CreateAction { name, config }: CreateAction,
    user: User,
  ) -> anyhow::Result<Action> {
    resource::create::<Action>(&name, config, &user).await
  }
}

impl Resolve<CopyAction, User> for State {
  #[instrument(name = "CopyAction", skip(self, user))]
  async fn resolve(
    &self,
    CopyAction { name, id }: CopyAction,
    user: User,
  ) -> anyhow::Result<Action> {
    let Action { config, .. } = resource::get_check_permissions::<
      Action,
    >(
      &id, &user, PermissionLevel::Write
    )
    .await?;
    resource::create::<Action>(&name, config.into(), &user).await
  }
}

impl Resolve<UpdateAction, User> for State {
  #[instrument(name = "UpdateAction", skip(self, user))]
  async fn resolve(
    &self,
    UpdateAction { id, config }: UpdateAction,
    user: User,
  ) -> anyhow::Result<Action> {
    resource::update::<Action>(&id, config, &user).await
  }
}

impl Resolve<DeleteAction, User> for State {
  #[instrument(name = "DeleteAction", skip(self, user))]
  async fn resolve(
    &self,
    DeleteAction { id }: DeleteAction,
    user: User,
  ) -> anyhow::Result<Action> {
    resource::delete::<Action>(&id, &user).await
  }
}
