use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::I64;

use super::MonitorWriteRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateApiKeyResponse)]
pub struct CreateApiKey {
  pub name: String,

  #[serde(default)]
  pub expires: I64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateApiKeyResponse {
  /// X-API-KEY
  pub key: String,

  /// X-API-SECRET
  /// There is no way to get the secret again after it is distributed in this message
  pub secret: String,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteApiKeyResponse)]
pub struct DeleteApiKey {
  pub key: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteApiKeyResponse {}

//

/// ADMIN ONLY
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateApiKeyForServiceUserResponse)]
pub struct CreateApiKeyForServiceUser {
  /// Must be service user
  pub user_id: String,
  pub name: String,
  #[serde(default)]
  pub expires: I64,
}

#[typeshare]
pub type CreateApiKeyForServiceUserResponse = CreateApiKeyResponse;

//

/// ADMIN ONLY
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteApiKeyForServiceUserResponse {}
