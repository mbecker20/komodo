use std::path::Path;

use monitor_client::entities::{
  alerter::Alerter, build::Build, builder::Builder,
  deployment::Deployment, repo::Repo, server::Server,
};

use crate::wait_for_enter;

mod resource_file;
mod sync_trait;

use sync_trait::Sync;

pub async fn run_sync(path: &Path) -> anyhow::Result<()> {
  info!("path: {path:?}");

  let resources = resource_file::read_resources(path)?;

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

  wait_for_enter("CONTINUE")?;

  Build::run_updates(build_updates, build_creates).await;
  Server::run_updates(server_updates, server_creates).await;
  Deployment::run_updates(deployment_updates, deployment_creates)
    .await;
  Builder::run_updates(builder_updates, builder_creates).await;
  Alerter::run_updates(alerter_updates, alerter_creates).await;
  Repo::run_updates(repo_updates, repo_creates).await;

  Ok(())
}
