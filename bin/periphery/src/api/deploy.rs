use anyhow::Context;
use command::run_monitor_command;
use formatting::format_serror;
use monitor_client::entities::{
  build::{ImageRegistry, StandardRegistryConfig},
  deployment::{
    extract_registry_domain, Conversion, Deployment,
    DeploymentConfig, DeploymentImage, RestartMode,
  },
  to_monitor_name,
  update::Log,
  EnvironmentVar, NoData,
};
use periphery_client::api::container::{Deploy, RemoveContainer};
use resolver_api::Resolve;

use crate::{
  config::periphery_config,
  docker::{docker_login, pull_image},
  helpers::{parse_extra_args, parse_labels},
  State,
};

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

    let image_registry = if aws_ecr.is_some() {
      ImageRegistry::AwsEcr(String::new())
    } else if deployment.config.image_registry_account.is_empty() {
      ImageRegistry::None(NoData {})
    } else {
      ImageRegistry::Standard(StandardRegistryConfig {
        account: deployment.config.image_registry_account.clone(),
        domain: extract_registry_domain(image)?,
        ..Default::default()
      })
    };

    if let Err(e) = docker_login(
      &image_registry,
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
      let (command, mut replacers) = match command {
        Ok(res) => res,
        Err(e) => {
          return Ok(Log::error("docker run", format!("{e:?}")));
        }
      };
      replacers.extend(core_replacers);
      let mut log = run_monitor_command("docker run", command).await;
      log.command = svi::replace_in_string(&log.command, &replacers);
      log.stdout = svi::replace_in_string(&log.stdout, &replacers);
      log.stderr = svi::replace_in_string(&log.stderr, &replacers);
      Ok(log)
    }
  }
}

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
