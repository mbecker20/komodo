use async_trait::async_trait;
use monitor_client::entities::{
  server::docker_network::DockerNetwork, update::Log,
};
use periphery_client::api::network::{
  CreateNetwork, DeleteNetwork, GetNetworkList, PruneNetworks,
};
use resolver_api::Resolve;

use crate::{
  helpers::docker::{self, client::docker_client},
  State,
};

//

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
