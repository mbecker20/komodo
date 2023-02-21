use crate::{
    Build, BuildActionState, CloneArgs, Deployment, DeploymentActionState, Group, PermissionLevel,
    PermissionsMap, Procedure, Server, ServerActionState,
};

pub trait Permissioned {
    fn permissions_map(&self) -> &PermissionsMap;

    fn get_user_permissions(&self, user_id: &str) -> PermissionLevel {
        *self.permissions_map().get(user_id).unwrap_or_default()
    }
}

impl Permissioned for Deployment {
    fn permissions_map(&self) -> &PermissionsMap {
        &self.permissions
    }
}

impl Permissioned for Build {
    fn permissions_map(&self) -> &PermissionsMap {
        &self.permissions
    }
}

impl Permissioned for Server {
    fn permissions_map(&self) -> &PermissionsMap {
        &self.permissions
    }
}

impl Permissioned for Procedure {
    fn permissions_map(&self) -> &PermissionsMap {
        &self.permissions
    }
}

impl Permissioned for Group {
    fn permissions_map(&self) -> &PermissionsMap {
        &self.permissions
    }
}

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
    }
}

impl Busy for BuildActionState {
    fn busy(&self) -> bool {
        self.building || self.updating
    }
}

impl From<&Deployment> for CloneArgs {
    fn from(d: &Deployment) -> Self {
        CloneArgs {
            name: d.name.clone(),
            repo: d.repo.clone(),
            branch: d.branch.clone(),
            on_clone: d.on_clone.clone(),
            on_pull: d.on_pull.clone(),
            github_account: d.github_account.clone(),
        }
    }
}

impl From<&Build> for CloneArgs {
    fn from(b: &Build) -> Self {
        CloneArgs {
            name: b.name.clone(),
            repo: b.repo.clone(),
            branch: b.branch.clone(),
            on_clone: b.pre_build.clone(),
            on_pull: None,
            github_account: b.github_account.clone(),
        }
    }
}
