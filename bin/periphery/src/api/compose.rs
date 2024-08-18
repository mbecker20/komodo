use std::path::PathBuf;

use anyhow::{anyhow, Context};
use command::run_monitor_command;
use formatting::format_serror;
use monitor_client::entities::{
  stack::{ComposeContents, ComposeProject},
  to_monitor_name,
  update::Log,
};
use periphery_client::api::compose::*;
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
    let res = run_monitor_command(
      "list projects",
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

impl Resolve<GetComposeContentsOnHost, ()> for State {
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
      periphery_config().stack_dir.join(to_monitor_name(&name));
    let run_directory = root.join(&run_directory);
    let run_directory = run_directory.canonicalize().context(
      "failed to validate run directory on host (canonicalize error)",
    )?;

    let file_paths = file_paths
      .iter()
      .map(|path| {
        run_directory.join(path).components().collect::<PathBuf>()
      })
      .collect::<Vec<_>>();

    let mut res = GetComposeContentsOnHostResponse::default();

    for full_path in &file_paths {
      match fs::read_to_string(&full_path).await.with_context(|| {
        format!(
          "failed to read compose file contents at {full_path:?}"
        )
      }) {
        Ok(contents) => {
          res.contents.push(ComposeContents {
            path: full_path.display().to_string(),
            contents,
          });
        }
        Err(e) => {
          res.errors.push(ComposeContents {
            path: full_path.display().to_string(),
            contents: format_serror(&e.into()),
          });
        }
      }
    }
    Ok(res)
  }
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
    }: GetComposeServiceLog,
    _: (),
  ) -> anyhow::Result<Log> {
    let docker_compose = docker_compose();
    let command = format!(
      "{docker_compose} -p {project} logs {service} --tail {tail}"
    );
    Ok(run_monitor_command("get stack log", command).await)
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
    }: GetComposeServiceLogSearch,
    _: (),
  ) -> anyhow::Result<Log> {
    let docker_compose = docker_compose();
    let grep = log_grep(&terms, combinator, invert);
    let command = format!("{docker_compose} -p {project} logs {service} --tail 5000 2>&1 | {grep}");
    Ok(run_monitor_command("get stack log grep", command).await)
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
    }: ComposeUp,
    _: (),
  ) -> anyhow::Result<ComposeUpResponse> {
    let mut res = ComposeUpResponse::default();
    if let Err(e) =
      compose_up(stack, service, git_token, registry_token, &mut res)
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
    let log = run_monitor_command(
      "compose command",
      format!("{docker_compose} -p {project} {command}"),
    )
    .await;
    Ok(log)
  }
}
