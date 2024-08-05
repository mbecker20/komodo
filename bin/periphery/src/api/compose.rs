use anyhow::{anyhow, Context};
use command::run_monitor_command;
use formatting::format_serror;
use monitor_client::entities::{to_monitor_name, update::Log};
use periphery_client::api::compose::*;
use resolver_api::Resolve;
use run_command::async_run_command;

use crate::{
  compose::{compose_up, docker_compose},
  config::periphery_config,
  helpers::log_grep,
  State,
};

impl Resolve<GetComposeInfo, ()> for State {
  #[instrument(name = "ComposeInfo", level = "debug", skip(self))]
  async fn resolve(
    &self,
    GetComposeInfo {
      name,
      run_directory,
      file_path,
      project,
    }: GetComposeInfo,
    _: (),
  ) -> anyhow::Result<GetComposeInfoReponse> {
    let file_missing = periphery_config()
      .stack_dir
      .join(to_monitor_name(&name))
      .join(run_directory)
      .join(file_path)
      .try_exists()
      .map(|exists| !exists)
      .context(
        "failed to reliably see whether the file is missing",
      )?;
    let docker_compose = docker_compose();
    // Note the space at the end of the regex:
    // `^{project} ` <-- Ensures exact name matches, as spaces aren't allowed in the names.
    let project_missing = !async_run_command(&format!(
      "{docker_compose} ls 2>&1 | grep -E '^{project} '"
    ))
    .await
    .success();
    Ok(GetComposeInfoReponse {
      file_missing,
      project_missing,
    })
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
      name,
      run_directory,
      file_path,
      service,
      tail,
    }: GetComposeServiceLog,
    _: (),
  ) -> anyhow::Result<Log> {
    let run_directory =
      periphery_config().stack_dir.join(&run_directory);

    if !run_directory.exists() {
      let e =
        anyhow!("the directory {run_directory:?} does not exist");
      return Ok(Log::error(
        "get stack log",
        format_serror(
          &e.context("Was the running stack imported? Be sure to set the correct run directory and file path to begin managing the stack")
            .context("Failed to get service log, stack run directory does not exist").into()
        ),
      ));
    }

    let run_directory = run_directory.display();
    let docker_compose = docker_compose();
    let name = to_monitor_name(&name);
    let command = format!("cd {run_directory} && {docker_compose} -p {name} logs {service} --tail {tail}");
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
      name,
      run_directory,
      file_path,
      service,
      terms,
      combinator,
      invert,
    }: GetComposeServiceLogSearch,
    _: (),
  ) -> anyhow::Result<Log> {
    let run_directory =
      periphery_config().stack_dir.join(&run_directory);

    if !run_directory.exists() {
      let e =
        anyhow!("the directory {run_directory:?} does not exist");
      return Ok(Log::error(
        "get stack log",
        format_serror(
          &e.context("Was a running stack imported? Be sure to set the correct run directory and file path to begin managing the stack")
            .context("Failed to get service log, stack run directory does not exist").into()
        ),
      ));
    }

    let run_directory = run_directory.display();
    let docker_compose = docker_compose();
    let grep = log_grep(&terms, combinator, invert);
    let name = to_monitor_name(&name);
    let command = format!("cd {run_directory} && {docker_compose} -p {name} logs {service} --tail 5000 2>&1 | {grep}");
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
