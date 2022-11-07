use types::Build;

use crate::DockerClient;

impl DockerClient {
    pub async fn build(&self, build: Build) -> anyhow::Result<()> {
        todo!()
    }
}
