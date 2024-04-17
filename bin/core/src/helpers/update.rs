use anyhow::Context;
use monitor_client::entities::{
  monitor_timestamp,
  update::{ResourceTarget, Update, UpdateListItem},
  user::User,
  Operation,
};
use mungos::{
  by_id::{find_one_by_id, update_one_by_id},
  mongodb::bson::to_document,
};

use crate::db::db_client;

use super::channel::update_channel;

pub fn make_update(
  target: impl Into<ResourceTarget>,
  operation: Operation,
  user: &User,
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

#[instrument(level = "debug")]
async fn update_list_item(
  update: Update,
) -> anyhow::Result<UpdateListItem> {
  let username = if User::is_service_user(&update.operator) {
    update.operator.clone()
  } else {
    find_one_by_id(&db_client().await.users, &update.operator)
      .await
      .context("failed to query mongo for user")?
      .with_context(|| {
        format!("no user found with id {}", update.operator)
      })?
      .username
  };
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

#[instrument(level = "debug")]
async fn send_update(update: UpdateListItem) -> anyhow::Result<()> {
  update_channel().sender.lock().await.send(update)?;
  Ok(())
}

#[instrument(level = "debug")]
pub async fn add_update(
  mut update: Update,
) -> anyhow::Result<String> {
  update.id = db_client()
    .await
    .updates
    .insert_one(&update, None)
    .await
    .context("failed to insert update into db")?
    .inserted_id
    .as_object_id()
    .context("inserted_id is not object id")?
    .to_string();
  let id = update.id.clone();
  let update = update_list_item(update).await?;
  let _ = send_update(update).await;
  Ok(id)
}

#[instrument(level = "debug")]
pub async fn update_update(update: Update) -> anyhow::Result<()> {
  update_one_by_id(&db_client().await.updates, &update.id, mungos::update::Update::Set(to_document(&update)?), None)
      .await
      .context("failed to update the update on db. the update build process was deleted")?;
  let update = update_list_item(update).await?;
  let _ = send_update(update).await;
  Ok(())
}
