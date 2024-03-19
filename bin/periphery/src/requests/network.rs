use async_trait::async_trait;
use monitor_client::entities::{
  server::docker_network::DockerNetwork, update::Log,
};
use resolver_api::{derive::Request, Resolve};
use serde::{Deserialize, Serialize};

use crate::{
  helpers::docker::{self, client::docker_client},
  State,
};

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<DockerNetwork>)]
pub struct GetNetworkList {}

#[async_trait]
impl Resolve<GetNetworkList> for State {
  async fn resolve(
    &self,
    _: GetNetworkList,
    _: (),
  ) -> anyhow::Result<Vec<DockerNetwork>> {
    docker_client().list_networks().await
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
    Ok(docker::network::create_network(&name, driver).await)
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
  async fn resolve(
    &self,
    DeleteNetwork { name }: DeleteNetwork,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::network::delete_network(&name).await)
  }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneNetworks {}

#[async_trait]
impl Resolve<PruneNetworks> for State {
  async fn resolve(
    &self,
    _: PruneNetworks,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::network::prune_networks().await)
  }
}
