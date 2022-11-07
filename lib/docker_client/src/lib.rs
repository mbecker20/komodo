#![allow(unused)]

use std::sync::Arc;

use anyhow::anyhow;
use axum::Extension;
use bollard::{container::ListContainersOptions, Docker};
use types::BasicContainerInfo;

mod build;
mod deploy;

pub type DockerExtenstion = Extension<Arc<DockerClient>>;

pub struct DockerClient {
    client: Docker,
}

impl DockerClient {
    pub fn extension() -> DockerExtenstion {
        let client = DockerClient {
            client: Docker::connect_with_local_defaults()
                .expect("failed to connect to docker daemon"),
        };
        Extension(Arc::new(client))
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
