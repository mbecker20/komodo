use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::ResourceTarget;

use super::MonitorWriteRequest;

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateDescriptionResponse)]
pub struct UpdateDescription {
    pub target: ResourceTarget,
    pub description: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDescriptionResponse {}
