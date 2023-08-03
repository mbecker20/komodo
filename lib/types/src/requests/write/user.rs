use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::ResourceTarget;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(PushRecentlyViewedResponse)]
pub struct PushRecentlyViewed {
    pub resource: ResourceTarget,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushRecentlyViewedResponse {}
