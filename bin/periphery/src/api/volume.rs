use command::run_monitor_command;
use monitor_client::entities::{
  docker::volume::{Volume, VolumeListItem},
  update::Log,
};
use periphery_client::api::volume::{
  GetVolumeList, InspectVolume, PruneVolumes,
};
use resolver_api::Resolve;

use crate::{docker::docker_client, State};

//

impl Resolve<GetVolumeList> for State {
  #[instrument(name = "GetVolumeList", level = "debug", skip(self))]
  async fn resolve(
    &self,
    _: GetVolumeList,
    _: (),
  ) -> anyhow::Result<Vec<VolumeListItem>> {
    docker_client().list_volumes().await
  }
}

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

impl Resolve<PruneVolumes> for State {
  #[instrument(name = "PruneVolumes", skip(self))]
  async fn resolve(
    &self,
    _: PruneVolumes,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = String::from("docker volume prune -a -f");
    Ok(run_monitor_command("prune volumes", command).await)
  }
}
