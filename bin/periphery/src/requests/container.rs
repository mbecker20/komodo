use anyhow::{anyhow, Context};
use monitor_types::{
    entities::{
        deployment::{BasicContainerInfo, Deployment, DockerContainerStats, TerminationSignal},
        update::Log,
    },
    optional_string,
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
    async fn resolve(&self, _: GetContainerList, _: ()) -> anyhow::Result<Vec<BasicContainerInfo>> {
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
    async fn resolve(&self, req: GetContainerLog, _: ()) -> anyhow::Result<Log> {
        Ok(docker::container_log(&req.name, req.tail).await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct GetContainerLogSearch {
    pub name: String,
    pub search: String,
}

#[async_trait::async_trait]
impl Resolve<GetContainerLogSearch> for State {
    async fn resolve(&self, req: GetContainerLogSearch, _: ()) -> anyhow::Result<Log> {
        Ok(docker::container_log_search(&req.name, &req.search).await)
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
    async fn resolve(&self, req: GetContainerStats, _: ()) -> anyhow::Result<DockerContainerStats> {
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
    async fn resolve(
        &self,
        _: GetContainerStatsList,
        _: (),
    ) -> anyhow::Result<Vec<DockerContainerStats>> {
        docker::container_stats(None).await
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
    async fn resolve(&self, req: StartContainer, _: ()) -> anyhow::Result<Log> {
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
    async fn resolve(&self, req: StopContainer, _: ()) -> anyhow::Result<Log> {
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
    async fn resolve(&self, req: RemoveContainer, _: ()) -> anyhow::Result<Log> {
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
    async fn resolve(&self, req: RenameContainer, _: ()) -> anyhow::Result<Log> {
        Ok(docker::rename_container(&req.curr_name, &req.new_name).await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneContainers {}

#[async_trait::async_trait]
impl Resolve<PruneContainers> for State {
    async fn resolve(&self, _: PruneContainers, _: ()) -> anyhow::Result<Log> {
        Ok(docker::prune_containers().await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct Deploy {
    pub deployment: Deployment,
    pub stop_signal: Option<TerminationSignal>,
    pub stop_time: Option<i32>,
}

#[async_trait::async_trait]
impl Resolve<Deploy> for State {
    async fn resolve(
        &self,
        Deploy {
            deployment,
            stop_signal,
            stop_time,
        }: Deploy,
        _: (),
    ) -> anyhow::Result<Log> {
        let secrets = self.secrets.clone();
        let log = match self.get_docker_token(&optional_string(&deployment.config.docker_account)) {
            Ok(docker_token) => tokio::spawn(async move {
                docker::deploy(
                    &deployment,
                    &docker_token,
                    &secrets,
                    stop_signal
                        .unwrap_or(deployment.config.termination_signal)
                        .into(),
                    stop_time
                        .unwrap_or(deployment.config.termination_timeout)
                        .into(),
                )
                .await
            })
            .await
            .context("failed at spawn thread for deploy")?,
            Err(e) => Log::error("docker login", format!("{e:#?}")),
        };
        Ok(log)
    }
}
