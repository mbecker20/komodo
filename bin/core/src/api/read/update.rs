use std::collections::HashMap;

use anyhow::{anyhow, Context};
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
    update::{
      ResourceTarget, ResourceTargetVariant, Update, UpdateListItem,
    },
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
  config::core_config,
  helpers::query::get_resource_ids_for_non_admin,
  resource,
  state::{db_client, State},
};

const UPDATES_PER_PAGE: i64 = 100;

impl Resolve<ListUpdates, User> for State {
  async fn resolve(
    &self,
    ListUpdates { query, page }: ListUpdates,
    user: User,
  ) -> anyhow::Result<ListUpdatesResponse> {
    let query = if user.admin || core_config().transparent_mode {
      query
    } else {
      let server_ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Server,
      )
      .await?;
      let deployment_ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Deployment,
      )
      .await?;
      let build_ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Build,
      )
      .await?;
      let repo_ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Repo,
      )
      .await?;
      let procedure_ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Procedure,
      )
      .await?;
      let builder_ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Builder,
      )
      .await?;
      let alerter_ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Alerter,
      )
      .await?;
      let server_template_ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::ServerTemplate,
      )
      .await?;

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
          { "target.type": "ServerTemplate", "target.id": { "$in": &server_template_ids } },
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
        other_data: u.other_data,
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
    if user.admin || core_config().transparent_mode {
      return Ok(update);
    }
    match &update.target {
      ResourceTarget::System(_) => {
        return Err(anyhow!(
          "user must be admin to view system updates"
        ))
      }
      ResourceTarget::Server(id) => {
        resource::get_check_permissions::<Server>(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Deployment(id) => {
        resource::get_check_permissions::<Deployment>(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Build(id) => {
        resource::get_check_permissions::<Build>(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Repo(id) => {
        resource::get_check_permissions::<Repo>(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Builder(id) => {
        resource::get_check_permissions::<Builder>(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Alerter(id) => {
        resource::get_check_permissions::<Alerter>(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::Procedure(id) => {
        resource::get_check_permissions::<Procedure>(
          id,
          &user,
          PermissionLevel::Read,
        )
        .await?;
      }
      ResourceTarget::ServerTemplate(id) => {
        resource::get_check_permissions::<ServerTemplate>(
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
