use anyhow::Context;
use monitor_client::{
  api::execute::RunSync,
  entities::{
    permission::PermissionLevel, sync::ResourceSync, update::Update,
    user::User,
  },
};
use resolver_api::Resolve;

use crate::{helpers::update::update_update, resource, state::State};

impl Resolve<RunSync, (User, Update)> for State {
  async fn resolve(
    &self,
    RunSync { sync }: RunSync,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let sync = resource::get_check_permissions::<ResourceSync>(
      &sync,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    let (res, logs) =
      crate::helpers::sync::get_remote_resources(&sync)
        .await
        .context("failed to get remote resources")?;

    update.logs.extend(logs);
    update_update(update.clone()).await?;

    let resources = res?;

    todo!()
  }
}
