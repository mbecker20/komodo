use crate::{Build, Deployment, PermissionLevel, PermissionsMap, Server};

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