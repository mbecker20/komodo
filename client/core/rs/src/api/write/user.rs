use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{update::ResourceTarget, user::User, NoData};

use super::MonitorWriteRequest;

//

/// Push a resource to the front of the users 10 most recently viewed resources.
/// Response: [NoData].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(PushRecentlyViewedResponse)]
pub struct PushRecentlyViewed {
  /// The target to push.
  pub resource: ResourceTarget,
}

#[typeshare]
pub type PushRecentlyViewedResponse = NoData;

//

/// Set the time the user last opened the UI updates.
/// Used for unseen notification dot.
/// Response: [NoData]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(SetLastSeenUpdateResponse)]
pub struct SetLastSeenUpdate {}

#[typeshare]
pub type SetLastSeenUpdateResponse = NoData;

//

/// **Admin only.** Create a service user.
/// Response: [User].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateServiceUserResponse)]
pub struct CreateServiceUser {
  /// The username for the service user.
  pub username: String,
  /// A description for the service user.
  pub description: String,
}

#[typeshare]
pub type CreateServiceUserResponse = User;

//

/// **Admin only.** Update a service user's description.
/// Response: [User].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateServiceUserDescriptionResponse)]
pub struct UpdateServiceUserDescription {
  /// The service user's username
  pub username: String,
  /// A new description for the service user.
  pub description: String,
}

#[typeshare]
pub type UpdateServiceUserDescriptionResponse = User;
