use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::provider::*;

use super::MonitorWriteRequest;

/// **Admin only.** Create a git provider account.
/// Response: [GitProviderAccount].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateGitProviderAccountResponse)]
pub struct CreateGitProviderAccount {
  /// The initial account config. Anything in the _id field will be ignored,
  /// as this is generated on creation.
  pub account: _PartialGitProviderAccount,
}

#[typeshare]
pub type CreateGitProviderAccountResponse = GitProviderAccount;

//

/// **Admin only.** Update a git provider account.
/// Response: [GitProviderAccount].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateGitProviderAccountResponse)]
pub struct UpdateGitProviderAccount {
  /// The id of the git provider account to update.
  pub id: String,
  /// The partial git provider account.
  pub account: _PartialGitProviderAccount,
}

#[typeshare]
pub type UpdateGitProviderAccountResponse = GitProviderAccount;

//

/// **Admin only.** Delete a git provider account.
/// Response: [User].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteGitProviderAccountResponse)]
pub struct DeleteGitProviderAccount {
  /// The id of the git provider to delete
  pub id: String,
}

#[typeshare]
pub type DeleteGitProviderAccountResponse = GitProviderAccount;

//

/// **Admin only.** Create a docker registry account.
/// Response: [DockerRegistryAccount].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateDockerRegistryAccountResponse)]
pub struct CreateDockerRegistryAccount {
  pub account: _PartialDockerRegistryAccount,
}

#[typeshare]
pub type CreateDockerRegistryAccountResponse = DockerRegistryAccount;

//

/// **Admin only.** Update a docker registry account.
/// Response: [DockerRegistryAccount].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateDockerRegistryAccountResponse)]
pub struct UpdateDockerRegistryAccount {
  /// The id of the docker registry to update
  pub id: String,
	/// The partial docker registry account.
  pub account: _PartialDockerRegistryAccount,
}

#[typeshare]
pub type UpdateDockerRegistryAccountResponse = DockerRegistryAccount;

//

/// **Admin only.** Delete a docker registry account.
/// Response: [DockerRegistryAccount].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteDockerRegistryAccountResponse)]
pub struct DeleteDockerRegistryAccount {
  /// The id of the docker registry account to delete
  pub id: String,
}

#[typeshare]
pub type DeleteDockerRegistryAccountResponse = DockerRegistryAccount;
