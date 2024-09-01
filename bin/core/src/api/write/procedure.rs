use komodo_client::{
  api::write::*,
  entities::{
    permission::PermissionLevel, procedure::Procedure, user::User,
  },
};
use resolver_api::Resolve;

use crate::{resource, state::State};

impl Resolve<CreateProcedure, User> for State {
  #[instrument(name = "CreateProcedure", skip(self, user))]
  async fn resolve(
    &self,
    CreateProcedure { name, config }: CreateProcedure,
    user: User,
  ) -> anyhow::Result<CreateProcedureResponse> {
    resource::create::<Procedure>(&name, config, &user).await
  }
}

impl Resolve<CopyProcedure, User> for State {
  #[instrument(name = "CopyProcedure", skip(self, user))]
  async fn resolve(
    &self,
    CopyProcedure { name, id }: CopyProcedure,
    user: User,
  ) -> anyhow::Result<CopyProcedureResponse> {
    let Procedure { config, .. } =
      resource::get_check_permissions::<Procedure>(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;
    resource::create::<Procedure>(&name, config.into(), &user).await
  }
}

impl Resolve<UpdateProcedure, User> for State {
  #[instrument(name = "UpdateProcedure", skip(self, user))]
  async fn resolve(
    &self,
    UpdateProcedure { id, config }: UpdateProcedure,
    user: User,
  ) -> anyhow::Result<UpdateProcedureResponse> {
    resource::update::<Procedure>(&id, config, &user).await
  }
}

impl Resolve<DeleteProcedure, User> for State {
  #[instrument(name = "DeleteProcedure", skip(self, user))]
  async fn resolve(
    &self,
    DeleteProcedure { id }: DeleteProcedure,
    user: User,
  ) -> anyhow::Result<DeleteProcedureResponse> {
    resource::delete::<Procedure>(&id, &user).await
  }
}
