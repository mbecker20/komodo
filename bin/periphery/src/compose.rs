use std::path::Path;

use anyhow::{anyhow, Context};
use command::run_monitor_command;
use formatting::format_serror;
use monitor_client::entities::{
  all_logs_success,
  build::{ImageRegistry, StandardRegistryConfig},
  environment_vars_to_string,
  stack::{ComposeFile, Stack},
  to_monitor_name,
  update::Log,
  CloneArgs,
};
use periphery_client::api::{
  compose::ComposeUpResponse,
  git::{CloneRepo, RepoActionResponse},
};
use regex::Regex;
use resolver_api::Resolve;
use tokio::fs;

use crate::{
  config::periphery_config,
  docker::{docker_client, docker_login},
  helpers::parse_extra_args,
  State,
};

pub fn docker_compose() -> &'static str {
  if periphery_config().legacy_compose_cli {
    "docker-compose"
  } else {
    "docker compose"
  }
}

// pub fn maybe_timeout(timeout: Option<i32>) -> String {
//   if let Some(timeout) = timeout {
//     format!(" --timeout {timeout}")
//   } else {
//     String::new()
//   }
// }

/// If Err, remember to write result to the log before return.
pub async fn compose_up(
  stack: Stack,
  service: Option<String>,
  git_token: Option<String>,
  registry_token: Option<String>,
  res: &mut ComposeUpResponse,
) -> anyhow::Result<()> {
  let run_directory = periphery_config()
    .stack_dir
    .join(to_monitor_name(&stack.name))
    .join(&stack.config.run_directory);
  let file_path = run_directory.join(&stack.config.file_path);
  // Store whether the file is healthy before clone.
  // If it is not healthy, the containers will need to be taken down manually.
  let file_healthy = file_path.exists();

  // Write the stack. For repos, will first delete existing folder to ensure fresh deploy.
  // Will also set additional fields on the reponse.
  write_stack(&stack, git_token, res)
    .await
    .context("failed to write / clone compose file")?;

  if !file_path.exists() {
    res.file_missing = true;
    return Err(anyhow!("Compose file doesn't exist after writing stack. Ensure the run_directory and file_path are correct."));
  }

  // Get the file contents
  let file_contents =
    match fs::read_to_string(&file_path).await.with_context(|| {
      format!("failed to read compose file contents at {file_path:?}")
    }) {
      Ok(res) => res,
      Err(e) => {
        let error = format_serror(&e.into());
        res
          .logs
          .push(Log::error("read compose file", error.clone()));
        // This should only happen for repo stacks, ie remote error
        res.remote_error = Some(error);
        return Err(anyhow!(
          "failed to read compose file, stopping run"
        ));
      }
    };
  res.file_contents = Some(file_contents.clone());

  let docker_compose = docker_compose();
  let run_dir = run_directory.display();
  let service_arg = service
    .as_ref()
    .map(|service| format!(" {service}"))
    .unwrap_or_default();
  let file = &stack.config.file_path;

  // Pull images before destroying to minimize downtime.
  // If this fails, do not continue.
  let log = run_monitor_command(
    "compose pull",
    format!(
      "cd {run_dir} && {docker_compose} -f {file} pull{service_arg}",
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

  // Take down the existing containers
  destroy_existing_containers(
    file_healthy,
    &run_directory,
    &stack.config.file_path,
    &file_contents,
    service,
    res,
  )
  .await
  .context("failed to destroy existing containers")?;

  // Run compose up
  let extra_args = parse_extra_args(&stack.config.extra_args);
  let log = run_monitor_command(
    "compose up",
    format!(
      "cd {run_dir} && {docker_compose} -f {file} up -d{extra_args}{service_arg}",
    ),
  )
  .await;
  res.deployed = log.success;
  res.logs.push(log);

  Ok(())
}

/// Either writes the stack file_contents to a file, or clones the repo.
/// Returns the run directory.
async fn write_stack(
  stack: &Stack,
  git_token: Option<String>,
  res: &mut ComposeUpResponse,
) -> anyhow::Result<()> {
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
              res.remote_error = Some(error);
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
        res.remote_error = Some(error);
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

    if write_environment_file(stack, &run_directory, &mut res.logs)
      .await
      .is_err()
    {
      return Err(anyhow!("failed to write environment file"));
    };
    Ok(())
  } else {
    // Ensure run directory exists
    fs::create_dir_all(&run_directory).await.with_context(|| {
      format!("failed to create stack run directory at {root:?}")
    })?;
    if write_environment_file(stack, &run_directory, &mut res.logs)
      .await
      .is_err()
    {
      return Err(anyhow!("failed to write environment file"));
    };
    let file = run_directory.join(&stack.config.file_path);
    fs::write(&file, &stack.config.file_contents)
      .await
      .with_context(|| {
        format!("failed to write compose file to {file:?}")
      })?;

    Ok(())
  }
}

/// Check that result is Ok before continuing.
async fn write_environment_file(
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

async fn destroy_existing_containers(
  file_healthy: bool,
  run_directory: &Path,
  file_path: &str,
  file_contents: &str,
  service: Option<String>,
  res: &mut ComposeUpResponse,
) -> anyhow::Result<()> {
  if file_healthy {
    // ########################
    // # Destroy with compose #
    // ########################
    let docker_compose = docker_compose();
    let run_dir = run_directory.display();
    let service_arg = service
      .as_ref()
      .map(|service| format!(" {service}"))
      .unwrap_or_default();
    let log = run_monitor_command(
      "destroy container",
      format!("cd {run_dir} && {docker_compose} -f {file_path} down{service_arg}"),
    )
    .await;
    let success = log.success;
    res.logs.push(log);
    if !success {
      return Err(anyhow!("Failed to bring down existing container(s) with docker compose down. Stopping run."));
    }
  } else {
    // #############################
    // # Destroy by container name #
    // #############################
    let containers = docker_client()
      .list_containers()
      .await
      .context("failed to get container list from the host")?;
    let compose = serde_yaml::from_str::<ComposeFile>(file_contents)
      .context(
        "failed to extract container names from compose file",
      )?;

    if let Some(service) = service {
      let config =
        compose.services.get(&service).with_context(|| {
          format!("did not find service {service} in compose file")
        })?;
      let name = if let Some(name) = &config.container_name {
        Some(name)
      } else {
        let regex = Regex::new(&format!("compose-{service}-[0-9]*$"))
          .context(
            "failed to construct service name matching regex",
          )?;
        containers
          .iter()
          .find(|container| regex.is_match(&container.name))
          .map(|container| &container.name)
      };
      if let Some(name) = name {
        let log = run_monitor_command(
          "destroy container",
          format!("docker stop {name} && docker container rm {name}"),
        )
        .await;
        if !log.success {
          res.logs.push(log);
          return Err(anyhow!(
            "Failed to destroy container {name}. Stopping."
          ));
        }
      }
    }

    let to_stop =
      compose.services.iter().filter_map(|(service, config)| {
        if let Some(name) = &config.container_name {
          Some(name)
        } else {
          let regex =
            Regex::new(&format!("compose-{service}-[0-9]*$")).ok()?;
          containers
            .iter()
            .find(|container| regex.is_match(&container.name))
            .map(|container| &container.name)
        }
      });
    for name in to_stop {
      let log = run_monitor_command(
        "destroy container",
        format!("docker stop {name} && docker container rm {name}"),
      )
      .await;
      if !log.success {
        res.logs.push(log);
        return Err(anyhow!(
          "Failed to destroy container {name}. Stopping."
        ));
      }
    }
  }

  Ok(())
}
