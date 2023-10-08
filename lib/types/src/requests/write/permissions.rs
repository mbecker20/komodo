use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
    update::{ResourceTarget, Update},
    PermissionLevel,
};

use super::MonitorWriteRequest;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request, EmptyTraits)]
#[empty_traits(MonitorWriteRequest)]
#[response(Update)]
pub struct UpdateUserPermissionsOnTarget {
    pub user_id: String,
    pub permission: PermissionLevel,
    pub target: ResourceTarget,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request, EmptyTraits)]
#[empty_traits(MonitorWriteRequest)]
#[response(Update)]
pub struct UpdateUserPermissions {
    pub user_id: String,
    pub enabled: Option<bool>,
    pub create_servers: Option<bool>,
    pub create_builds: Option<bool>,
}
