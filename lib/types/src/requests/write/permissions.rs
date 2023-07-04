use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
    update::{ResourceTarget, Update},
    PermissionLevel,
};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct UpdateUserPermissionsOnTarget {
    pub user_id: String,
    pub permission: PermissionLevel,
    pub target: ResourceTarget,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct UpdateUserPermissions {
    pub user_id: String,
    pub enabled: Option<bool>,
    pub create_servers: Option<bool>,
    pub create_builds: Option<bool>,
}