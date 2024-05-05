use std::path::Path;

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
  info!("path: {path:?}");

  let resources = resource_file::read_resources(path)?;

  println!("{resources:#?}");

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

  wait_for_enter("CONTINUE")?;

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
