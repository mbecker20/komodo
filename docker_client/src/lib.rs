use anyhow::anyhow;
use bollard::{container::ListContainersOptions, Docker};
use types::BasicContainerInfo;

mod build;
mod deploy;

pub struct DockerClient {
    client: Docker,
}

impl DockerClient {
    pub fn new() -> anyhow::Result<DockerClient> {
        Ok(DockerClient {
            client: Docker::connect_with_local_defaults()?,
        })
    }

    pub async fn list_containers(&self) -> anyhow::Result<Vec<BasicContainerInfo>> {
        let res = self
            .client
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await?
            .into_iter()
            .map(|s| {
                let info = BasicContainerInfo {
                    name: s
                        .names
                        .ok_or(anyhow!("no names on container"))?
                        .pop()
                        .ok_or(anyhow!("no names on container (empty vec)"))?
                        .replace("/", ""),
                    state: s.state.unwrap().parse().unwrap(),
                    status: s.status,
                };
                Ok::<_, anyhow::Error>(info)
            })
            .collect::<anyhow::Result<Vec<BasicContainerInfo>>>()?;
        Ok(res)
    }
}
