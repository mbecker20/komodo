use std::str::FromStr;

use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::{
    GetUserGroup, GetUserGroupResponse, ListUserGroups,
    ListUserGroupsResponse,
  },
  entities::user::User,
};
use mungos::{
  find::find_collect,
  mongodb::bson::{doc, oid::ObjectId, Document},
};
use resolver_api::Resolve;

use crate::state::{db_client, State};

#[async_trait]
impl Resolve<GetUserGroup, User> for State {
  async fn resolve(
    &self,
    GetUserGroup { user_group }: GetUserGroup,
    user: User,
  ) -> anyhow::Result<GetUserGroupResponse> {
    let mut filter = match ObjectId::from_str(&user_group) {
      Ok(id) => doc! { "_id": id },
      Err(_) => doc! { "name": &user_group },
    };
    // Don't allow non admin users to get UserGroups they aren't a part of.
    if !user.admin {
      // Filter for only UserGroups which contain the users id
      filter.insert("users", &user.id);
    }
    db_client()
      .await
      .user_groups
      .find_one(filter, None)
      .await
      .context("failed to query db for user groups")?
      .context("no UserGroup found with given name or id")
  }
}

#[async_trait]
impl Resolve<ListUserGroups, User> for State {
  async fn resolve(
    &self,
    ListUserGroups {}: ListUserGroups,
    user: User,
  ) -> anyhow::Result<ListUserGroupsResponse> {
    let mut filter = Document::new();
    if !user.admin {
      filter.insert("users", &user.id);
    }
    find_collect(&db_client().await.user_groups, filter, None)
      .await
      .context("failed to query db for UserGroups")
  }
}
