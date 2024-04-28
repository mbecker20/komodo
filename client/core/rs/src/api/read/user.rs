use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{api_key::ApiKey, user::User};

use super::MonitorReadRequest;

/// Gets list of api keys for the calling user.
/// Response: [ListApiKeysResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListApiKeysResponse)]
pub struct ListApiKeys {}

#[typeshare]
pub type ListApiKeysResponse = Vec<ApiKey>;

//

/// Gets list of api keys for the user at ID.
/// **Admin only.**
/// Will still fail if you call for a user_id that isn't a service user.
/// Response: [ListApiKeysForServiceUserResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListApiKeysForServiceUserResponse)]
pub struct ListApiKeysForServiceUser {
  /// The id of the user.
  pub user_id: String,
}

#[typeshare]
pub type ListApiKeysForServiceUserResponse = Vec<ApiKey>;

//

/// Gets list of monitor users.
/// **Admin only.**
/// Response: [ListUsersResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListUsersResponse)]
pub struct ListUsers {}

#[typeshare]
pub type ListUsersResponse = Vec<User>;

//

/// Gets the username of a specific user.
/// Response: [GetUsernameResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetUsernameResponse)]
pub struct GetUsername {
  /// The id of the user.
  pub user_id: String,
}

/// Response for [GetUsername].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUsernameResponse {
  /// The username of the user.
  pub username: String,
}
