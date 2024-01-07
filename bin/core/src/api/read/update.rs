use std::collections::HashMap;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::read::{GetUpdate, ListUpdates, ListUpdatesResponse},
  entities::{
    alerter::Alerter,
    build::Build,
    builder::Builder,
    deployment::Deployment,
    repo::Repo,
    server::Server,
    update::{ResourceTarget, Update, UpdateListItem},
    PermissionLevel,
  },
};
use mungos::{
  by_id::find_one_by_id,
  find::find_collect,
  mongodb::{bson::doc, options::FindOptions},
};
use resolver_api::Resolve;

use crate::{
  auth::RequestUser, helpers::resource::StateResource, state::State,
};

const UPDATES_PER_PAGE: i64 = 20;

#[async_trait]
impl Resolve<ListUpdates, RequestUser> for State {
  async fn resolve(
    &self,
    ListUpdates { query, page }: ListUpdates,
    user: RequestUser,
  ) -> anyhow::Result<ListUpdatesResponse> {
    let query = if user.is_admin {
      query
    } else {
      let server_ids =
                <State as StateResource<Server>>::get_resource_ids_for_non_admin(self, &user.id)
                    .await?;
      let deployment_ids = <State as StateResource<
                Deployment,
            >>::get_resource_ids_for_non_admin(
                self, &user.id
            )
            .await?;
      let build_ids =
                <State as StateResource<Build>>::get_resource_ids_for_non_admin(self, &user.id)
                    .await?;
      let repo_ids =
                <State as StateResource<Repo>>::get_resource_ids_for_non_admin(self, &user.id)
                    .await?;
      let builder_ids =
                <State as StateResource<Builder>>::get_resource_ids_for_non_admin(self, &user.id)
                    .await?;
      let alerter_ids =
                <State as StateResource<Alerter>>::get_resource_ids_for_non_admin(self, &user.id)
                    .await?;
      let mut query = query.unwrap_or_default();
      query.extend(doc! {
                "$or": [
                   { "target.type": "Server", "target.id": { "$in": &server_ids } },
                   { "target.type": "Deployment", "target.id": { "$in": &deployment_ids } },
                   { "target.type": "Build", "target.id": { "$in": &build_ids } },
                   { "target.type": "Repo", "target.id": { "$in": &repo_ids } },
                   { "target.type": "Builder", "target.id": { "$in": &builder_ids } },
                   { "target.type": "Alerter", "target.id": { "$in": &alerter_ids } },
                ]
            });
      query.into()
    };

    let usernames = find_collect(&self.db.users, None, None)
      .await
      .context("failed to pull users from db")?
      .into_iter()
      .map(|u| (u.id, u.username))
      .collect::<HashMap<_, _>>();

    let updates = find_collect(
      &self.db.updates,
      query,
      FindOptions::builder()
        .sort(doc! { "start_ts": -1 })
        .skip(page as u64 * UPDATES_PER_PAGE as u64)
        .limit(UPDATES_PER_PAGE)
        .build(),
    )
    .await?
    .into_iter()
    .map(|u| UpdateListItem {
      id: u.id,
      operation: u.operation,
      start_ts: u.start_ts,
      success: u.success,
      username: usernames
        .get(&u.operator)
        .cloned()
        .unwrap_or("unknown".to_string()),
      operator: u.operator,
      target: u.target,
      status: u.status,
      version: u.version,
    })
    .collect::<Vec<_>>();

    let next_page = if updates.len() == UPDATES_PER_PAGE as usize {
      Some(page + 1)
    } else {
      None
    };

    Ok(ListUpdatesResponse { updates, next_page })
  }
}

#[async_trait]
impl Resolve<GetUpdate, RequestUser> for State {
  async fn resolve(
    &self,
    GetUpdate { id }: GetUpdate,
    user: RequestUser,
  ) -> anyhow::Result<Update> {
    let update = find_one_by_id(&self.db.updates, &id)
      .await
      .context("failed to query to db")?
      .context("no update exists with given id")?;
    if user.is_admin {
      return Ok(update);
    }
    match &update.target {
      ResourceTarget::System(_) => {
        return Err(anyhow!(
          "user must be admin to view system updates"
        ))
      }
      ResourceTarget::Server(id) => {
        let _: Server = self
          .get_resource_check_permissions(
            id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
      }
      ResourceTarget::Deployment(id) => {
        let _: Deployment = self
          .get_resource_check_permissions(
            id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
      }
      ResourceTarget::Build(id) => {
        let _: Build = self
          .get_resource_check_permissions(
            id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
      }
      ResourceTarget::Repo(id) => {
        let _: Repo = self
          .get_resource_check_permissions(
            id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
      }
      ResourceTarget::Builder(id) => {
        let _: Builder = self
          .get_resource_check_permissions(
            id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
      }
      ResourceTarget::Alerter(id) => {
        let _: Alerter = self
          .get_resource_check_permissions(
            id,
            &user,
            PermissionLevel::Read,
          )
          .await?;
      }
    }
    Ok(update)
  }
}
