use derive_empty_traits::EmptyTraits;
use resolver_api::{derive::Request, HasResponse};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::user::User;

pub trait MonitorAuthRequest: HasResponse {}

/// JSON containing an authentication token.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtResponse {
  /// A token the user can use to authenticate their requests.
  pub jwt: String,
}

//

/// Non authenticated route to see the available options
/// users have to login to monitor, eg. local auth, github, google.
/// Response: [GetLoginOptionsResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorAuthRequest)]
#[response(GetLoginOptionsResponse)]
pub struct GetLoginOptions {}

/// The response for [GetLoginOptions].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GetLoginOptionsResponse {
  /// Whether local auth is enabled.
  pub local: bool,
  /// Whether github login is enabled.
  pub github: bool,
  /// Whether google login is enabled.
  pub google: bool,
}

//

/// Create a new local user account. Will fail if a user with the
/// given username already exists.
/// Response: [CreateLocalUserResponse].
///
/// Note. This method is only available if the core api has `local_auth` enabled.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorAuthRequest)]
#[response(CreateLocalUserResponse)]
pub struct CreateLocalUser {
  /// The username for the new user.
  pub username: String,
  /// The password for the new user.
  /// This cannot be retreived later.
  pub password: String,
}

/// Response for [CreateLocalUser].
#[typeshare]
pub type CreateLocalUserResponse = JwtResponse;

//

/// Login as a local user. Will fail if the users credentials don't match
/// any local user.
///
/// Note. This method is only available if the core api has `local_auth` enabled.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorAuthRequest)]
#[response(LoginLocalUserResponse)]
pub struct LoginLocalUser {
  /// The user's username
  pub username: String,
  /// The user's password
  pub password: String,
}

/// The response for [LoginLocalUser]
#[typeshare]
pub type LoginLocalUserResponse = JwtResponse;

//

/// Exchange a single use exchange token (safe for transport in url query)
/// for a jwt.
/// Response: [ExchangeForJwtResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorAuthRequest)]
#[response(ExchangeForJwtResponse)]
pub struct ExchangeForJwt {
  /// The 'exchange token'
  pub token: String,
}

/// Response for [ExchangeForJwt].
#[typeshare]
pub type ExchangeForJwtResponse = JwtResponse;

//

/// Get the user extracted from the request headers.
/// Response: [User].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorAuthRequest)]
#[response(GetUserResponse)]
pub struct GetUser {}

#[typeshare]
pub type GetUserResponse = User;

//
