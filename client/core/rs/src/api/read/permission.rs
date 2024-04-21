use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::permission::Permission;

use super::MonitorReadRequest;

/// List permissions for the calling user. Response: [ListPermissionsResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListPermissionsResponse)]
pub struct ListPermissions {}

#[typeshare]
pub type ListPermissionsResponse = Vec<Permission>;

//

/// List permissions for a specific user. Admin only. Response: [ListUserPermissionsResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListUserPermissionsResponse)]
pub struct ListUserPermissions {
  pub user_id: String,
}

#[typeshare]
pub type ListUserPermissionsResponse = Vec<Permission>;
