use command::run_komodo_command;
use komodo_client::entities::{docker::volume::Volume, update::Log};
use periphery_client::api::volume::*;
use resolver_api::Resolve;

use crate::{docker::docker_client, State};

//

impl Resolve<InspectVolume> for State {
  #[instrument(name = "InspectVolume", level = "debug", skip(self))]
  async fn resolve(
    &self,
    InspectVolume { name }: InspectVolume,
    _: (),
  ) -> anyhow::Result<Volume> {
    docker_client().inspect_volume(&name).await
  }
}

//

impl Resolve<DeleteVolume> for State {
  #[instrument(name = "DeleteVolume", skip(self))]
  async fn resolve(
    &self,
    DeleteVolume { name }: DeleteVolume,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = format!("docker volume rm {name}");
    Ok(run_komodo_command("delete volume", command).await)
  }
}

//

impl Resolve<PruneVolumes> for State {
  #[instrument(name = "PruneVolumes", skip(self))]
  async fn resolve(
    &self,
    _: PruneVolumes,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = String::from("docker volume prune -a -f");
    Ok(run_komodo_command("prune volumes", command).await)
  }
}
