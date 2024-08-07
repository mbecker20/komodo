use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use command::run_monitor_command;
use formatting::format_serror;
use monitor_client::entities::{
  all_logs_success,
  build::{ImageRegistry, StandardRegistryConfig},
  environment_vars_to_string,
  stack::{ComposeContents, Stack},
  to_monitor_name,
  update::Log,
  CloneArgs,
};
use periphery_client::api::{
  compose::ComposeUpResponse,
  git::{CloneRepo, RepoActionResponse},
};
use resolver_api::Resolve;
use tokio::fs;

use crate::{
  config::periphery_config, docker::docker_login,
  helpers::parse_extra_args, State,
};

pub fn docker_compose() -> &'static str {
  if periphery_config().legacy_compose_cli {
    "docker-compose"
  } else {
    "docker compose"
  }
}

/// If Err, remember to write result to the log before return.
pub async fn compose_up(
  stack: Stack,
  service: Option<String>,
  git_token: Option<String>,
  registry_token: Option<String>,
  res: &mut ComposeUpResponse,
) -> anyhow::Result<()> {
  // Write the stack to local disk. For repos, will first delete any existing folder to ensure fresh deploy.
  // Will also set additional fields on the reponse.
  // Use the env_file_path in the compose command.
  let env_file_path = write_stack(&stack, git_token, res)
    .await
    .context("failed to write / clone compose file")?;

  let root = periphery_config()
    .stack_dir
    .join(to_monitor_name(&stack.name));
  let run_directory = root.join(&stack.config.run_directory);
  let run_directory = run_directory
    .canonicalize()
    .context("failed to canonicalize run directory on host")?;

  let file_paths = stack
    .file_paths()
    .iter()
    .map(|path| (path, run_directory.join(path)))
    .collect::<Vec<_>>();

  for (path, full_path) in &file_paths {
    if !full_path.exists() {
      res.missing_files.push(path.to_string());
    }
  }
  if !res.missing_files.is_empty() {
    return Err(anyhow!("A compose file doesn't exist after writing stack. Ensure the run_directory and file_paths are correct."));
  }

  for (path, full_path) in &file_paths {
    let file_contents =
      match fs::read_to_string(&full_path).await.with_context(|| {
        format!(
          "failed to read compose file contents at {full_path:?}"
        )
      }) {
        Ok(res) => res,
        Err(e) => {
          let error = format_serror(&e.into());
          res
            .logs
            .push(Log::error("read compose file", error.clone()));
          // This should only happen for repo stacks, ie remote error
          res.remote_errors.push(ComposeContents {
            path: path.to_string(),
            contents: error,
          });
          return Err(anyhow!(
          "failed to read compose file at {full_path:?}, stopping run"
        ));
        }
      };
    res.file_contents.push(ComposeContents {
      path: full_path.display().to_string(),
      contents: file_contents,
    });
  }

  let docker_compose = docker_compose();
  let run_dir = run_directory
    .canonicalize()
    .context("failed to canonicalize run directory on host")?;
  let run_dir = run_dir.display();
  let service_arg = service
    .as_ref()
    .map(|service| format!(" {service}"))
    .unwrap_or_default();
  let file_args = if stack.config.file_paths.is_empty() {
    String::from("compose.yaml")
  } else {
    stack.config.file_paths.join(" -f ")
  };
  let last_project_name = stack.project_name(false);
  let project_name = stack.project_name(true);

  // Pull images before destroying to minimize downtime.
  // If this fails, do not continue.
  let log = run_monitor_command(
    "compose pull",
    format!(
      "cd {run_dir} && {docker_compose} -p {project_name} -f {file_args} pull{service_arg}",
    ),
  )
  .await;
  if !log.success {
    res.logs.push(log);
    return Err(anyhow!(
      "Failed to pull required images, stopping the run."
    ));
  }

  // Login to the registry to pull private images, if account is set
  if !stack.config.registry_account.is_empty() {
    let registry = ImageRegistry::Standard(StandardRegistryConfig {
      domain: stack.config.registry_provider.clone(),
      account: stack.config.registry_account.clone(),
      ..Default::default()
    });
    docker_login(&registry, registry_token.as_deref(), None)
      .await
      .with_context(|| {
        format!(
          "domain: {} | account: {}",
          stack.config.registry_provider,
          stack.config.registry_account
        )
      })
      .context("failed to login to image registry")?;
  }

  // Take down the existing containers.
  // This one tries to use the previously deployed service name, to ensure the right stack is taken down.
  destroy_existing_containers(&last_project_name, service, res)
    .await
    .context("failed to destroy existing containers")?;

  // Run compose up
  let extra_args = parse_extra_args(&stack.config.extra_args);
  let env_file = env_file_path
    .map(|path| format!(" --env-file {}", path.display()))
    .unwrap_or_default();
  let log = run_monitor_command(
    "compose up",
    format!(
      "cd {run_dir} && {docker_compose} -p {project_name} -f {file_args}{env_file} up -d{extra_args}{service_arg}",
    ),
  )
  .await;
  res.deployed = log.success;
  res.logs.push(log);

  if let Err(e) = fs::remove_dir_all(&root).await.with_context(|| {
    format!("failed to clean up files after deploy | path: {root:?}")
  }) {
    res
      .logs
      .push(Log::error("clean up files", format_serror(&e.into())))
  }

  Ok(())
}

/// Either writes the stack file_contents to a file, or clones the repo.
/// Returns the env file path, to maybe include in command with --env-file.
async fn write_stack(
  stack: &Stack,
  git_token: Option<String>,
  res: &mut ComposeUpResponse,
) -> anyhow::Result<Option<PathBuf>> {
  let root = periphery_config()
    .stack_dir
    .join(to_monitor_name(&stack.name));
  let run_directory = root.join(&stack.config.run_directory);

  if stack.config.file_contents.is_empty() {
    // Clone the repo
    if stack.config.repo.is_empty() {
      // Err response will be written to return, no need to add it to log here
      return Err(anyhow!("Must either input compose file contents directly or provide a repo. Got neither."));
    }
    let mut args: CloneArgs = stack.into();
    // Set the clone destination to the one created for this run
    args.destination = Some(root.display().to_string());

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
              let error = format_serror(&e.into());
              res
                .logs
                .push(Log::error("no git token", error.clone()));
              res.remote_errors.push(ComposeContents {
                path: Default::default(),
                contents: error,
              });
              return Err(anyhow!(
                "failed to find required git token, stopping run"
              ));
            }
          }
        } else {
          None
        }
      }
    };

    // Ensure directory is clear going in.
    fs::remove_dir_all(&root).await.ok();

    let RepoActionResponse {
      logs,
      commit_hash,
      commit_message,
    } = match State.resolve(CloneRepo { args, git_token }, ()).await {
      Ok(res) => res,
      Err(e) => {
        let error = format_serror(
          &e.context("failed to clone stack repo").into(),
        );
        res.logs.push(Log::error("clone stack repo", error.clone()));
        res.remote_errors.push(ComposeContents {
          path: Default::default(),
          contents: error,
        });
        return Err(anyhow!(
          "failed to clone stack repo, stopping run"
        ));
      }
    };

    res.logs.extend(logs);
    res.commit_hash = commit_hash;
    res.commit_message = commit_message;

    if !all_logs_success(&res.logs) {
      return Err(anyhow!("Stopped after clone failure"));
    }

    let env_file_path = match write_environment_file(
      stack,
      &run_directory,
      &mut res.logs,
    )
    .await
    {
      Ok(path) => path,
      Err(_) => {
        return Err(anyhow!("failed to write environment file"));
      }
    };
    Ok(env_file_path)
  } else {
    // Ensure run directory exists
    fs::create_dir_all(&run_directory).await.with_context(|| {
      format!("failed to create stack run directory at {root:?}")
    })?;
    let env_file_path = match write_environment_file(
      stack,
      &run_directory,
      &mut res.logs,
    )
    .await
    {
      Ok(path) => path,
      Err(_) => {
        return Err(anyhow!("failed to write environment file"));
      }
    };
    let file_path = run_directory.join(
      stack
        .config
        .file_paths
        // only need the first one, or default
        .first()
        .map(String::as_str)
        .unwrap_or("compose.yaml"),
    );
    fs::write(&file_path, &stack.config.file_contents)
      .await
      .with_context(|| {
        format!("failed to write compose file to {file_path:?}")
      })?;

    Ok(env_file_path)
  }
}

/// If the environment was written and needs to be passed to the compose command,
/// will return the env file PathBuf
async fn write_environment_file(
  stack: &Stack,
  folder: &Path,
  logs: &mut Vec<Log>,
) -> Result<Option<PathBuf>, ()> {
  if stack.config.environment.is_empty() {
    return Ok(None);
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

  Ok(Some(file))
}

async fn destroy_existing_containers(
  project: &str,
  service: Option<String>,
  res: &mut ComposeUpResponse,
) -> anyhow::Result<()> {
  let docker_compose = docker_compose();
  let service_arg = service
    .as_ref()
    .map(|service| format!(" {service}"))
    .unwrap_or_default();
  let log = run_monitor_command(
    "destroy container",
    format!("{docker_compose} -p {project} down{service_arg}"),
  )
  .await;
  let success = log.success;
  res.logs.push(log);
  if !success {
    return Err(anyhow!("Failed to bring down existing container(s) with docker compose down. Stopping run."));
  }

  Ok(())
}
