use anyhow::{anyhow, Context};
use command::run_komodo_command;
use futures::future::join_all;
use komodo_client::entities::{
  docker::container::{Container, ContainerListItem, ContainerStats},
  to_komodo_name,
  update::Log,
};
use periphery_client::api::container::*;
use resolver_api::Resolve;

use crate::{
  docker::{container_stats, docker_client, stop_container_command},
  helpers::log_grep,
};

// ======
//  READ
// ======

//

impl Resolve<super::Args> for InspectContainer {
  #[instrument(name = "InspectContainer", level = "debug")]
  async fn resolve(
    InspectContainer { name }: InspectContainer,
    _: &super::Args,
  ) -> anyhow::Result<Container> {
    docker_client().inspect_container(&name).await
  }
}

//

impl Resolve<super::Args> for GetContainerLog {
  #[instrument(name = "GetContainerLog", level = "debug")]
  async fn resolve(
    GetContainerLog {
      name,
      tail,
      timestamps,
    }: GetContainerLog,
    _: &super::Args,
  ) -> Result<Log, std::convert::Infallible> {
    let timestamps =
      timestamps.then_some(" --timestamps").unwrap_or_default();
    let command =
      format!("docker logs {name} --tail {tail}{timestamps}");
    Ok(
      run_komodo_command("get container log", None, command, false)
        .await,
    )
  }
}

//

impl Resolve<super::Args> for GetContainerLogSearch {
  #[instrument(name = "GetContainerLogSearch", level = "debug")]
  async fn resolve(
    GetContainerLogSearch {
      name,
      terms,
      combinator,
      invert,
      timestamps,
    }: GetContainerLogSearch,
    _: &super::Args,
  ) -> anyhow::Result<Log> {
    let grep = log_grep(&terms, combinator, invert);
    let timestamps =
      timestamps.then_some(" --timestamps").unwrap_or_default();
    let command = format!(
      "docker logs {name} --tail 5000{timestamps} 2>&1 | {grep}"
    );
    Ok(
      run_komodo_command(
        "get container log grep",
        None,
        command,
        false,
      )
      .await,
    )
  }
}

//

impl Resolve<super::Args> for GetContainerStats {
  #[instrument(name = "GetContainerStats", level = "debug")]
  async fn resolve(
    req: GetContainerStats,
    _: &super::Args,
  ) -> anyhow::Result<ContainerStats> {
    let error = anyhow!("no stats matching {}", req.name);
    let mut stats = container_stats(Some(req.name)).await?;
    let stats = stats.pop().ok_or(error)?;
    Ok(stats)
  }
}

//

impl Resolve<super::Args> for GetContainerStatsList {
  #[instrument(name = "GetContainerStatsList", level = "debug")]
  async fn resolve(
    _: GetContainerStatsList,
    _: &super::Args,
  ) -> anyhow::Result<Vec<ContainerStats>> {
    container_stats(None).await
  }
}

// =========
//  ACTIONS
// =========

impl Resolve<super::Args> for StartContainer {
  #[instrument(name = "StartContainer")]
  async fn resolve(
    StartContainer { name }: StartContainer,
    _: &super::Args,
  ) -> anyhow::Result<Log> {
    Ok(
      run_komodo_command(
        "docker start",
        None,
        format!("docker start {name}"),
        false,
      )
      .await,
    )
  }
}

//

impl Resolve<super::Args> for RestartContainer {
  #[instrument(name = "RestartContainer")]
  async fn resolve(
    RestartContainer { name }: RestartContainer,
    _: &super::Args,
  ) -> anyhow::Result<Log> {
    Ok(
      run_komodo_command(
        "docker restart",
        None,
        format!("docker restart {name}"),
        false,
      )
      .await,
    )
  }
}

//

impl Resolve<super::Args> for PauseContainer {
  #[instrument(name = "PauseContainer")]
  async fn resolve(
    PauseContainer { name }: PauseContainer,
    _: &super::Args,
  ) -> anyhow::Result<Log> {
    Ok(
      run_komodo_command(
        "docker pause",
        None,
        format!("docker pause {name}"),
        false,
      )
      .await,
    )
  }
}

impl Resolve<super::Args> for UnpauseContainer {
  #[instrument(name = "UnpauseContainer")]
  async fn resolve(
    UnpauseContainer { name }: UnpauseContainer,
    _: &super::Args,
  ) -> anyhow::Result<Log> {
    Ok(
      run_komodo_command(
        "docker unpause",
        None,
        format!("docker unpause {name}"),
        false,
      )
      .await,
    )
  }
}

//

impl Resolve<super::Args> for StopContainer {
  #[instrument(name = "StopContainer")]
  async fn resolve(
    StopContainer { name, signal, time }: StopContainer,
    _: &super::Args,
  ) -> anyhow::Result<Log> {
    let command = stop_container_command(&name, signal, time);
    let log =
      run_komodo_command("docker stop", None, command, false).await;
    if log.stderr.contains("unknown flag: --signal") {
      let command = stop_container_command(&name, None, time);
      let mut log =
        run_komodo_command("docker stop", None, command, false).await;
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

impl Resolve<super::Args> for RemoveContainer {
  #[instrument(name = "RemoveContainer")]
  async fn resolve(
    RemoveContainer { name, signal, time }: RemoveContainer,
    _: &super::Args,
  ) -> anyhow::Result<Log> {
    let stop_command = stop_container_command(&name, signal, time);
    let command =
      format!("{stop_command} && docker container rm {name}");
    let log = run_komodo_command(
      "docker stop and remove",
      None,
      command,
      false,
    )
    .await;
    if log.stderr.contains("unknown flag: --signal") {
      let stop_command = stop_container_command(&name, None, time);
      let command =
        format!("{stop_command} && docker container rm {name}");
      let mut log =
        run_komodo_command("docker stop", None, command, false).await;
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

impl Resolve<super::Args> for RenameContainer {
  #[instrument(name = "RenameContainer")]
  async fn resolve(
    RenameContainer {
      curr_name,
      new_name,
    }: RenameContainer,
    _: &super::Args,
  ) -> anyhow::Result<Log> {
    let new = to_komodo_name(&new_name);
    let command = format!("docker rename {curr_name} {new}");
    Ok(
      run_komodo_command("docker rename", None, command, false).await,
    )
  }
}

//

impl Resolve<super::Args> for PruneContainers {
  #[instrument(name = "PruneContainers")]
  async fn resolve(
    _: PruneContainers,
    _: &super::Args,
  ) -> anyhow::Result<Log> {
    let command = String::from("docker container prune -f");
    Ok(
      run_komodo_command("prune containers", None, command, false)
        .await,
    )
  }
}

//

impl Resolve<super::Args> for StartAllContainers {
  #[instrument(name = "StartAllContainers")]
  async fn resolve(
    StartAllContainers {}: StartAllContainers,
    _: &super::Args,
  ) -> anyhow::Result<Vec<Log>> {
    let containers = docker_client()
      .list_containers()
      .await
      .context("failed to list all containers on host")?;
    let futures = containers.iter().filter_map(
      |ContainerListItem { name, labels, .. }| {
        if labels.contains_key("komodo.skip") {
          return None;
        }
        let command = format!("docker start {name}");
        Some(async move {
          run_komodo_command(&command.clone(), None, command, false)
            .await
        })
      },
    );
    Ok(join_all(futures).await)
  }
}

//

impl Resolve<super::Args> for RestartAllContainers {
  #[instrument(name = "RestartAllContainers")]
  async fn resolve(
    RestartAllContainers {}: RestartAllContainers,
    _: &super::Args,
  ) -> anyhow::Result<Vec<Log>> {
    let containers = docker_client()
      .list_containers()
      .await
      .context("failed to list all containers on host")?;
    let futures = containers.iter().filter_map(
      |ContainerListItem { name, labels, .. }| {
        if labels.contains_key("komodo.skip") {
          return None;
        }
        let command = format!("docker restart {name}");
        Some(async move {
          run_komodo_command(&command.clone(), None, command, false)
            .await
        })
      },
    );
    Ok(join_all(futures).await)
  }
}

//

impl Resolve<super::Args> for PauseAllContainers {
  #[instrument(name = "PauseAllContainers")]
  async fn resolve(
    PauseAllContainers {}: PauseAllContainers,
    _: &super::Args,
  ) -> anyhow::Result<Vec<Log>> {
    let containers = docker_client()
      .list_containers()
      .await
      .context("failed to list all containers on host")?;
    let futures = containers.iter().filter_map(
      |ContainerListItem { name, labels, .. }| {
        if labels.contains_key("komodo.skip") {
          return None;
        }
        let command = format!("docker pause {name}");
        Some(async move {
          run_komodo_command(&command.clone(), None, command, false)
            .await
        })
      },
    );
    Ok(join_all(futures).await)
  }
}

//

impl Resolve<super::Args> for UnpauseAllContainers {
  #[instrument(name = "UnpauseAllContainers")]
  async fn resolve(
    UnpauseAllContainers {}: UnpauseAllContainers,
    _: &super::Args,
  ) -> anyhow::Result<Vec<Log>> {
    let containers = docker_client()
      .list_containers()
      .await
      .context("failed to list all containers on host")?;
    let futures = containers.iter().filter_map(
      |ContainerListItem { name, labels, .. }| {
        if labels.contains_key("komodo.skip") {
          return None;
        }
        let command = format!("docker unpause {name}");
        Some(async move {
          run_komodo_command(&command.clone(), None, command, false)
            .await
        })
      },
    );
    Ok(join_all(futures).await)
  }
}

//

impl Resolve<super::Args> for StopAllContainers {
  #[instrument(name = "StopAllContainers")]
  async fn resolve(
    StopAllContainers {}: StopAllContainers,
    _: &super::Args,
  ) -> anyhow::Result<Vec<Log>> {
    let containers = docker_client()
      .list_containers()
      .await
      .context("failed to list all containers on host")?;
    let futures = containers.iter().filter_map(
      |ContainerListItem { name, labels, .. }| {
        if labels.contains_key("komodo.skip") {
          return None;
        }
        Some(async move {
          run_komodo_command(
            &format!("docker stop {name}"),
            None,
            stop_container_command(name, None, None),
            false,
          )
          .await
        })
      },
    );
    Ok(join_all(futures).await)
  }
}
