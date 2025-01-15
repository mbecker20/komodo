use derive_empty_traits::EmptyTraits;
use resolver_api::{HasResponse, Resolve};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::user::User;

pub trait KomodoAuthRequest: HasResponse {}

/// JSON containing an authentication token.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtResponse {
  /// A token the user can use to authenticate their requests.
  pub jwt: String,
}

//

/// Non authenticated route to see the available options
/// users have to login to Komodo, eg. local auth, github, google.
/// Response: [GetLoginOptionsResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Resolve, EmptyTraits,
)]
#[empty_traits(KomodoAuthRequest)]
#[response(GetLoginOptionsResponse)]
#[error(serror::Error)]
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
  /// Whether OIDC login is enabled.
  pub oidc: bool,
  /// Whether user registration (Sign Up) has been disabled
  pub registration_disabled: bool,
}

//

/// Create a new local user account. Will fail if a user with the
/// given username already exists.
/// Response: [CreateLocalUserResponse].
///
/// Note. This method is only available if the core api has `local_auth` enabled.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Resolve, EmptyTraits,
)]
#[empty_traits(KomodoAuthRequest)]
#[response(CreateLocalUserResponse)]
#[error(serror::Error)]
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
  Serialize, Deserialize, Debug, Clone, Resolve, EmptyTraits,
)]
#[empty_traits(KomodoAuthRequest)]
#[response(LoginLocalUserResponse)]
#[error(serror::Error)]
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
  Serialize, Deserialize, Debug, Clone, Resolve, EmptyTraits,
)]
#[empty_traits(KomodoAuthRequest)]
#[response(ExchangeForJwtResponse)]
#[error(serror::Error)]
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
  Serialize, Deserialize, Debug, Clone, Resolve, EmptyTraits,
)]
#[empty_traits(KomodoAuthRequest)]
#[response(GetUserResponse)]
#[error(serror::Error)]
pub struct GetUser {}

#[typeshare]
pub type GetUserResponse = User;

//
