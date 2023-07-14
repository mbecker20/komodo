use async_trait::async_trait;
use monitor_types::entities::{server::docker_network::DockerNetwork, update::Log};
use resolver_api::{derive::Request, Resolve};
use serde::{Deserialize, Serialize};

use crate::{helpers::docker, state::State};

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<DockerNetwork>)]
pub struct GetNetworkList {}

#[async_trait]
impl Resolve<GetNetworkList> for State {
    async fn resolve(&self, _: GetNetworkList, _: ()) -> anyhow::Result<Vec<DockerNetwork>> {
        self.docker.list_networks().await
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct CreateNetwork {
    pub name: String,
    pub driver: Option<String>,
}

#[async_trait]
impl Resolve<CreateNetwork> for State {
    async fn resolve(
        &self,
        CreateNetwork { name, driver }: CreateNetwork,
        _: (),
    ) -> anyhow::Result<Log> {
        Ok(docker::create_network(&name, driver).await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct DeleteNetwork {
    pub name: String,
}

#[async_trait]
impl Resolve<DeleteNetwork> for State {
    async fn resolve(&self, DeleteNetwork { name }: DeleteNetwork, _: ()) -> anyhow::Result<Log> {
        Ok(docker::delete_network(&name).await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneNetworks {}

#[async_trait]
impl Resolve<PruneNetworks> for State {
    async fn resolve(&self, _: PruneNetworks, _: ()) -> anyhow::Result<Log> {
        Ok(docker::prune_networks().await)
    }
}
