use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::permission::Permission;

use super::MonitorReadRequest;

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
