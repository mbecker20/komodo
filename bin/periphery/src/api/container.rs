use anyhow::{anyhow, Context};
use command::run_monitor_command;
use formatting::format_serror;
use monitor_client::entities::{
  deployment::{
    ContainerSummary, Conversion, Deployment, DeploymentConfig,
    DeploymentImage, DockerContainerStats, RestartMode,
    TerminationSignal,
  },
  to_monitor_name,
  update::Log,
  EnvironmentVar, SearchCombinator,
};
use periphery_client::api::container::*;
use resolver_api::Resolve;
use run_command::async_run_command;

use crate::{
  config::periphery_config,
  docker::{
    client::docker_client, docker_login, parse_extra_args,
    parse_labels,
  },
  State,
};

//

impl Resolve<GetContainerList> for State {
  #[instrument(
    name = "GetContainerList",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    _: GetContainerList,
    _: (),
  ) -> anyhow::Result<Vec<ContainerSummary>> {
    docker_client().list_containers().await
  }
}

//

impl Resolve<GetContainerLog> for State {
  #[instrument(name = "GetContainerLog", level = "debug", skip(self))]
  async fn resolve(
    &self,
    GetContainerLog { name, tail }: GetContainerLog,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = format!("docker logs {name} --tail {tail}");
    Ok(run_monitor_command("get container log", command).await)
  }
}

//

impl Resolve<GetContainerLogSearch> for State {
  #[instrument(
    name = "GetContainerLogSearch",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    GetContainerLogSearch {
      name,
      terms,
      combinator,
      invert,
    }: GetContainerLogSearch,
    _: (),
  ) -> anyhow::Result<Log> {
    let maybe_invert = invert.then_some(" -v").unwrap_or_default();
    let grep = match combinator {
      SearchCombinator::Or => {
        format!("grep{maybe_invert} -E '{}'", terms.join("|"))
      }
      SearchCombinator::And => {
        format!(
          "grep{maybe_invert} -P '^(?=.*{})'",
          terms.join(")(?=.*")
        )
      }
    };
    let command =
      format!("docker logs {name} --tail 5000 2>&1 | {grep}");
    Ok(run_monitor_command("get container log grep", command).await)
  }
}

//

impl Resolve<GetContainerStats> for State {
  #[instrument(
    name = "GetContainerStats",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    req: GetContainerStats,
    _: (),
  ) -> anyhow::Result<DockerContainerStats> {
    let error = anyhow!("no stats matching {}", req.name);
    let mut stats = container_stats(Some(req.name)).await?;
    let stats = stats.pop().ok_or(error)?;
    Ok(stats)
  }
}

//

impl Resolve<GetContainerStatsList> for State {
  #[instrument(
    name = "GetContainerStatsList",
    level = "debug",
    skip(self)
  )]
  async fn resolve(
    &self,
    _: GetContainerStatsList,
    _: (),
  ) -> anyhow::Result<Vec<DockerContainerStats>> {
    container_stats(None).await
  }
}

//

impl Resolve<StartContainer> for State {
  #[instrument(name = "StartContainer", skip(self))]
  async fn resolve(
    &self,
    StartContainer { name }: StartContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(
      run_monitor_command(
        "docker start",
        format!("docker start {name}"),
      )
      .await,
    )
  }
}

//

impl Resolve<StopContainer> for State {
  #[instrument(name = "StopContainer", skip(self))]
  async fn resolve(
    &self,
    StopContainer { name, signal, time }: StopContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = stop_container_command(&name, signal, time);
    let log = run_monitor_command("docker stop", command).await;
    if log.stderr.contains("unknown flag: --signal") {
      let command = stop_container_command(&name, None, time);
      let mut log = run_monitor_command("docker stop", command).await;
      log.stderr = format!(
        "old docker version: unable to use --signal flag{}",
        if !log.stderr.is_empty() {
          format!("\n\n{}", log.stderr)
        } else {
          String::new()
        }
      );
      Ok(log)
    } else {
      Ok(log)
    }
  }
}

//

impl Resolve<RemoveContainer> for State {
  #[instrument(name = "RemoveContainer", skip(self))]
  async fn resolve(
    &self,
    RemoveContainer { name, signal, time }: RemoveContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    let stop_command = stop_container_command(&name, signal, time);
    let command =
      format!("{stop_command} && docker container rm {name}");
    let log =
      run_monitor_command("docker stop and remove", command).await;
    if log.stderr.contains("unknown flag: --signal") {
      let stop_command = stop_container_command(&name, None, time);
      let command =
        format!("{stop_command} && docker container rm {name}");
      let mut log = run_monitor_command("docker stop", command).await;
      log.stderr = format!(
        "old docker version: unable to use --signal flag{}",
        if !log.stderr.is_empty() {
          format!("\n\n{}", log.stderr)
        } else {
          String::new()
        }
      );
      Ok(log)
    } else {
      Ok(log)
    }
  }
}

//

impl Resolve<RenameContainer> for State {
  #[instrument(name = "RenameContainer", skip(self))]
  async fn resolve(
    &self,
    RenameContainer {
      curr_name,
      new_name,
    }: RenameContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    let new = to_monitor_name(&new_name);
    let command = format!("docker rename {curr_name} {new}");
    Ok(run_monitor_command("docker rename", command).await)
  }
}

//

impl Resolve<PruneContainers> for State {
  #[instrument(name = "PruneContainers", skip(self))]
  async fn resolve(
    &self,
    _: PruneContainers,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = String::from("docker container prune -f");
    Ok(run_monitor_command("prune containers", command).await)
  }
}

//

impl Resolve<Deploy> for State {
  #[instrument(
    name = "Deploy",
    skip(self, core_replacers, aws_ecr, registry_token)
  )]
  async fn resolve(
    &self,
    Deploy {
      deployment,
      stop_signal,
      stop_time,
      registry_token,
      replacers: core_replacers,
      aws_ecr,
    }: Deploy,
    _: (),
  ) -> anyhow::Result<Log> {
    if let Err(e) = docker_login(
      &deployment.config.image_registry,
      registry_token.as_deref(),
      aws_ecr.as_ref(),
    )
    .await
    {
      return Ok(Log::error(
        "docker login",
        format_serror(
          &e.context("failed to login to docker registry").into(),
        ),
      ));
    }

    let image = if let DeploymentImage::Image { image } =
      &deployment.config.image
    {
      if image.is_empty() {
        return Ok(Log::error(
          "get image",
          String::from("deployment does not have image attached"),
        ));
      }
      image
    } else {
      return Ok(Log::error(
        "get image",
        String::from("deployment does not have image attached"),
      ));
    };

    let _ = pull_image(image).await;
    debug!("image pulled");
    let _ = State
      .resolve(
        RemoveContainer {
          name: deployment.name.clone(),
          signal: stop_signal,
          time: stop_time,
        },
        (),
      )
      .await;
    debug!("container stopped and removed");

    let command = docker_run_command(&deployment, image);
    debug!("docker run command: {command}");

    if deployment.config.skip_secret_interp {
      Ok(run_monitor_command("docker run", command).await)
    } else {
      let command = svi::interpolate_variables(
        &command,
        &periphery_config().secrets,
        svi::Interpolator::DoubleBrackets,
        true,
      )
      .context(
        "failed to interpolate secrets into docker run command",
      );
      if let Err(e) = command {
        return Ok(Log::error("docker run", format!("{e:?}")));
      }
      let (command, mut replacers) = command.unwrap();
      replacers.extend(core_replacers);
      let mut log = run_monitor_command("docker run", command).await;
      log.command = svi::replace_in_string(&log.command, &replacers);
      log.stdout = svi::replace_in_string(&log.stdout, &replacers);
      log.stderr = svi::replace_in_string(&log.stderr, &replacers);
      Ok(log)
    }
  }
}

//

fn docker_run_command(
  Deployment {
    name,
    config:
      DeploymentConfig {
        volumes,
        ports,
        network,
        command,
        restart,
        environment,
        labels,
        extra_args,
        ..
      },
    ..
  }: &Deployment,
  image: &str,
) -> String {
  let name = to_monitor_name(name);
  let ports = parse_conversions(ports, "-p");
  let volumes = volumes.to_owned();
  let volumes = parse_conversions(&volumes, "-v");
  let network = parse_network(network);
  let restart = parse_restart(restart);
  let environment = parse_environment(environment);
  let labels = parse_labels(labels);
  let command = parse_command(command);
  let extra_args = parse_extra_args(extra_args);
  format!("docker run -d --name {name}{ports}{volumes}{network}{restart}{environment}{labels}{extra_args} {image}{command}")
}

fn parse_conversions(
  conversions: &[Conversion],
  flag: &str,
) -> String {
  conversions
    .iter()
    .map(|p| format!(" {flag} {}:{}", p.local, p.container))
    .collect::<Vec<_>>()
    .join("")
}

fn parse_environment(environment: &[EnvironmentVar]) -> String {
  environment
    .iter()
    .map(|p| format!(" --env {}=\"{}\"", p.variable, p.value))
    .collect::<Vec<_>>()
    .join("")
}

fn parse_network(network: &str) -> String {
  format!(" --network {network}")
}

fn parse_restart(restart: &RestartMode) -> String {
  let restart = match restart {
    RestartMode::OnFailure => "on-failure:10".to_string(),
    _ => restart.to_string(),
  };
  format!(" --restart {restart}")
}

fn parse_command(command: &str) -> String {
  if command.is_empty() {
    String::new()
  } else {
    format!(" {command}")
  }
}

//

async fn container_stats(
  container_name: Option<String>,
) -> anyhow::Result<Vec<DockerContainerStats>> {
  let format = "--format \"{{ json . }}\"";
  let container_name = match container_name {
    Some(name) => format!(" {name}"),
    None => "".to_string(),
  };
  let command =
    format!("docker stats{container_name} --no-stream {format}");
  let output = async_run_command(&command).await;
  if output.success() {
    let res = output
      .stdout
      .split('\n')
      .filter(|e| !e.is_empty())
      .map(|e| {
        let parsed = serde_json::from_str(e)
          .context(format!("failed at parsing entry {e}"))?;
        Ok(parsed)
      })
      .collect::<anyhow::Result<Vec<DockerContainerStats>>>()?;
    Ok(res)
  } else {
    Err(anyhow!("{}", output.stderr.replace('\n', "")))
  }
}

#[instrument]
async fn pull_image(image: &str) -> Log {
  let command = format!("docker pull {image}");
  run_monitor_command("docker pull", command).await
}

fn stop_container_command(
  container_name: &str,
  signal: Option<TerminationSignal>,
  time: Option<i32>,
) -> String {
  let container_name = to_monitor_name(container_name);
  let signal = signal
    .map(|signal| format!(" --signal {signal}"))
    .unwrap_or_default();
  let time = time
    .map(|time| format!(" --time {time}"))
    .unwrap_or_default();
  format!("docker stop{signal}{time} {container_name}")
}
