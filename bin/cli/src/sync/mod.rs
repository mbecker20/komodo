use std::path::Path;

use colored::Colorize;
use monitor_client::entities::{
  alerter::Alerter, build::Build, builder::Builder,
  deployment::Deployment, procedure::Procedure, repo::Repo,
  server::Server, server_template::ServerTemplate,
};

use crate::{sync::resources::ResourceSync, wait_for_enter};

mod resource_file;
mod resources;
mod user_group;

pub async fn run_sync(path: &Path) -> anyhow::Result<()> {
  info!(
    "resources path: {}",
    path.display().to_string().blue().bold()
  );

  let resources = resource_file::read_resources(path)?;

  info!("computing sync actions...");

  let (server_template_creates, server_template_updates) =
    ServerTemplate::get_updates(resources.server_templates).await?;
  let (server_creates, server_updates) =
    Server::get_updates(resources.servers).await?;
  let (deployment_creates, deployment_updates) =
    Deployment::get_updates(resources.deployments).await?;
  let (build_creates, build_updates) =
    Build::get_updates(resources.builds).await?;
  let (builder_creates, builder_updates) =
    Builder::get_updates(resources.builders).await?;
  let (alerter_creates, alerter_updates) =
    Alerter::get_updates(resources.alerters).await?;
  let (repo_creates, repo_updates) =
    Repo::get_updates(resources.repos).await?;
  let (procedure_creates, procedure_updates) =
    Procedure::get_updates(resources.procedures).await?;
  let (user_group_creates, user_group_updates) =
    user_group::get_updates(resources.user_groups).await?;

  if server_template_creates.is_empty()
    && server_template_updates.is_empty()
    && server_creates.is_empty()
    && server_updates.is_empty()
    && deployment_creates.is_empty()
    && deployment_updates.is_empty()
    && build_creates.is_empty()
    && build_updates.is_empty()
    && builder_creates.is_empty()
    && builder_updates.is_empty()
    && alerter_creates.is_empty()
    && alerter_updates.is_empty()
    && repo_creates.is_empty()
    && repo_updates.is_empty()
    && procedure_creates.is_empty()
    && procedure_updates.is_empty()
    && user_group_creates.is_empty()
    && user_group_updates.is_empty()
  {
    info!("{}. exiting.", "nothing to do".green().bold());
    return Ok(());
  }

  wait_for_enter("run sync")?;

  // No deps
  ServerTemplate::run_updates(
    server_template_creates,
    server_template_updates,
  )
  .await;
  Server::run_updates(server_creates, server_updates).await;
  Alerter::run_updates(alerter_creates, alerter_updates).await;

  // Dependant on server
  Builder::run_updates(builder_creates, builder_updates).await;
  Repo::run_updates(repo_creates, repo_updates).await;

  // Dependant on builder
  Build::run_updates(build_creates, build_updates).await;

  // Dependant on server / builder
  Deployment::run_updates(deployment_creates, deployment_updates)
    .await;

  // Dependant on everything
  Procedure::run_updates(procedure_creates, procedure_updates).await;
  user_group::run_updates(user_group_creates, user_group_updates)
    .await;

  Ok(())
}
