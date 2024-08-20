use monitor_client::entities::{
  docker::volume::{Volume, VolumeListItem},
  update::Log,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<VolumeListItem>)]
pub struct GetVolumeList {}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Volume)]
pub struct InspectVolume {
  pub name: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneVolumes {}
