use std::sync::Arc;

use anyhow::anyhow;
use axum::Extension;
use bollard::{container::ListContainersOptions, Docker};
use types::BasicContainerInfo;

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
                    state: s.state.unwrap().parse().unwrap(),
                    status: s.status,
                };
                Ok::<_, anyhow::Error>(info)
            })
            .collect::<anyhow::Result<Vec<BasicContainerInfo>>>()?;
        Ok(res)
    }
}
