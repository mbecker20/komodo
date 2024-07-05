use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  repo::{Repo, _PartialRepoConfig},
  NoData,
};

use super::MonitorWriteRequest;

//

/// Create a repo. Response: [Repo].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Repo)]
pub struct CreateRepo {
  /// The name given to newly created repo.
  pub name: String,
  /// Optional partial config to initialize the repo with.
  pub config: _PartialRepoConfig,
}

//

/// Creates a new repo with given `name` and the configuration
/// of the repo at the given `id`. Response: [Repo].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Repo)]
pub struct CopyRepo {
  /// The name of the new repo.
  pub name: String,
  /// The id of the repo to copy.
  pub id: String,
}

//

/// Deletes the repo at the given id, and returns the deleted repo.
/// Response: [Repo]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Repo)]
pub struct DeleteRepo {
  /// The id or name of the repo to delete.
  pub id: String,
}

//

/// Update the repo at the given id, and return the updated repo.
/// Response: [Repo].
///
/// Note. If the attached server for the repo changes,
/// the repo will be deleted / cleaned up on the old server.
///
/// Note. This method updates only the fields which are set in the [_PartialRepoConfig],
/// effectively merging diffs into the final document.
/// This is helpful when multiple users are using
/// the same resources concurrently by ensuring no unintentional
/// field changes occur from out of date local state.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Repo)]
pub struct UpdateRepo {
  /// The id of the repo to update.
  pub id: String,
  /// The partial config update to apply.
  pub config: _PartialRepoConfig,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateRepoWebhookResponse)]
pub struct CreateRepoWebhook {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub repo: String,
}

#[typeshare]
pub type CreateRepoWebhookResponse = NoData;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteRepoWebhookResponse)]
pub struct DeleteRepoWebhook {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub repo: String,
}

#[typeshare]
pub type DeleteRepoWebhookResponse = NoData;