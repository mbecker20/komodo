use anyhow::{anyhow, Context};
use monitor_client::{
  api::read::{
    GetPermissionLevel, GetPermissionLevelResponse, ListPermissions,
    ListPermissionsResponse, ListUserTargetPermissions,
    ListUserTargetPermissionsResponse,
  },
  entities::{permission::PermissionLevel, user::User},
};
use mungos::{find::find_collect, mongodb::bson::doc};
use resolver_api::Resolve;

use crate::{
  helpers::query::get_user_permission_on_resource,
  state::{db_client, State},
};

impl Resolve<ListPermissions, User> for State {
  async fn resolve(
    &self,
    ListPermissions {}: ListPermissions,
    user: User,
  ) -> anyhow::Result<ListPermissionsResponse> {
    find_collect(
      &db_client().await.permissions,
      doc! {
        "user_target.type": "User",
        "user_target.id": &user.id
      },
      None,
    )
    .await
    .context("failed to query db for permissions")
  }
}

impl Resolve<GetPermissionLevel, User> for State {
  async fn resolve(
    &self,
    GetPermissionLevel { target }: GetPermissionLevel,
    user: User,
  ) -> anyhow::Result<GetPermissionLevelResponse> {
    if user.admin {
      return Ok(PermissionLevel::Write);
    }
    let (variant, id) = target.extract_variant_id();
    get_user_permission_on_resource(&user, variant, id).await
  }
}

impl Resolve<ListUserTargetPermissions, User> for State {
  async fn resolve(
    &self,
    ListUserTargetPermissions { user_target }: ListUserTargetPermissions,
    user: User,
  ) -> anyhow::Result<ListUserTargetPermissionsResponse> {
    if !user.admin {
      return Err(anyhow!("this method is admin only"));
    }
    let (variant, id) = user_target.extract_variant_id();
    find_collect(
      &db_client().await.permissions,
      doc! {
        "user_target.type": variant.as_ref(),
        "user_target.id": id
      },
      None,
    )
    .await
    .context("failed to query db for permissions")
  }
}
