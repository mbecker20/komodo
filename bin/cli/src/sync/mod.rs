use std::path::Path;

use colored::Colorize;
use monitor_client::entities::{
  alerter::Alerter, build::Build, builder::Builder,
  deployment::Deployment, procedure::Procedure, repo::Repo,
  server::Server, server_template::ServerTemplate,
};

use crate::{sync::resources::ResourceSync, wait_for_enter};

mod file;
mod resources;
mod user_group;
mod variables;

pub async fn run_sync(path: &Path) -> anyhow::Result<()> {
  info!(
    "resources path: {}",
    path.display().to_string().blue().bold()
  );

  let resources = file::read_resources(path)?;

  info!("computing sync actions...");

  let (
    (server_template_creates, server_template_updates),
    (server_creates, server_updates),
    (deployment_creates, deployment_updates),
    (build_creates, build_updates),
    (builder_creates, builder_updates),
    (alerter_creates, alerter_updates),
    (repo_creates, repo_updates),
    (procedure_creates, procedure_updates),
    (user_group_creates, user_group_updates),
    (variable_creates, variable_updates),
  ) = tokio::try_join!(
    ServerTemplate::get_updates(resources.server_templates),
    Server::get_updates(resources.servers),
    Deployment::get_updates(resources.deployments),
    Build::get_updates(resources.builds),
    Builder::get_updates(resources.builders),
    Alerter::get_updates(resources.alerters),
    Repo::get_updates(resources.repos),
    Procedure::get_updates(resources.procedures),
    user_group::get_updates(resources.user_groups),
    variables::get_updates(resources.variables),
  )?;

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
    && variable_creates.is_empty()
    && variable_updates.is_empty()
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
  variables::run_updates(variable_creates, variable_updates).await;
  user_group::run_updates(user_group_creates, user_group_updates)
    .await;

  Ok(())
}
