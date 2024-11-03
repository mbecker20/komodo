use std::{fmt::Write, path::PathBuf};

use anyhow::{anyhow, Context};
use command::run_komodo_command;
use formatting::format_serror;
use git::{write_commit_file, GitRes};
use komodo_client::entities::{
  stack::ComposeProject, to_komodo_name, update::Log, FileContents,
};
use periphery_client::api::{compose::*, git::RepoActionResponse};
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
  compose::{compose_up, docker_compose, write_stack, WriteStackRes},
  config::periphery_config,
  docker::docker_login,
  helpers::{log_grep, pull_or_clone_stack},
  State,
};

impl Resolve<ListComposeProjects, ()> for State {
  #[instrument(name = "ComposeInfo", level = "debug", skip(self))]
  async fn resolve(
    &self,
    ListComposeProjects {}: ListComposeProjects,
    _: (),
  ) -> anyhow::Result<Vec<ComposeProject>> {
    let docker_compose = docker_compose();
    let res = run_komodo_command(
      "list projects",
      None,
      format!("{docker_compose} ls --all --format json"),
      false,
    )
    .await;

    if !res.success {
      return Err(anyhow!("{}", res.combined()).context(format!(
        "failed to list compose projects using {docker_compose} ls"
      )));
    }

    let res =
      serde_json::from_str::<Vec<DockerComposeLsItem>>(&res.stdout)
        .with_context(|| res.stdout.clone())
        .with_context(|| {
          format!(
            "failed to parse '{docker_compose} ls' response to json"
          )
        })?
        .into_iter()
        .filter(|item| !item.name.is_empty())
        .map(|item| ComposeProject {
          name: item.name,
          status: item.status,
          compose_files: item
            .config_files
            .split(',')
            .map(str::to_string)
            .collect(),
        })
        .collect();

    Ok(res)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerComposeLsItem {
  #[serde(default, alias = "Name")]
  pub name: String,
  #[serde(alias = "Status")]
  pub status: Option<String>,
  /// Comma seperated list of paths
  #[serde(default, alias = "ConfigFiles")]
  pub config_files: String,
}

//

impl Resolve<GetComposeServiceLog> for State {
  #[instrument(
    name = "GetComposeServiceLog",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    GetComposeServiceLog {
      project,
      service,
      tail,
      timestamps,
    }: GetComposeServiceLog,
    _: (),
  ) -> anyhow::Result<Log> {
    let docker_compose = docker_compose();
    let timestamps =
      timestamps.then_some(" --timestamps").unwrap_or_default();
    let command = format!(
      "{docker_compose} -p {project} logs {service} --tail {tail}{timestamps}"
    );
    Ok(
      run_komodo_command("get stack log", None, command, false).await,
    )
  }
}

impl Resolve<GetComposeServiceLogSearch> for State {
  #[instrument(
    name = "GetComposeServiceLogSearch",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    GetComposeServiceLogSearch {
      project,
      service,
      terms,
      combinator,
      invert,
      timestamps,
    }: GetComposeServiceLogSearch,
    _: (),
  ) -> anyhow::Result<Log> {
    let docker_compose = docker_compose();
    let grep = log_grep(&terms, combinator, invert);
    let timestamps =
      timestamps.then_some(" --timestamps").unwrap_or_default();
    let command = format!("{docker_compose} -p {project} logs {service} --tail 5000{timestamps} 2>&1 | {grep}");
    Ok(
      run_komodo_command("get stack log grep", None, command, false)
        .await,
    )
  }
}

//

impl Resolve<GetComposeContentsOnHost, ()> for State {
  #[instrument(
    name = "GetComposeContentsOnHost",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    GetComposeContentsOnHost {
      name,
      run_directory,
      file_paths,
    }: GetComposeContentsOnHost,
    _: (),
  ) -> anyhow::Result<GetComposeContentsOnHostResponse> {
    let root =
      periphery_config().stack_dir.join(to_komodo_name(&name));
    let run_directory =
      root.join(&run_directory).components().collect::<PathBuf>();

    if !run_directory.exists() {
      fs::create_dir_all(&run_directory)
        .await
        .context("Failed to initialize run directory")?;
    }

    let mut res = GetComposeContentsOnHostResponse::default();

    for path in file_paths {
      let full_path =
        run_directory.join(&path).components().collect::<PathBuf>();
      match fs::read_to_string(&full_path).await.with_context(|| {
        format!(
          "Failed to read compose file contents at {full_path:?}"
        )
      }) {
        Ok(contents) => {
          // The path we store here has to be the same as incoming file path in the array,
          // in order for WriteComposeContentsToHost to write to the correct path.
          res.contents.push(FileContents { path, contents });
        }
        Err(e) => {
          res.errors.push(FileContents {
            path,
            contents: format_serror(&e.into()),
          });
        }
      }
    }

    Ok(res)
  }
}

//

impl Resolve<WriteComposeContentsToHost> for State {
  #[instrument(name = "WriteComposeContentsToHost", skip(self))]
  async fn resolve(
    &self,
    WriteComposeContentsToHost {
      name,
      run_directory,
      file_path,
      contents,
    }: WriteComposeContentsToHost,
    _: (),
  ) -> anyhow::Result<Log> {
    let file_path = periphery_config()
      .stack_dir
      .join(to_komodo_name(&name))
      .join(&run_directory)
      .join(file_path)
      .components()
      .collect::<PathBuf>();
    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
      let _ = fs::create_dir_all(&parent).await;
    }
    fs::write(&file_path, contents).await.with_context(|| {
      format!(
        "Failed to write compose file contents to {file_path:?}"
      )
    })?;
    Ok(Log::simple(
      "Write contents to host",
      format!("File contents written to {file_path:?}"),
    ))
  }
}

//

impl Resolve<WriteCommitComposeContents> for State {
  #[instrument(name = "WriteCommitComposeContents", skip(self))]
  async fn resolve(
    &self,
    WriteCommitComposeContents {
      stack,
      username,
      file_path,
      contents,
      git_token,
    }: WriteCommitComposeContents,
    _: (),
  ) -> anyhow::Result<RepoActionResponse> {
    let root = pull_or_clone_stack(&stack, git_token).await?;

    let file_path = stack
      .config
      .run_directory
      .parse::<PathBuf>()
      .context("Run directory is not a valid path")?
      .join(&file_path);

    let msg = if let Some(username) = username {
      format!("{}: Write Compose File", username)
    } else {
      "Write Compose File".to_string()
    };

    let GitRes {
      logs,
      hash,
      message,
      ..
    } = write_commit_file(&msg, &root, &file_path, &contents).await?;

    Ok(RepoActionResponse {
      logs,
      commit_hash: hash,
      commit_message: message,
      env_file_path: None,
    })
  }
}

//

impl<'a> WriteStackRes for &'a mut ComposePullResponse {
  fn logs(&mut self) -> &mut Vec<Log> {
    &mut self.logs
  }
}

impl Resolve<ComposePull> for State {
  #[instrument(
    name = "ComposePull",
    skip(self, git_token, registry_token)
  )]
  async fn resolve(
    &self,
    ComposePull {
      stack,
      service,
      git_token,
      registry_token,
    }: ComposePull,
    _: (),
  ) -> anyhow::Result<ComposePullResponse> {
    let mut res = ComposePullResponse::default();
    let (run_directory, env_file_path) =
      write_stack(&stack, git_token, &mut res).await?;

    // Canonicalize the path to ensure it exists, and is the cleanest path to the run directory.
    let run_directory = run_directory.canonicalize().context(
    "Failed to validate run directory on host after stack write (canonicalize error)",
  )?;

    let file_paths = stack
      .file_paths()
      .iter()
      .map(|path| {
        (
          path,
          // This will remove any intermediate uneeded '/./' in the path
          run_directory.join(path).components().collect::<PathBuf>(),
        )
      })
      .collect::<Vec<_>>();

    for (path, full_path) in &file_paths {
      if !full_path.exists() {
        return Err(anyhow!("Missing compose file at {path}"));
      }
    }

    let docker_compose = docker_compose();
    let service_arg = service
      .as_ref()
      .map(|service| format!(" {service}"))
      .unwrap_or_default();

    let file_args = if stack.config.file_paths.is_empty() {
      String::from("compose.yaml")
    } else {
      stack.config.file_paths.join(" -f ")
    };

    // Login to the registry to pull private images, if provider / account are set
    if !stack.config.registry_provider.is_empty()
      && !stack.config.registry_account.is_empty()
    {
      docker_login(
        &stack.config.registry_provider,
        &stack.config.registry_account,
        registry_token.as_deref(),
      )
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

    let env_file = env_file_path
      .map(|path| format!(" --env-file {path}"))
      .unwrap_or_default();

    let additional_env_files = stack
      .config
      .additional_env_files
      .iter()
      .fold(String::new(), |mut output, file| {
        let _ = write!(output, " --env-file {file}");
        output
      });

    let project_name = stack.project_name(false);

    let log = run_komodo_command(
      "compose pull",
      run_directory.as_ref(),
      format!(
        "{docker_compose} -p {project_name} -f {file_args}{env_file}{additional_env_files} pull{service_arg}",
      ),
      false,
    )
    .await;

    res.logs.push(log);

    Ok(res)
  }
}

//

impl Resolve<ComposeUp> for State {
  #[instrument(
    name = "ComposeUp",
    skip(self, git_token, registry_token)
  )]
  async fn resolve(
    &self,
    ComposeUp {
      stack,
      service,
      git_token,
      registry_token,
      replacers,
    }: ComposeUp,
    _: (),
  ) -> anyhow::Result<ComposeUpResponse> {
    let mut res = ComposeUpResponse::default();
    if let Err(e) = compose_up(
      stack,
      service,
      git_token,
      registry_token,
      &mut res,
      replacers,
    )
    .await
    {
      res.logs.push(Log::error(
        "compose up failed",
        format_serror(&e.into()),
      ));
    };
    Ok(res)
  }
}

//

impl Resolve<ComposeExecution> for State {
  #[instrument(name = "ComposeExecution", skip(self))]
  async fn resolve(
    &self,
    ComposeExecution { project, command }: ComposeExecution,
    _: (),
  ) -> anyhow::Result<Log> {
    let docker_compose = docker_compose();
    let log = run_komodo_command(
      "compose command",
      None,
      format!("{docker_compose} -p {project} {command}"),
      false,
    )
    .await;
    Ok(log)
  }
}
