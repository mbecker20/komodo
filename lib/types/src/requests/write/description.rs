use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::ResourceTarget;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(UpdateDescriptionResponse)]
pub struct UpdateDescription {
    pub target: ResourceTarget,
    pub description: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDescriptionResponse {}
