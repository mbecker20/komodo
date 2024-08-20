use command::run_monitor_command;
use monitor_client::entities::{
  docker::network::{Network, NetworkListItem},
  update::Log,
};
use periphery_client::api::network::{
  CreateNetwork, DeleteNetwork, GetNetworkList, InspectNetwork,
  PruneNetworks,
};
use resolver_api::Resolve;

use crate::{docker::docker_client, State};

//

impl Resolve<GetNetworkList> for State {
  #[instrument(name = "GetNetworkList", level = "debug", skip(self))]
  async fn resolve(
    &self,
    _: GetNetworkList,
    _: (),
  ) -> anyhow::Result<Vec<NetworkListItem>> {
    docker_client().list_networks().await
  }
}

//

impl Resolve<InspectNetwork> for State {
  #[instrument(name = "InspectNetwork", level = "debug", skip(self))]
  async fn resolve(
    &self,
    InspectNetwork { name }: InspectNetwork,
    _: (),
  ) -> anyhow::Result<Network> {
    docker_client().inspect_network(&name).await
  }
}

//

impl Resolve<CreateNetwork> for State {
  #[instrument(name = "CreateNetwork", skip(self))]
  async fn resolve(
    &self,
    CreateNetwork { name, driver }: CreateNetwork,
    _: (),
  ) -> anyhow::Result<Log> {
    let driver = match driver {
      Some(driver) => format!(" -d {driver}"),
      None => String::new(),
    };
    let command = format!("docker network create{driver} {name}");
    Ok(run_monitor_command("create network", command).await)
  }
}

//

impl Resolve<DeleteNetwork> for State {
  #[instrument(name = "DeleteNetwork", skip(self))]
  async fn resolve(
    &self,
    DeleteNetwork { name }: DeleteNetwork,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = format!("docker network rm {name}");
    Ok(run_monitor_command("delete network", command).await)
  }
}

//

impl Resolve<PruneNetworks> for State {
  #[instrument(name = "PruneNetworks", skip(self))]
  async fn resolve(
    &self,
    _: PruneNetworks,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = String::from("docker network prune -f");
    Ok(run_monitor_command("prune networks", command).await)
  }
}
