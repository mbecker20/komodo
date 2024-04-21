use std::str::FromStr;

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  deployment::{Deployment, DockerContainerState},
  server::{Server, ServerStatus},
  tag::Tag,
  user::{admin_service_user, User},
};
use mungos::{
  by_id::find_one_by_id,
  find::find_collect,
  mongodb::bson::{doc, oid::ObjectId, Document},
};

use crate::db::db_client;

use super::resource::StateResource;

#[instrument(level = "debug")]
pub async fn get_user(user_id: &str) -> anyhow::Result<User> {
  if let Some(user) = admin_service_user(user_id) {
    return Ok(user);
  }
  find_one_by_id(&db_client().await.users, user_id)
    .await
    .context("failed to query mongo for user")?
    .with_context(|| format!("no user found with id {user_id}"))
}

#[instrument(level = "debug")]
pub async fn get_server_with_status(
  server_id_or_name: &str,
) -> anyhow::Result<(Server, ServerStatus)> {
  let server = Server::get_resource(server_id_or_name).await?;
  if !server.config.enabled {
    return Ok((server, ServerStatus::Disabled));
  }
  let status = match super::periphery_client(&server)?
    .request(periphery_client::api::GetHealth {})
    .await
  {
    Ok(_) => ServerStatus::Ok,
    Err(_) => ServerStatus::NotOk,
  };
  Ok((server, status))
}

#[instrument(level = "debug")]
pub async fn get_deployment_state(
  deployment: &Deployment,
) -> anyhow::Result<DockerContainerState> {
  if deployment.config.server_id.is_empty() {
    return Ok(DockerContainerState::NotDeployed);
  }
  let (server, status) =
    get_server_with_status(&deployment.config.server_id).await?;
  if status != ServerStatus::Ok {
    return Ok(DockerContainerState::Unknown);
  }
  let container = super::periphery_client(&server)?
    .request(periphery_client::api::container::GetContainerList {})
    .await?
    .into_iter()
    .find(|container| container.name == deployment.name);

  let state = match container {
    Some(container) => container.state,
    None => DockerContainerState::NotDeployed,
  };

  Ok(state)
}

// TAG

#[instrument(level = "debug")]
pub async fn get_tag(id_or_name: &str) -> anyhow::Result<Tag> {
  let query = match ObjectId::from_str(id_or_name) {
    Ok(id) => doc! { "_id": id },
    Err(_) => doc! { "name": id_or_name },
  };
  db_client()
    .await
    .tags
    .find_one(query, None)
    .await
    .context("failed to query mongo for tag")?
    .with_context(|| format!("no tag found matching {id_or_name}"))
}

#[instrument(level = "debug")]
pub async fn get_tag_check_owner(
  id_or_name: &str,
  user: &User,
) -> anyhow::Result<Tag> {
  let tag = get_tag(id_or_name).await?;
  if user.admin || tag.owner == user.id {
    return Ok(tag);
  }
  Err(anyhow!("user must be tag owner or admin"))
}

#[instrument(level = "debug")]
pub async fn get_user_user_group_ids(
  user_id: &str,
) -> anyhow::Result<Vec<String>> {
  let res = find_collect(
    &db_client().await.user_groups,
    doc! {
      "users": user_id
    },
    None,
  )
  .await
  .context("failed to query db for user groups")?
  .into_iter()
  .map(|ug| ug.id)
  .collect();
  Ok(res)
}

#[instrument(level = "debug")]
pub async fn user_target_query(
  user_id: &str,
) -> anyhow::Result<Vec<Document>> {
  let mut user_target_query = vec![
    doc! { "user_target.type": "User", "user_target.id": user_id },
  ];
  let user_groups = get_user_user_group_ids(user_id)
    .await?
    .into_iter()
    .map(|ug_id| {
      doc! {
        "user_target.type": "UserGroup", "user_target.id": ug_id,
      }
    });
  user_target_query.extend(user_groups);
  Ok(user_target_query)
}
