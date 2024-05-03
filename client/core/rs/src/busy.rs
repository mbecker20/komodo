use crate::entities::{
  build::BuildActionState, deployment::DeploymentActionState,
  procedure::ProcedureActionState, repo::RepoActionState,
  server::ServerActionState,
};

pub trait Busy {
  fn busy(&self) -> bool;
}

impl Busy for ServerActionState {
  fn busy(&self) -> bool {
    self.pruning_containers
      || self.pruning_images
      || self.pruning_networks
  }
}

impl Busy for DeploymentActionState {
  fn busy(&self) -> bool {
    self.deploying
      || self.removing
      || self.starting
      || self.stopping
      || self.renaming
      || self.deleting
  }
}

impl Busy for BuildActionState {
  fn busy(&self) -> bool {
    self.building
  }
}

impl Busy for RepoActionState {
  fn busy(&self) -> bool {
    self.cloning || self.pulling || self.updating || self.deleting
  }
}

impl Busy for ProcedureActionState {
  fn busy(&self) -> bool {
    self.running
  }
}
