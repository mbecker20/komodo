use anyhow::{anyhow, Context};
use axum::async_trait;
use monitor_client::{
  api::read::{ListUserPermissions, ListUserPermissionsResponse},
  entities::user::User,
};
use mungos::{find::find_collect, mongodb::bson::doc};
use resolver_api::Resolve;

use crate::{db::db_client, state::State};

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
      doc! { "user_id": user_id },
      None,
    )
    .await
    .context("failed to query db for permissions")
  }
}
