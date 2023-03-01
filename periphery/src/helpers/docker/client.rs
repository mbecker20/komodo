use std::sync::Arc;

use anyhow::anyhow;
use axum::Extension;
use bollard::{container::ListContainersOptions, Docker};
use types::{BasicContainerInfo, ImageSummary, Network};

pub use bollard::container::Stats;

pub type DockerExtension = Extension<Arc<DockerClient>>;

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub fn extension() -> DockerExtension {
        let client = DockerClient {
            docker: Docker::connect_with_local_defaults()
                .expect("failed to connect to docker daemon"),
        };
        Extension(Arc::new(client))
    }

    pub async fn list_containers(&self) -> anyhow::Result<Vec<BasicContainerInfo>> {
        let res = self
            .docker
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await?
            .into_iter()
            .map(|s| {
                let info = BasicContainerInfo {
                    id: s.id.unwrap_or_default(),
                    name: s
                        .names
                        .ok_or(anyhow!("no names on container"))?
                        .pop()
                        .ok_or(anyhow!("no names on container (empty vec)"))?
                        .replace("/", ""),
                    image: s.image.unwrap_or(String::from("unknown")),
                    state: s.state.unwrap().parse().unwrap(),
                    status: s.status,
                };
                Ok::<_, anyhow::Error>(info)
            })
            .collect::<anyhow::Result<Vec<BasicContainerInfo>>>()?;
        Ok(res)
    }

    // pub fn container_stats_stream(
    //     &self,
    //     container_name: &str,
    // ) -> impl Stream<Item = Result<Stats, bollard::errors::Error>> {
    //     self.docker.stats(
    //         container_name,
    //         Some(StatsOptions {
    //             stream: true,
    //             ..Default::default()
    //         }),
    //     )
    // }

    // pub async fn get_container_stats(&self, container_name: &str) -> anyhow::Result<Stats> {
    //     let mut stats = self
    //         .docker
    //         .stats(
    //             container_name,
    //             Some(StatsOptions {
    //                 stream: false,
    //                 ..Default::default()
    //             }),
    //         )
    //         .take(1)
    //         .next()
    //         .await
    //         .ok_or(anyhow!("got no stats for {container_name}"))??;
    //     stats.name = stats.name.replace("/", "");
    //     Ok(stats)
    // }

    // pub async fn get_container_stats_list(&self) -> anyhow::Result<Vec<Stats>> {
    //     let futures = self
    //         .list_containers()
    //         .await?
    //         .into_iter()
    //         .filter(|c| c.state == DockerContainerState::Running)
    //         .map(|c| async move {
    //             let mut stats = self
    //                 .docker
    //                 .stats(
    //                     &c.name,
    //                     Some(StatsOptions {
    //                         stream: false,
    //                         ..Default::default()
    //                     }),
    //                 )
    //                 .take(1)
    //                 .next()
    //                 .await
    //                 .ok_or(anyhow!("got no stats for {}", c.name))??;
    //             stats.name = stats.name.replace("/", "");
    //             Ok::<_, anyhow::Error>(stats)
    //         });
    //     let stats = join_all(futures)
    //         .await
    //         .into_iter()
    //         .collect::<anyhow::Result<_>>()?;
    //     Ok(stats)
    // }

    pub async fn list_networks(&self) -> anyhow::Result<Vec<Network>> {
        let networks = self.docker.list_networks::<String>(None).await?;
        Ok(networks)
    }

    pub async fn list_images(&self) -> anyhow::Result<Vec<ImageSummary>> {
        let images = self.docker.list_images::<String>(None).await?;
        Ok(images)
    }
}
