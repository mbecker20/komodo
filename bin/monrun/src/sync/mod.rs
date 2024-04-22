use std::path::Path;

use monitor_client::entities::{
  alerter::Alerter, build::Build, builder::Builder,
  deployment::Deployment, procedure::Procedure, repo::Repo,
  server::Server,
};

use crate::{sync::resources::ResourceSync, wait_for_enter};

mod resource_file;
mod resources;
mod user_group;

pub async fn run_sync(path: &Path) -> anyhow::Result<()> {
  info!("path: {path:?}");

  let resources = resource_file::read_resources(path)?;

  println!("{resources:#?}");

  let (server_updates, server_creates) =
    Server::get_updates(resources.servers)?;
  let (deployment_updates, deployment_creates) =
    Deployment::get_updates(resources.deployments)?;
  let (build_updates, build_creates) =
    Build::get_updates(resources.builds)?;
  let (builder_updates, builder_creates) =
    Builder::get_updates(resources.builders)?;
  let (alerter_updates, alerter_creates) =
    Alerter::get_updates(resources.alerters)?;
  let (repo_updates, repo_creates) =
    Repo::get_updates(resources.repos)?;
  let (procedure_updates, procedure_creates) =
    Procedure::get_updates(resources.procedures)?;

  wait_for_enter("CONTINUE")?;

  // No deps
  Server::run_updates(server_updates, server_creates).await;
  Alerter::run_updates(alerter_updates, alerter_creates).await;

  // Dependant on server
  Builder::run_updates(builder_updates, builder_creates).await;
  Repo::run_updates(repo_updates, repo_creates).await;

  // Dependant on builder
  Build::run_updates(build_updates, build_creates).await;

  // Dependant on server / builder
  Deployment::run_updates(deployment_updates, deployment_creates)
    .await;

  // Dependant on everything
  Procedure::run_updates(procedure_updates, procedure_creates).await;

  Ok(())
}
