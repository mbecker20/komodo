use std::path::Path;

use anyhow::{anyhow, Context};
use command::run_monitor_command;
use formatting::format_serror;
use monitor_client::entities::{
  build::{ImageRegistry, StandardRegistryConfig},
  environment_vars_to_string,
  stack::Stack,
  to_monitor_name,
  update::Log,
  CloneArgs,
};
use periphery_client::api::{
  compose::ComposeResponse,
  git::{CloneRepo, RepoActionResponse},
};
use resolver_api::Resolve;
use run_command::async_run_command;
use tokio::fs;

use crate::{config::periphery_config, docker::docker_login, State};

const DEFAULT_FILE_NAME: &str = "compose.yaml";

pub fn maybe_timeout(timeout: Option<i32>) -> String {
  if let Some(timeout) = timeout {
    format!(" --timeout {timeout}")
  } else {
    String::new()
  }
}

/// Checks if theres a directory at stack_dir/stack_name from the deploy.
/// If not will take care of writing / cloning the file.
///
/// If its a deploy for a repo stack, will pull the repo
pub async fn run_compose_command(
  stack: &Stack,
  service: Option<&str>,
  git_token: Option<String>,
  registry_token: Option<String>,
  is_deploy: bool,
  stage: &str,
  command: &str,
) -> ComposeResponse {
  let mut res = ComposeResponse::default();

  let run_directory = periphery_config()
    .stack_dir
    .join(to_monitor_name(&stack.name))
    .join(&stack.config.run_directory);
  let file_path = run_directory.join(&stack.config.file_path);

  if is_deploy || !file_path.exists() {
    let git_token = match git_token {
      Some(token) => Some(token),
      None => {
        if !stack.config.git_account.is_empty() {
          match crate::helpers::git_token(
            &stack.config.git_provider,
            &stack.config.git_account,
          ) {
            Ok(token) => Some(token.to_string()),
            Err(e) => {
              res.logs.push(Log::error(
                "find git token",
                format_serror(&e.into()),
              ));
              return res;
            }
          }
        } else {
          None
        }
      }
    };
    // Write / clone the file
    if let Err(e) = write_stack(stack, git_token, &mut res).await {
      res.logs.push(Log::error(
        "write stack compose file",
        format_serror(
          &e.context("failed to write / clone compose file").into(),
        ),
      ));
      return res;
    }
  }

  let docker_compose = if periphery_config().legacy_compose_cli {
    "docker-compose"
  } else {
    "docker compose"
  };
  let run_dir = run_directory.display();
  let file = &stack.config.file_path;
  let service = service
    .map(|service| format!(" {service}"))
    .unwrap_or_default();

  if is_deploy {
    match fs::read_to_string(&file_path).await.with_context(|| {
      format!("failed to read compose file contents at {file_path:?}")
    }) {
      Ok(contents) => res.file_contents = Some(contents),
      Err(e) => {
        res.logs.push(Log::error(
          "read compose contents",
          format_serror(&e.into()),
        ));
        return res;
      }
    };
    // Login to the registry to pull private images if account is set
    if !stack.config.registry_account.is_empty() {
      let registry =
        ImageRegistry::Standard(StandardRegistryConfig {
          domain: stack.config.registry_provider.clone(),
          account: stack.config.registry_account.clone(),
          ..Default::default()
        });
      if let Err(e) =
        docker_login(&registry, registry_token.as_deref(), None)
          .await
          .with_context(|| {
            format!(
              "domain: {} | account: {}",
              stack.config.registry_provider,
              stack.config.registry_account
            )
          })
          .context("failed to login to image registry")
      {
        res.logs.push(Log::error(
          "login to registry",
          format_serror(&e.into()),
        ));
        return res;
      };
    }

    // Pull images before destroying to minimize downtime
    let log = run_monitor_command(
      "compose pull",
      format!(
        "cd {run_dir} && {docker_compose} -f {file} pull{service}",
      ),
    )
    .await;

    if !log.success {
      res.logs.push(log);
      return res;
    }

    // Maybe destroy existing
    async_run_command(&format!(
      "cd {run_dir} && {docker_compose} -f {file} down{service}",
    ))
    .await;
  }

  res.logs.push(
    run_monitor_command(
      stage,
      format!(
        "cd {run_dir} && {docker_compose} -f {file} {command}",
      ),
    )
    .await,
  );

  res
}

/// Either writes the stack file_contents to a file, or clones the repo.
/// Returns the run directory.
pub async fn write_stack(
  stack: &Stack,
  git_token: Option<String>,
  res: &mut ComposeResponse,
) -> anyhow::Result<()> {
  let root = periphery_config()
    .stack_dir
    .join(to_monitor_name(&stack.name));

  // Ensure directory is clear going in.
  fs::remove_dir_all(&root).await.ok();

  if stack.config.file_contents.is_empty() {
    // Clone the repo
    if stack.config.repo.is_empty() {
      return Err(anyhow!("Must either input compose file contents directly or provide a repo. Got neither."));
    }
    let mut args: CloneArgs = stack.into();
    // Set the clone destination to the one created for this run
    args.destination = Some(root.display().to_string());
    let RepoActionResponse {
      logs,
      commit_hash,
      commit_message,
    } = State
      .resolve(CloneRepo { args, git_token }, ())
      .await
      .context("failed to clone stack repo")?;

    res.logs.extend(logs);
    res.commit_hash = commit_hash;
    res.commit_message = commit_message;

    let run_directory = root.join(&stack.config.run_directory);

    if write_environment_file(stack, &run_directory, &mut res.logs)
      .await
      .is_err()
    {
      return Err(anyhow!("failed to write environment file"));
    };
    Ok(())
  } else {
    // Write the file
    fs::create_dir_all(&root).await.with_context(|| {
      format!("failed to create stack root directory at {root:?}")
    })?;
    if write_environment_file(stack, &root, &mut res.logs)
      .await
      .is_err()
    {
      return Err(anyhow!("failed to write environment file"));
    };
    fs::write(
      root.join(DEFAULT_FILE_NAME),
      &stack.config.file_contents,
    )
    .await
    .context("failed to write compose file")?;

    Ok(())
  }
}

/// Check that result is Ok before continuing.
pub async fn write_environment_file(
  stack: &Stack,
  folder: &Path,
  logs: &mut Vec<Log>,
) -> Result<(), ()> {
  if stack.config.environment.is_empty() {
    return Ok(());
  }

  let contents =
    environment_vars_to_string(&stack.config.environment);

  let contents = if stack.config.skip_secret_interp {
    contents
  } else {
    let res = svi::interpolate_variables(
      &contents,
      &periphery_config().secrets,
      svi::Interpolator::DoubleBrackets,
      true,
    )
    .context("failed to interpolate secrets into stack environment");

    let (contents, replacers) = match res {
      Ok(res) => res,
      Err(e) => {
        logs.push(Log::error(
          "interpolate periphery secrets",
          format_serror(&e.into()),
        ));
        return Err(());
      }
    };

    if !replacers.is_empty() {
      logs.push(Log::simple(
        "interpolate periphery secrets",
        replacers
            .iter()
            .map(|(_, variable)| format!("<span class=\"text-muted-foreground\">replaced:</span> {variable}"))
            .collect::<Vec<_>>()
            .join("\n"),
      ))
    }

    contents
  };

  let file = folder.join(&stack.config.env_file_path);

  if let Err(e) =
    fs::write(&file, contents).await.with_context(|| {
      format!("failed to write environment file to {file:?}")
    })
  {
    logs.push(Log::error(
      "write environment file",
      format_serror(&e.into()),
    ));
    return Err(());
  }

  logs.push(Log::simple(
    "write environment file",
    format!("environment written to {file:?}"),
  ));

  Ok(())
}
