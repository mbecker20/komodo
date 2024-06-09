use anyhow::anyhow;
use monitor_client::entities::{
  deployment::{ContainerSummary, DockerContainerStats},
  update::Log,
};
use periphery_client::api::container::*;
use resolver_api::Resolve;

use crate::{
  docker::{self, client::docker_client},
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
    req: GetContainerLog,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::container::container_log(&req.name, req.tail).await)
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
    req: GetContainerLogSearch,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(
      docker::container::container_log_search(
        &req.name,
        &req.terms,
        req.combinator,
      )
      .await,
    )
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
    let mut stats =
      docker::container::container_stats(Some(req.name)).await?;
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
    docker::container::container_stats(None).await
  }
}

//

impl Resolve<StartContainer> for State {
  #[instrument(name = "StartContainer", skip(self))]
  async fn resolve(
    &self,
    req: StartContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::container::start_container(&req.name).await)
  }
}

//

impl Resolve<StopContainer> for State {
  #[instrument(name = "StopContainer", skip(self))]
  async fn resolve(
    &self,
    req: StopContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(
      docker::container::stop_container(
        &req.name, req.signal, req.time,
      )
      .await,
    )
  }
}

//

impl Resolve<RemoveContainer> for State {
  #[instrument(name = "RemoveContainer", skip(self))]
  async fn resolve(
    &self,
    req: RemoveContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(
      docker::container::stop_and_remove_container(
        &req.name, req.signal, req.time,
      )
      .await,
    )
  }
}

//

impl Resolve<RenameContainer> for State {
  #[instrument(name = "RenameContainer", skip(self))]
  async fn resolve(
    &self,
    req: RenameContainer,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(
      docker::container::rename_container(
        &req.curr_name,
        &req.new_name,
      )
      .await,
    )
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
    Ok(docker::container::prune_containers().await)
  }
}

//

impl Resolve<Deploy> for State {
  #[instrument(name = "Deploy", skip(self, replacers))]
  async fn resolve(
    &self,
    Deploy {
      deployment,
      stop_signal,
      stop_time,
      registry_token,
      replacers,
    }: Deploy,
    _: (),
  ) -> anyhow::Result<Log> {
    let res = docker::container::deploy(
      &deployment,
      stop_signal
        .unwrap_or(deployment.config.termination_signal)
        .into(),
      stop_time
        .unwrap_or(deployment.config.termination_timeout)
        .into(),
      registry_token,
      replacers,
    )
    .await;
    Ok(res)
  }
}
