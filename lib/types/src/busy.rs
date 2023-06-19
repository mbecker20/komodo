use crate::entities::{server::ServerActionState, deployment::DeploymentActionState, build::BuildActionState};

pub trait Busy {
    fn busy(&self) -> bool;
}

impl Busy for ServerActionState {
    fn busy(&self) -> bool {
        self.pruning_containers || self.pruning_images || self.pruning_networks
    }
}

impl Busy for DeploymentActionState {
    fn busy(&self) -> bool {
        self.deploying
            || self.pulling
            || self.recloning
            || self.removing
            || self.starting
            || self.stopping
            || self.updating
            || self.renaming
    }
}

impl Busy for BuildActionState {
    fn busy(&self) -> bool {
        self.building || self.updating
    }
}