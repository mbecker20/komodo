use types::Deployment;

use crate::DockerClient;

impl DockerClient {
    pub async fn deploy(&self, deployment: Deployment) -> anyhow::Result<()> {
        todo!()
    }
}
