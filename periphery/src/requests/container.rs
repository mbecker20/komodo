use anyhow::anyhow;
use monitor_types::entities::{
    deployment::{BasicContainerInfo, DockerContainerStats, TerminationSignal},
    server::{docker_image::ImageSummary, docker_network::DockerNetwork},
    update::Log,
};
use resolver_api::{derive::Request, Resolve};
use serde::{Deserialize, Serialize};

use crate::{helpers::docker, state::State};

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<BasicContainerInfo>)]
pub struct GetContainerList {}

#[async_trait::async_trait]
impl Resolve<GetContainerList> for State {
    async fn resolve(&self, _: GetContainerList) -> anyhow::Result<Vec<BasicContainerInfo>> {
        self.docker.list_containers().await
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct GetContainerLog {
    pub name: String,
    #[serde(default = "default_tail")]
    pub tail: u64,
}

fn default_tail() -> u64 {
    50
}

#[async_trait::async_trait]
impl Resolve<GetContainerLog> for State {
    async fn resolve(&self, req: GetContainerLog) -> anyhow::Result<Log> {
        Ok(docker::container_log(&req.name, req.tail).await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(DockerContainerStats)]
pub struct GetContainerStats {
    pub name: String,
}

#[async_trait::async_trait]
impl Resolve<GetContainerStats> for State {
    async fn resolve(&self, req: GetContainerStats) -> anyhow::Result<DockerContainerStats> {
        let error = anyhow!("no stats matching {}", req.name);
        let mut stats = docker::container_stats(Some(req.name)).await?;
        let stats = stats.pop().ok_or(error)?;
        Ok(stats)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<DockerContainerStats>)]
pub struct GetContainerStatsList {}

#[async_trait::async_trait]
impl Resolve<GetContainerStatsList> for State {
    async fn resolve(&self, _: GetContainerStatsList) -> anyhow::Result<Vec<DockerContainerStats>> {
        docker::container_stats(None).await
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<DockerNetwork>)]
pub struct GetNetworkList {}

#[async_trait::async_trait]
impl Resolve<GetNetworkList> for State {
    async fn resolve(&self, _: GetNetworkList) -> anyhow::Result<Vec<DockerNetwork>> {
        self.docker.list_networks().await
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<ImageSummary>)]
pub struct GetImageList {}

#[async_trait::async_trait]
impl Resolve<GetImageList> for State {
    async fn resolve(&self, _: GetImageList) -> anyhow::Result<Vec<ImageSummary>> {
        self.docker.list_images().await
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct StartContainer {
    pub name: String,
}

#[async_trait::async_trait]
impl Resolve<StartContainer> for State {
    async fn resolve(&self, req: StartContainer) -> anyhow::Result<Log> {
        Ok(docker::start_container(&req.name).await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct StopContainer {
    pub name: String,
    pub signal: Option<TerminationSignal>,
    pub time: Option<i32>,
}

#[async_trait::async_trait]
impl Resolve<StopContainer> for State {
    async fn resolve(&self, req: StopContainer) -> anyhow::Result<Log> {
        Ok(docker::stop_container(&req.name, req.signal, req.time).await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct RemoveContainer {
    pub name: String,
    pub signal: Option<TerminationSignal>,
    pub time: Option<i32>,
}

#[async_trait::async_trait]
impl Resolve<RemoveContainer> for State {
    async fn resolve(&self, req: RemoveContainer) -> anyhow::Result<Log> {
        Ok(docker::stop_and_remove_container(&req.name, req.signal, req.time).await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct RenameContainer {
    pub curr_name: String,
    pub new_name: String,
}

#[async_trait::async_trait]
impl Resolve<RenameContainer> for State {
    async fn resolve(&self, req: RenameContainer) -> anyhow::Result<Log> {
        Ok(docker::rename_container(&req.curr_name, &req.new_name).await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneContainers {}

#[async_trait::async_trait]
impl Resolve<PruneContainers> for State {
    async fn resolve(&self, _: PruneContainers) -> anyhow::Result<Log> {
        Ok(docker::prune_containers().await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct Deploy {}

#[async_trait::async_trait]
impl Resolve<Deploy> for State {
    async fn resolve(&self, _: Deploy) -> anyhow::Result<Log> {
        todo!()
    }
}
