use std::path::PathBuf;

use anyhow::{anyhow, Context};
use command::run_komodo_command;
use formatting::format_serror;
use git::{write_commit_file, GitRes};
use komodo_client::entities::{
  stack::ComposeProject, to_komodo_name, update::Log, CloneArgs,
  FileContents,
};
use periphery_client::api::{
  compose::*,
  git::{PullOrCloneRepo, RepoActionResponse},
};
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
  compose::{compose_up, docker_compose},
  config::periphery_config,
  helpers::log_grep,
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
    Ok(run_komodo_command("get stack log", None, command).await)
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
    Ok(run_komodo_command("get stack log grep", None, command).await)
  }
}

//

const DEFAULT_COMPOSE_CONTENTS: &str = "## 🦎 Hello Komodo 🦎
services:
  hello_world:
    image: hello-world
    # networks:
    #   - default
    # ports:
    #   - 3000:3000
    # volumes:
    #   - data:/data

# networks:
#   default: {}

# volumes:
#   data:
";

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
      if !full_path.exists() {
        fs::write(&full_path, DEFAULT_COMPOSE_CONTENTS)
          .await
          .context("Failed to init missing compose file on host")?;
      }
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
    if stack.config.files_on_host {
      return Err(anyhow!(
        "Wrong method called for files on host stack"
      ));
    }
    if stack.config.repo.is_empty() {
      return Err(anyhow!("Repo is not configured"));
    }

    let root = periphery_config()
      .stack_dir
      .join(to_komodo_name(&stack.name));

    let mut args: CloneArgs = (&stack).into();
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
              return Err(
                e.context("Failed to find required git token"),
              );
            }
          }
        } else {
          None
        }
      }
    };

    State
      .resolve(
        PullOrCloneRepo {
          args,
          git_token,
          environment: vec![],
          env_file_path: stack.config.env_file_path.clone(),
          skip_secret_interp: stack.config.skip_secret_interp,
          // repo replacer only needed for on_clone / on_pull,
          // which aren't available for stacks
          replacers: Default::default(),
        },
        (),
      )
      .await?;

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
    )
    .await;
    Ok(log)
  }
}
