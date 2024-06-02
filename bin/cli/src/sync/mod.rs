use colored::Colorize;
use monitor_client::entities::{
  alerter::Alerter, build::Build, builder::Builder,
  deployment::Deployment, procedure::Procedure, repo::Repo,
  server::Server, server_template::ServerTemplate,
};
use resource::ResourceSync;

use crate::{helpers::wait_for_enter, state::cli_args};

mod file;
mod resource;
mod resources;
mod user_group;
mod variables;

pub async fn run(path: &str, delete: bool) -> anyhow::Result<()> {
  info!("resources path: {}", path.blue().bold());
  if delete {
    warn!("Delete mode {}", "enabled".bold());
  }

  let resources = file::read_resources(path)?;

  info!("computing sync actions...");

  let (
    server_template_creates,
    server_template_updates,
    server_template_deletes,
  ) = resource::get_updates::<ServerTemplate>(
    resources.server_templates,
    delete,
  )?;
  let (server_creates, server_updates, server_deletes) =
    resource::get_updates::<Server>(resources.servers, delete)?;
  let (deployment_creates, deployment_updates, deployment_deletes) =
    resource::get_updates::<Deployment>(
      resources.deployments,
      delete,
    )?;
  let (build_creates, build_updates, build_deletes) =
    resource::get_updates::<Build>(resources.builds, delete)?;
  let (builder_creates, builder_updates, builder_deletes) =
    resource::get_updates::<Builder>(resources.builders, delete)?;
  let (alerter_creates, alerter_updates, alerter_deletes) =
    resource::get_updates::<Alerter>(resources.alerters, delete)?;
  let (repo_creates, repo_updates, repo_deletes) =
    resource::get_updates::<Repo>(resources.repos, delete)?;
  let (procedure_creates, procedure_updates, procedure_deletes) =
    resource::get_updates::<Procedure>(resources.procedures, delete)?;

  let (variable_creates, variable_updates, variable_deletes) =
    variables::get_updates(resources.variables, delete)?;

  let (user_group_creates, user_group_updates, user_group_deletes) =
    user_group::get_updates(resources.user_groups, delete).await?;

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
    && user_group_deletes.is_empty()
    && variable_creates.is_empty()
    && variable_updates.is_empty()
    && variable_deletes.is_empty()
  {
    info!("{}. exiting.", "nothing to do".green().bold());
    return Ok(());
  }

  if !cli_args().yes {
    wait_for_enter("run sync")?;
  }

  // No deps
  ServerTemplate::run_updates(
    server_template_creates,
    server_template_updates,
    server_template_deletes,
  )
  .await;
  Server::run_updates(server_creates, server_updates, server_deletes)
    .await;
  Alerter::run_updates(
    alerter_creates,
    alerter_updates,
    alerter_deletes,
  )
  .await;

  // Dependant on server
  Builder::run_updates(
    builder_creates,
    builder_updates,
    builder_deletes,
  )
  .await;
  Repo::run_updates(repo_creates, repo_updates, repo_deletes).await;

  // Dependant on builder
  Build::run_updates(build_creates, build_updates, build_deletes)
    .await;

  // Dependant on server / builder
  Deployment::run_updates(
    deployment_creates,
    deployment_updates,
    deployment_deletes,
  )
  .await;

  // Dependant on everything
  Procedure::run_updates(
    procedure_creates,
    procedure_updates,
    procedure_deletes,
  )
  .await;
  variables::run_updates(
    variable_creates,
    variable_updates,
    variable_deletes,
  )
  .await;
  user_group::run_updates(
    user_group_creates,
    user_group_updates,
    user_group_deletes,
  )
  .await;

  Ok(())
}
