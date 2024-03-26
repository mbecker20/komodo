use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{update::ResourceTarget, user::User};

use super::MonitorWriteRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(PushRecentlyViewedResponse)]
pub struct PushRecentlyViewed {
  pub resource: ResourceTarget,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushRecentlyViewedResponse {}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(SetLastSeenUpdateResponse)]
pub struct SetLastSeenUpdate {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetLastSeenUpdateResponse {}

//

/// ADMIN ONLY
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateServiceUserResponse)]
pub struct CreateServiceUser {
  pub username: String,
  pub description: String,
}

#[typeshare]
pub type CreateServiceUserResponse = User;

//

/// ADMIN ONLY
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateServiceUserDescriptionResponse)]
pub struct UpdateServiceUserDescription {
  pub username: String,
  pub description: String,
}

#[typeshare]
pub type UpdateServiceUserDescriptionResponse = User;
