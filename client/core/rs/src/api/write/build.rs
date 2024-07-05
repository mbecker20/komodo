use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  build::{Build, _PartialBuildConfig},
  NoData,
};

use super::MonitorWriteRequest;

//

/// Create a build. Response: [Build].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Build)]
pub struct CreateBuild {
  /// The name given to newly created build.
  pub name: String,
  /// Optional partial config to initialize the build with.
  pub config: _PartialBuildConfig,
}

//

/// Creates a new build with given `name` and the configuration
/// of the build at the given `id`. Response: [Build].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Build)]
pub struct CopyBuild {
  /// The name of the new build.
  pub name: String,
  /// The id of the build to copy.
  pub id: String,
}

//

/// Deletes the build at the given id, and returns the deleted build.
/// Response: [Build]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Build)]
pub struct DeleteBuild {
  /// The id or name of the build to delete.
  pub id: String,
}

//

/// Update the build at the given id, and return the updated build.
/// Response: [Build].
///
/// Note. This method updates only the fields which are set in the [_PartialBuildConfig],
/// effectively merging diffs into the final document.
/// This is helpful when multiple users are using
/// the same resources concurrently by ensuring no unintentional
/// field changes occur from out of date local state.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Build)]
pub struct UpdateBuild {
  /// The id of the build to update.
  pub id: String,
  /// The partial config update to apply.
  pub config: _PartialBuildConfig,
}

//

/// Create a webhook on the github repo attached to the build
/// passed in request. Response: [CreateBuildWebhookResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateBuildWebhookResponse)]
pub struct CreateBuildWebhook {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub build: String,
}

#[typeshare]
pub type CreateBuildWebhookResponse = NoData;

//

/// Delete a webhook on the github repo attached to the build
/// passed in request. Response: [CreateBuildWebhookResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteBuildWebhookResponse)]
pub struct DeleteBuildWebhook {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub build: String,
}

#[typeshare]
pub type DeleteBuildWebhookResponse = NoData;
