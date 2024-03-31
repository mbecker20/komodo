use derive_variants::EnumVariants;
use resolver_api::HasResponse;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

mod build;
mod deployment;
mod procedure;
mod repo;
mod server;

pub use build::*;
pub use deployment::*;
pub use procedure::*;
pub use repo::*;
pub use server::*;

pub trait MonitorExecuteRequest: HasResponse {}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, EnumVariants)]
#[variant_derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum Execution {
  /// For new executions upon instantiation
  None(None),

  // PROCEDURE
  RunProcedure(RunProcedure),

  // BUILD
  RunBuild(RunBuild),

  // DEPLOYMENT
  Deploy(Deploy),
  StartContainer(StartContainer),
  StopContainer(StopContainer),
  StopAllContainers(StopAllContainers),
  RemoveContainer(RemoveContainer),

  // REPO
  CloneRepo(CloneRepo),
  PullRepo(PullRepo),

  // SERVER
  PruneDockerNetworks(PruneDockerNetworks),
  PruneDockerImages(PruneDockerImages),
  PruneDockerContainers(PruneDockerContainers),
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct None {}
