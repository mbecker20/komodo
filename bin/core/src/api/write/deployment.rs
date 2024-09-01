use anyhow::{anyhow, Context};
use komodo_client::{
  api::write::*,
  entities::{
    deployment::{Deployment, DeploymentState},
    komodo_timestamp,
    permission::PermissionLevel,
    server::Server,
    to_komodo_name,
    update::Update,
    user::User,
    Operation,
  },
};
use mungos::{by_id::update_one_by_id, mongodb::bson::doc};
use periphery_client::api;
use resolver_api::Resolve;

use crate::{
  helpers::{
    periphery_client,
    query::get_deployment_state,
    update::{add_update, make_update},
  },
  resource,
  state::{action_states, db_client, State},
};

impl Resolve<CreateDeployment, User> for State {
  #[instrument(name = "CreateDeployment", skip(self, user))]
  async fn resolve(
    &self,
    CreateDeployment { name, config }: CreateDeployment,
    user: User,
  ) -> anyhow::Result<Deployment> {
    resource::create::<Deployment>(&name, config, &user).await
  }
}

impl Resolve<CopyDeployment, User> for State {
  #[instrument(name = "CopyDeployment", skip(self, user))]
  async fn resolve(
    &self,
    CopyDeployment { name, id }: CopyDeployment,
    user: User,
  ) -> anyhow::Result<Deployment> {
    let Deployment { config, .. } =
      resource::get_check_permissions::<Deployment>(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;
    resource::create::<Deployment>(&name, config.into(), &user).await
  }
}

impl Resolve<DeleteDeployment, User> for State {
  #[instrument(name = "DeleteDeployment", skip(self, user))]
  async fn resolve(
    &self,
    DeleteDeployment { id }: DeleteDeployment,
    user: User,
  ) -> anyhow::Result<Deployment> {
    resource::delete::<Deployment>(&id, &user).await
  }
}

impl Resolve<UpdateDeployment, User> for State {
  #[instrument(name = "UpdateDeployment", skip(self, user))]
  async fn resolve(
    &self,
    UpdateDeployment { id, config }: UpdateDeployment,
    user: User,
  ) -> anyhow::Result<Deployment> {
    resource::update::<Deployment>(&id, config, &user).await
  }
}

impl Resolve<RenameDeployment, User> for State {
  #[instrument(name = "RenameDeployment", skip(self, user))]
  async fn resolve(
    &self,
    RenameDeployment { id, name }: RenameDeployment,
    user: User,
  ) -> anyhow::Result<Update> {
    let deployment = resource::get_check_permissions::<Deployment>(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    // get the action state for the deployment (or insert default).
    let action_state = action_states()
      .deployment
      .get_or_insert_default(&deployment.id)
      .await;

    // Will check to ensure deployment not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.renaming = true)?;

    let name = to_komodo_name(&name);

    let container_state = get_deployment_state(&deployment).await?;

    if container_state == DeploymentState::Unknown {
      return Err(anyhow!(
        "cannot rename deployment when container status is unknown"
      ));
    }

    let mut update =
      make_update(&deployment, Operation::RenameDeployment, &user);

    update_one_by_id(
      &db_client().await.deployments,
      &deployment.id,
      mungos::update::Update::Set(
        doc! { "name": &name, "updated_at": komodo_timestamp() },
      ),
      None,
    )
    .await
    .context("failed to update deployment name on db")?;

    if container_state != DeploymentState::NotDeployed {
      let server =
        resource::get::<Server>(&deployment.config.server_id).await?;
      let log = periphery_client(&server)?
        .request(api::container::RenameContainer {
          curr_name: deployment.name.clone(),
          new_name: name.clone(),
        })
        .await
        .context("failed to rename container on server")?;
      update.logs.push(log);
    }

    update.push_simple_log(
      "rename deployment",
      format!(
        "renamed deployment from {} to {}",
        deployment.name, name
      ),
    );
    update.finalize();

    add_update(update.clone()).await?;

    Ok(update)
  }
}
