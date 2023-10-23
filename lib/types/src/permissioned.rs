use crate::entities::{
  resource::Resource, PermissionLevel, PermissionsMap,
};

pub trait Permissioned {
  fn permissions_map(&self) -> &PermissionsMap;

  fn get_user_permissions(&self, user_id: &str) -> PermissionLevel {
    *self.permissions_map().get(user_id).unwrap_or_default()
  }
}

impl<C, I: Default> Permissioned for Resource<C, I> {
  fn permissions_map(&self) -> &PermissionsMap {
    &self.permissions
  }
}
