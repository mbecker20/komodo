use anyhow::Context;
use monitor_client::entities::{
  build::Build,
  deployment::Deployment,
  monitor_timestamp,
  procedure::Procedure,
  repo::Repo,
  server::Server,
  server_template::ServerTemplate,
  stack::Stack,
  sync::ResourceSync,
  update::{ResourceTarget, Update, UpdateListItem},
  user::User,
  Operation,
};
use mungos::{
  by_id::{find_one_by_id, update_one_by_id},
  mongodb::bson::to_document,
};

use crate::{
  api::execute::ExecuteRequest, resource, state::db_client,
};

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
pub async fn add_update(
  mut update: Update,
) -> anyhow::Result<String> {
  update.id = db_client()
    .await
    .updates
    .insert_one(&update)
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
    other_data: update.other_data,
    username,
  };
  Ok(update)
}

#[instrument(level = "debug")]
async fn send_update(update: UpdateListItem) -> anyhow::Result<()> {
  update_channel().sender.lock().await.send(update)?;
  Ok(())
}

pub async fn init_execution_update(
  request: &ExecuteRequest,
  user: &User,
) -> anyhow::Result<Update> {
  let (operation, target) = match &request {
    // Server
    ExecuteRequest::StopAllContainers(data) => (
      Operation::StopAllContainers,
      ResourceTarget::Server(
        resource::get::<Server>(&data.server).await?.id,
      ),
    ),
    ExecuteRequest::PruneContainers(data) => (
      Operation::PruneImages,
      ResourceTarget::Server(
        resource::get::<Server>(&data.server).await?.id,
      ),
    ),
    ExecuteRequest::PruneImages(data) => (
      Operation::PruneImages,
      ResourceTarget::Server(
        resource::get::<Server>(&data.server).await?.id,
      ),
    ),
    ExecuteRequest::PruneNetworks(data) => (
      Operation::PruneNetworks,
      ResourceTarget::Server(
        resource::get::<Server>(&data.server).await?.id,
      ),
    ),

    // Deployment
    ExecuteRequest::Deploy(data) => (
      Operation::Deploy,
      ResourceTarget::Deployment(
        resource::get::<Deployment>(&data.deployment).await?.id,
      ),
    ),
    ExecuteRequest::StartContainer(data) => (
      Operation::StartContainer,
      ResourceTarget::Deployment(
        resource::get::<Deployment>(&data.deployment).await?.id,
      ),
    ),
    ExecuteRequest::RestartContainer(data) => (
      Operation::RestartContainer,
      ResourceTarget::Deployment(
        resource::get::<Deployment>(&data.deployment).await?.id,
      ),
    ),
    ExecuteRequest::PauseContainer(data) => (
      Operation::PauseContainer,
      ResourceTarget::Deployment(
        resource::get::<Deployment>(&data.deployment).await?.id,
      ),
    ),
    ExecuteRequest::UnpauseContainer(data) => (
      Operation::UnpauseContainer,
      ResourceTarget::Deployment(
        resource::get::<Deployment>(&data.deployment).await?.id,
      ),
    ),
    ExecuteRequest::StopContainer(data) => (
      Operation::StopContainer,
      ResourceTarget::Deployment(
        resource::get::<Deployment>(&data.deployment).await?.id,
      ),
    ),
    ExecuteRequest::RemoveContainer(data) => (
      Operation::RemoveContainer,
      ResourceTarget::Deployment(
        resource::get::<Deployment>(&data.deployment).await?.id,
      ),
    ),

    // Build
    ExecuteRequest::RunBuild(data) => (
      Operation::RunBuild,
      ResourceTarget::Build(
        resource::get::<Build>(&data.build).await?.id,
      ),
    ),
    ExecuteRequest::CancelBuild(data) => (
      Operation::CancelBuild,
      ResourceTarget::Build(
        resource::get::<Build>(&data.build).await?.id,
      ),
    ),

    // Repo
    ExecuteRequest::CloneRepo(data) => (
      Operation::CloneRepo,
      ResourceTarget::Repo(
        resource::get::<Repo>(&data.repo).await?.id,
      ),
    ),
    ExecuteRequest::PullRepo(data) => (
      Operation::PullRepo,
      ResourceTarget::Repo(
        resource::get::<Repo>(&data.repo).await?.id,
      ),
    ),

    // Procedure
    ExecuteRequest::RunProcedure(data) => (
      Operation::RunProcedure,
      ResourceTarget::Procedure(
        resource::get::<Procedure>(&data.procedure).await?.id,
      ),
    ),

    // Server template
    ExecuteRequest::LaunchServer(data) => (
      Operation::LaunchServer,
      ResourceTarget::ServerTemplate(
        resource::get::<ServerTemplate>(&data.server_template)
          .await?
          .id,
      ),
    ),

    // Resource Sync
    ExecuteRequest::RunSync(data) => (
      Operation::RunSync,
      ResourceTarget::ResourceSync(
        resource::get::<ResourceSync>(&data.sync).await?.id,
      ),
    ),

    // Stack
    ExecuteRequest::DeployStack(data) => (
      Operation::DeployStack,
      ResourceTarget::Stack(
        resource::get::<Stack>(&data.stack).await?.id,
      ),
    ),
    ExecuteRequest::StartStack(data) => (
      if data.service.is_some() {
        Operation::StartStackService
      } else {
        Operation::StartStack
      },
      ResourceTarget::Stack(
        resource::get::<Stack>(&data.stack).await?.id,
      ),
    ),
    ExecuteRequest::RestartStack(data) => (
      if data.service.is_some() {
        Operation::RestartStackService
      } else {
        Operation::RestartStack
      },
      ResourceTarget::Stack(
        resource::get::<Stack>(&data.stack).await?.id,
      ),
    ),
    ExecuteRequest::PauseStack(data) => (
      if data.service.is_some() {
        Operation::PauseStackService
      } else {
        Operation::PauseStack
      },
      ResourceTarget::Stack(
        resource::get::<Stack>(&data.stack).await?.id,
      ),
    ),
    ExecuteRequest::UnpauseStack(data) => (
      if data.service.is_some() {
        Operation::UnpauseStackService
      } else {
        Operation::UnpauseStack
      },
      ResourceTarget::Stack(
        resource::get::<Stack>(&data.stack).await?.id,
      ),
    ),
    ExecuteRequest::StopStack(data) => (
      if data.service.is_some() {
        Operation::StopStackService
      } else {
        Operation::StopStack
      },
      ResourceTarget::Stack(
        resource::get::<Stack>(&data.stack).await?.id,
      ),
    ),
    ExecuteRequest::DestroyStack(data) => (
      Operation::DestroyStack,
      ResourceTarget::Stack(
        resource::get::<Stack>(&data.stack).await?.id,
      ),
    ),
  };
  let mut update = make_update(target, operation, user);
  update.in_progress();
  update.id = add_update(update.clone()).await?;
  Ok(update)
}
