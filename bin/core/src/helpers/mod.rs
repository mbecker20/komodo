use std::{str::FromStr, time::Duration};

use anyhow::{anyhow, Context};
use axum::{headers::ContentType, http::StatusCode, TypedHeader};
use monitor_types::{
  entities::{
    deployment::{Deployment, DockerContainerState},
    server::{Server, ServerStatus},
    tag::CustomTag,
    update::{ResourceTarget, Update, UpdateListItem},
    user::User,
    Operation,
  },
  monitor_timestamp,
};
use mungos::{
  by_id::find_one_by_id,
  mongodb::bson::{doc, oid::ObjectId, to_bson},
};
use periphery_client::{requests, PeripheryClient};
use rand::{thread_rng, Rng};
use serror::serialize_error_pretty;

use crate::{auth::RequestUser, state::State};

use self::resource::StateResource;

pub mod alert;
pub mod cache;
pub mod channel;
pub mod resource;

pub fn empty_or_only_spaces(word: &str) -> bool {
  if word.is_empty() {
    return true;
  }
  for char in word.chars() {
    if char != ' ' {
      return false;
    }
  }
  true
}

pub fn random_duration(min_ms: u64, max_ms: u64) -> Duration {
  Duration::from_millis(thread_rng().gen_range(min_ms..max_ms))
}

pub fn make_update(
  target: impl Into<ResourceTarget>,
  operation: Operation,
  user: &RequestUser,
) -> Update {
  Update {
    start_ts: monitor_timestamp(),
    target: target.into(),
    operation,
    operator: user.id.clone(),
    success: true,
    ..Default::default()
  }
}

pub fn into_response_error(
  e: anyhow::Error,
) -> (StatusCode, TypedHeader<ContentType>, String) {
  (
    StatusCode::INTERNAL_SERVER_ERROR,
    TypedHeader(ContentType::json()),
    serialize_error_pretty(e),
  )
}

impl State {
  pub async fn get_user(
    &self,
    user_id: &str,
  ) -> anyhow::Result<User> {
    find_one_by_id(&self.db.users, user_id)
      .await
      .context("failed to query mongo for user")?
      .with_context(|| format!("no user found with id {user_id}"))
  }

  pub async fn get_server_with_status(
    &self,
    server_id: &str,
  ) -> anyhow::Result<(Server, ServerStatus)> {
    let server: Server = self.get_resource(server_id).await?;
    if !server.config.enabled {
      return Ok((server, ServerStatus::Disabled));
    }
    let status = match self
      .periphery_client(&server)?
      .request(requests::GetHealth {})
      .await
    {
      Ok(_) => ServerStatus::Ok,
      Err(_) => ServerStatus::NotOk,
    };
    Ok((server, status))
  }

  pub async fn get_deployment_state(
    &self,
    deployment: &Deployment,
  ) -> anyhow::Result<DockerContainerState> {
    if deployment.config.server_id.is_empty() {
      return Ok(DockerContainerState::NotDeployed);
    }
    let (server, status) = self
      .get_server_with_status(&deployment.config.server_id)
      .await?;
    if status != ServerStatus::Ok {
      return Ok(DockerContainerState::Unknown);
    }
    let container = self
      .periphery_client(&server)?
      .request(requests::GetContainerList {})
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

  pub async fn get_tag(
    &self,
    tag_id: &str,
  ) -> anyhow::Result<CustomTag> {
    find_one_by_id(&self.db.tags, tag_id)
      .await
      .context("failed to query mongo for tag")?
      .with_context(|| format!("no tag found with id {tag_id}"))
  }

  pub async fn get_tag_check_owner(
    &self,
    tag_id: &str,
    user: &RequestUser,
  ) -> anyhow::Result<CustomTag> {
    let tag = self.get_tag(tag_id).await?;
    if !user.is_admin && tag.owner != user.id {
      return Err(anyhow!("user must be tag owner or admin"));
    }
    Ok(tag)
  }

  // UPDATE

  async fn update_list_item(
    &self,
    update: Update,
  ) -> anyhow::Result<UpdateListItem> {
    let username = find_one_by_id(&self.db.users, &update.operator)
      .await
      .context("failed to query mongo for user")?
      .with_context(|| {
        format!("no user found with id {}", update.operator)
      })?
      .username;
    let update = UpdateListItem {
      id: update.id,
      operation: update.operation,
      start_ts: update.start_ts,
      success: update.success,
      operator: update.operator,
      target: update.target,
      status: update.status,
      version: update.version,
      username,
    };
    Ok(update)
  }

  async fn send_update(
    &self,
    update: UpdateListItem,
  ) -> anyhow::Result<()> {
    self.update.sender.lock().await.send(update)?;
    Ok(())
  }

  pub async fn add_update(
    &self,
    mut update: Update,
  ) -> anyhow::Result<String> {
    update.id = self
      .db
      .updates
      .insert_one(&update, None)
      .await
      .context("failed to insert update into db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not object id")?
      .to_string();
    let id = update.id.clone();
    let update = self.update_list_item(update).await?;
    let _ = self.send_update(update).await;
    Ok(id)
  }

  pub async fn update_update(
    &self,
    update: Update,
  ) -> anyhow::Result<()> {
    self.db
      .updates
      .update_one(doc! { "_id": ObjectId::from_str(&update.id)? }, doc! { "$set": to_bson(&update)? }, None)
      .await
      .context("failed to update the update on db. the update build process was deleted")?;
    let update = self.update_list_item(update).await?;
    let _ = self.send_update(update).await;
    Ok(())
  }

  pub async fn remove_from_recently_viewed(
    &self,
    resource: impl Into<ResourceTarget>,
  ) -> anyhow::Result<()> {
    let resource: ResourceTarget = resource.into();
    self.db
      .users
      .update_many(
          doc! {},
          doc! {
              "$pull": {
                  "recently_viewed":
                      to_bson(&resource).context("failed to convert resource to bson")?
              }
          },
          None
      )
      .await
      .context("failed to remove resource from users recently viewed")?;
    Ok(())
  }

  //

  pub fn periphery_client(
    &self,
    server: &Server,
  ) -> anyhow::Result<PeripheryClient> {
    if !server.config.enabled {
      return Err(anyhow!("server not enabled"));
    }

    let client = PeripheryClient::new(
      &server.config.address,
      &self.config.passkey,
    );

    Ok(client)
  }
}
