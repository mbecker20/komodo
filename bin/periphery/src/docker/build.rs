use anyhow::Context;
use command::run_monitor_command;
use monitor_client::entities::{
  build::{Build, BuildConfig},
  get_image_name, optional_string, to_monitor_name,
  update::Log,
  EnvironmentVar, Version,
};
use serror::serialize_error_pretty;

use crate::config::periphery_config;

use super::{docker_login, parse_extra_args, parse_labels};

#[instrument]
pub async fn prune_images() -> Log {
  let command = String::from("docker image prune -a -f");
  run_monitor_command("prune images", command).await
}

#[instrument(skip(registry_token, core_replacers))]
pub async fn build(
  build: &Build,
  registry_token: Option<String>,
  core_replacers: Vec<(String, String)>,
) -> anyhow::Result<Vec<Log>> {
  let Build {
    name,
    config:
      BuildConfig {
        version,
        skip_secret_interp,
        build_path,
        dockerfile_path,
        build_args,
        labels,
        extra_args,
        use_buildx,
        image_registry,
        ..
      },
    ..
  } = build;

  let mut logs = Vec::new();

  // Maybe docker login
  let should_push = match docker_login(
    image_registry,
    registry_token.as_deref(),
  )
  .await
  {
    Ok(should_push) => should_push,
    Err(e) => {
      logs.push(Log::error(
        "docker login",
        serialize_error_pretty(
          &e.context("failed to login to docker registry"),
        ),
      ));
      return Ok(logs);
    }
  };

  // Get paths
  let name = to_monitor_name(name);
  let build_dir =
    periphery_config().repo_dir.join(&name).join(build_path);
  let dockerfile_path = match optional_string(dockerfile_path) {
    Some(dockerfile_path) => dockerfile_path.to_owned(),
    None => "Dockerfile".to_owned(),
  };

  // Get command parts
  let build_args = parse_build_args(build_args);
  let labels = parse_labels(labels);
  let extra_args = parse_extra_args(extra_args);
  let buildx = if *use_buildx { " buildx" } else { "" };
  let image_name = get_image_name(build);
  let image_tags = image_tags(&image_name, version);
  let push_command = should_push
    .then(|| format!(" && docker image push --all-tags {image_name}"))
    .unwrap_or_default();

  // Construct command
  let command = format!(
    "cd {} && docker{buildx} build{build_args}{extra_args}{labels}{image_tags} -f {dockerfile_path} .{push_command}",
    build_dir.display()
  );

  if *skip_secret_interp {
    let build_log =
      run_monitor_command("docker build", command).await;
    info!("finished building docker image");
    logs.push(build_log);
  } else {
    // Interpolate any missing secrets
    let (command, mut replacers) = svi::interpolate_variables(
      &command,
      &periphery_config().secrets,
      svi::Interpolator::DoubleBrackets,
      true,
    )
    .context(
      "failed to interpolate secrets into docker build command",
    )?;
    replacers.extend(core_replacers);

    let mut build_log =
      run_monitor_command("docker build", command).await;
    build_log.command =
      svi::replace_in_string(&build_log.command, &replacers);
    build_log.stdout =
      svi::replace_in_string(&build_log.stdout, &replacers);
    build_log.stderr =
      svi::replace_in_string(&build_log.stderr, &replacers);
    logs.push(build_log);
  }

  Ok(logs)
}

fn image_tags(image_name: &str, version: &Version) -> String {
  let Version { major, minor, .. } = version;
  format!(
    " -t {image_name}:latest -t {image_name}:{version} -t {image_name}:{major}.{minor} -t {image_name}:{major}",
  )
}

fn parse_build_args(build_args: &[EnvironmentVar]) -> String {
  build_args
    .iter()
    .map(|p| format!(" --build-arg {}=\"{}\"", p.variable, p.value))
    .collect::<Vec<_>>()
    .join("")
}
