use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{tag::Tag, NoData, ResourceTarget};

use super::MonitorWriteRequest;

//

/// Create a tag. Response: [Tag].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Tag)]
pub struct CreateTag {
  /// The name of the tag.
  pub name: String,
}

//

/// Delete a tag, and return the deleted tag. Response: [Tag].
///
/// Note. Will also remove this tag from all attached resources.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Tag)]
pub struct DeleteTag {
  /// The id of the tag to delete.
  pub id: String,
}

//

/// Rename a tag at id. Response: [Tag].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Tag)]
pub struct RenameTag {
  /// The id of the tag to rename.
  pub id: String,
  /// The new name of the tag.
  pub name: String,
}

//

/// Update the tags on a resource.
/// Response: [NoData]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateTagsOnResourceResponse)]
pub struct UpdateTagsOnResource {
  pub target: ResourceTarget,
  /// Tag Ids
  pub tags: Vec<String>,
}

#[typeshare]
pub type UpdateTagsOnResourceResponse = NoData;

//
