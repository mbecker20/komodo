use anyhow::{anyhow, Context};
use axum::async_trait;
use monitor_client::{
  api::read::{
    GetPermissionLevel, GetPermissionLevelResponse, ListPermissions,
    ListPermissionsResponse, ListUserPermissions,
    ListUserPermissionsResponse,
  },
  entities::user::User,
};
use mungos::{find::find_collect, mongodb::bson::doc};
use resolver_api::Resolve;

use crate::{
  db::db_client, helpers::resource::get_user_permission_on_resource,
  state::State,
};

#[async_trait]
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

#[async_trait]
impl Resolve<GetPermissionLevel, User> for State {
  async fn resolve(
    &self,
    GetPermissionLevel { target }: GetPermissionLevel,
    user: User,
  ) -> anyhow::Result<GetPermissionLevelResponse> {
    let (variant, id) = target.extract_variant_id();
    get_user_permission_on_resource(&user.id, variant, id).await
  }
}

#[async_trait]
impl Resolve<ListUserPermissions, User> for State {
  async fn resolve(
    &self,
    ListUserPermissions { user_id }: ListUserPermissions,
    user: User,
  ) -> anyhow::Result<ListUserPermissionsResponse> {
    if !user.admin {
      return Err(anyhow!("this method is admin only"));
    }
    find_collect(
      &db_client().await.permissions,
      doc! {
        "user_target.type": "User",
        "user_target.id": user_id
      },
      None,
    )
    .await
    .context("failed to query db for permissions")
  }
}
