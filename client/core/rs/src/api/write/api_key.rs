use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{NoData, I64};

use super::MonitorWriteRequest;

//

/// Create an api key for the calling user.
/// Response: [CreateApiKeyResponse].
///
/// Note. After the response is served, there will be no way
/// to get the secret later.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateApiKeyResponse)]
pub struct CreateApiKey {
  /// The name for the api key.
  pub name: String,

  /// A unix timestamp in millseconds specifying api key expire time.
  /// Default is 0, which means no expiry.
  #[serde(default)]
  pub expires: I64,
}

/// Response for [CreateApiKey].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateApiKeyResponse {
  /// X-API-KEY
  pub key: String,

  /// X-API-SECRET
  ///
  /// Note.
  /// There is no way to get the secret again after it is distributed in this message
  pub secret: String,
}

//

/// Delete an api key for the calling user.
/// Response: [NoData]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteApiKeyResponse)]
pub struct DeleteApiKey {
  /// The key which the user intends to delete.
  pub key: String,
}

#[typeshare]
pub type DeleteApiKeyResponse = NoData;

//

/// Admin only method to create an api key for a service user.
/// Response: [CreateApiKeyResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateApiKeyForServiceUserResponse)]
pub struct CreateApiKeyForServiceUser {
  /// Must be service user
  pub user_id: String,
  /// The name for the api key
  pub name: String,
  /// A unix timestamp in millseconds specifying api key expire time.
  /// Default is 0, which means no expiry.
  #[serde(default)]
  pub expires: I64,
}

#[typeshare]
pub type CreateApiKeyForServiceUserResponse = CreateApiKeyResponse;

//

/// Admin only method to delete an api key for a service user.
/// Response: [NoData].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteApiKeyForServiceUserResponse)]
pub struct DeleteApiKeyForServiceUser {
  pub key: String,
}

#[typeshare]
pub type DeleteApiKeyForServiceUserResponse = NoData;
