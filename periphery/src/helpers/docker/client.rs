use anyhow::anyhow;
use bollard::{container::ListContainersOptions, Docker};
use monitor_types::entities::deployment::BasicContainerInfo;

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub fn new() -> DockerClient {
        DockerClient {
            docker: Docker::connect_with_local_defaults()
                .expect("failed to connect to docker daemon"),
        }
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
                        .replace('/', ""),
                    image: s.image.unwrap_or(String::from("unknown")),
                    state: s.state.unwrap().parse().unwrap(),
                    status: s.status,
                };
                Ok::<_, anyhow::Error>(info)
            })
            .collect::<anyhow::Result<Vec<BasicContainerInfo>>>()?;
        Ok(res)
    }

    // pub async fn list_networks(&self) -> anyhow::Result<Vec<Network>> {
    //     let networks = self.docker.list_networks::<String>(None).await?;
    //     Ok(networks)
    // }

    // pub async fn list_images(&self) -> anyhow::Result<Vec<ImageSummary>> {
    //     let images = self.docker.list_images::<String>(None).await?;
    //     Ok(images)
    // }
}
