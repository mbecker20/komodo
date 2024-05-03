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
    permission::PermissionLevel,
    procedure::Procedure,
    repo::Repo,
    server::Server,
    server_template::ServerTemplate,
    update::{ResourceTarget, Update, UpdateListItem},
    user::User,
  },
};
use mungos::{
  by_id::find_one_by_id,
  find::find_collect,
  mongodb::{bson::doc, options::FindOptions},
};
use resolver_api::Resolve;

use crate::{
  helpers::resource::StateResource,
  state::{db_client, State},
};

const UPDATES_PER_PAGE: i64 = 20;

#[async_trait]
impl Resolve<ListUpdates, User> for State {
  async fn resolve(
    &self,
    ListUpdates { query, page }: ListUpdates,
    user: User,
  ) -> anyhow::Result<ListUpdatesResponse> {
    let query = if user.admin {
      query
    } else {
      let server_ids =
        Server::get_resource_ids_for_non_admin(&user.id).await?;
      let deployment_ids =
        Deployment::get_resource_ids_for_non_admin(&user.id).await?;
      let build_ids =
        Build::get_resource_ids_for_non_admin(&user.id).await?;
      let repo_ids =
        Repo::get_resource_ids_for_non_admin(&user.id).await?;
      let procedure_ids =
        Procedure::get_resource_ids_for_non_admin(&user.id).await?;
      let builder_ids =
        Builder::get_resource_ids_for_non_admin(&user.id).await?;
      let alerter_ids =
        Alerter::get_resource_ids_for_non_admin(&user.id).await?;
      let mut query = query.unwrap_or_default();
      query.extend(doc! {
        "$or": [
          { "target.type": "Server", "target.id": { "$in": &server_ids } },
          { "target.type": "Deployment", "target.id": { "$in": &deployment_ids } },
          { "target.type": "Build", "target.id": { "$in": &build_ids } },
          { "target.type": "Repo", "target.id": { "$in": &repo_ids } },
          { "target.type": "Procedure", "target.id": { "$in": &procedure_ids } },
          { "target.type": "Builder", "target.id": { "$in": &builder_ids } },
          { "target.type": "Alerter", "target.id": { "$in": &alerter_ids } },
        ]
      });
      query.into()
    };

    let usernames =
      find_collect(&db_client().await.users, None, None)
        .await
        .context("failed to pull users from db")?
        .into_iter()
        .map(|u| (u.id, u.username))
        .collect::<HashMap<_, _>>();

    let updates = find_collect(
      &db_client().await.updates,
      query,
      FindOptions::builder()
        .sort(doc! { "start_ts": -1 })
        .skip(page as u64 * UPDATES_PER_PAGE as u64)
        .limit(UPDATES_PER_PAGE)
        .build(),
    )
    .await
    .context("failed to pull updates from db")?
    .into_iter()
    .map(|u| {
      let username = if User::is_service_user(&u.operator) {
        u.operator.clone()
      } else {
        usernames
          .get(&u.operator)
          .cloned()
          .unwrap_or("unknown".to_string())
      };
      UpdateListItem {
        username,
        id: u.id,
        operation: u.operation,
        start_ts: u.start_ts,
        success: u.success,
        operator: u.operator,
        target: u.target,
        status: u.status,
        version: u.version,
      }
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
impl Resolve<GetUpdate, User> for State {
  async fn resolve(
    &self,
    GetUpdate { id }: GetUpdate,
    user: User,
  ) -> anyhow::Result<Update> {
    let update = find_one_by_id(&db_client().await.updates, &id)
      .await
      .context("failed to query to db")?
      .context("no update exists with given id")?;
    if user.admin {
      return Ok(update);
    }
    match &update.target {
      ResourceTarget::System(_) => {
        return Err(anyhow!(
          "user must be admin to view system updates"
        ))
      }
      ResourceTarget::Server(id) => {
        Server::get_resource_check_permissions(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Deployment(id) => {
        Deployment::get_resource_check_permissions(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Build(id) => {
        Build::get_resource_check_permissions(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Repo(id) => {
        Repo::get_resource_check_permissions(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Builder(id) => {
        Builder::get_resource_check_permissions(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Alerter(id) => {
        Alerter::get_resource_check_permissions(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Procedure(id) => {
        Procedure::get_resource_check_permissions(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::ServerTemplate(id) => {
        ServerTemplate::get_resource_check_permissions(
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
