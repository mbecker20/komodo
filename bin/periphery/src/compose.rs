use anyhow::{anyhow, Context};
use command::run_monitor_command;
use formatting::format_serror;
use monitor_client::entities::{
  all_logs_success,
  build::{ImageRegistry, StandardRegistryConfig},
  stack::Stack,
  update::Log,
  CloneArgs,
};
use periphery_client::api::{
  compose::ComposeUpResponse, git::CloneRepo,
};
use resolver_api::Resolve;
use run_command::async_run_command;
use tokio::fs;

use crate::{
  config::periphery_config, docker::docker_login,
  helpers::random_string, State,
};

const DEFAULT_FILE_NAME: &str = "compose.yaml";

pub fn docker_compose() -> &'static str {
  if periphery_config().legacy_compose_cli {
    "docker-compose"
  } else {
    "docker compose"
  }
}

pub fn maybe_timeout(timeout: Option<i32>) -> String {
  if let Some(timeout) = timeout {
    format!(" --timeout {timeout}")
  } else {
    String::new()
  }
}

pub fn maybe_args(args: &str) -> String {
  if args.is_empty() {
    String::new()
  } else {
    format!(" {args}")
  }
}

pub async fn compose_up(
  stack: Stack,
  git_token: Option<String>,
  registry_token: Option<String>,
  service: Option<&str>,
) -> anyhow::Result<ComposeUpResponse> {
  let mut logs = Vec::new();

  let folder = periphery_config().repo_dir.join(random_string(10));

  let (run_directory, file_path, commit_hash, commit_message) =
    if stack.config.file_contents.is_empty() {
      // try repo to stack contents
      if stack.config.repo.is_empty() {
        return Err(anyhow!("Must either input compose file contents directly or provide a repo. Got neither."));
      }
      let mut args: CloneArgs = (&stack).into();
      // Set the clone destination to the one created for this run
      args.destination = Some(folder.display().to_string());
      let res = State
        .resolve(CloneRepo { args, git_token }, ())
        .await
        .context("failed to clone compose repo")?;
      logs.extend(res.logs);

      // Failed to clone repo
      if !all_logs_success(&logs) {
        return Ok(ComposeUpResponse {
          logs,
          file_contents: None,
          commit_hash: None,
          commit_message: None,
        });
      }

      let run_directory = folder.join(&stack.config.run_directory);

      (
        run_directory,
        stack.config.file_path.as_str(),
        res.commit_hash,
        res.commit_message,
      )
    } else {
      // Use file_contents directly on the stack
      let file_path = folder.join(DEFAULT_FILE_NAME);
      fs::write(&file_path, &stack.config.file_contents)
        .await
        .context("failed to write compose file")?;
      (folder.clone(), DEFAULT_FILE_NAME, None, None)
    };

  let run_directory = run_directory
    .canonicalize()
    .context("failed to get absolute path of run directory")?;

  let file_contents =
    fs::read_to_string(run_directory.join(file_path))
      .await
      .context("failed to read compose file contents")?;

  // Login to the registry to pull private images if account is set
  if !stack.config.registry_account.is_empty() {
    let registry = ImageRegistry::Standard(StandardRegistryConfig {
      domain: stack.config.registry_provider,
      account: stack.config.registry_account,
      ..Default::default()
    });
    docker_login(&registry, registry_token.as_deref(), None)
      .await
      .context("failed to login to image registry")?;
  }

  let run_directory = run_directory.display();
  let docker_compose = docker_compose();
  let args = maybe_args(&stack.config.deploy_args);
  let service = service
    .map(|service| format!(" {service}"))
    .unwrap_or_default();

  // Pull images before destroying to minimize downtime
  let log = run_monitor_command(
    "compose pull",
    format!(
      "cd {run_directory} && {docker_compose} -f {file_path} pull{service}",
    ),
  )
  .await;

  // Early exit if fail to pre pull required images.
  if !log.success {
    logs.push(log);
    return Ok(ComposeUpResponse {
      logs,
      file_contents: Some(file_contents),
      commit_hash,
      commit_message,
    });
  }

  // Maybe destroy existing compose
  async_run_command(&format!(
    "cd {run_directory} && {docker_compose} -f {file_path} down{service}",
  ))
  .await;

  // Deploy the compose file
  logs.push(
    run_monitor_command(
      "compose up",
      format!(
        "cd {run_directory} && {docker_compose} -f {file_path} up -d{args}{service}",
      ),
    )
    .await,
  );

  // Remove the folder afterwards
  if let Err(e) = fs::remove_dir_all(&folder)
    .await
    .with_context(|| format!("directory: {folder:?}"))
    .context("failed to remove compose directory")
  {
    error!("{e:#}");
    logs
      .push(Log::error("post run cleanup", format_serror(&e.into())));
  }

  Ok(ComposeUpResponse {
    logs,
    file_contents: Some(file_contents),
    commit_hash,
    commit_message,
  })
}
