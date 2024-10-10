use command::run_komodo_command;
use komodo_client::entities::{
  docker::network::Network, update::Log,
};
use periphery_client::api::network::*;
use resolver_api::Resolve;

use crate::{docker::docker_client, State};

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
    Ok(run_komodo_command("create network", None, command).await)
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
    Ok(run_komodo_command("delete network", None, command).await)
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
    Ok(run_komodo_command("prune networks", None, command).await)
  }
}
