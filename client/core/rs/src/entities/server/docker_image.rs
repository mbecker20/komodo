use std::collections::HashMap;

use serde::{
  de::DeserializeOwned, Deserialize, Deserializer, Serialize,
};
use typeshare::typeshare;

use crate::entities::I64;

/// Summary of a docker image cached on a server
#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct ImageSummary {
  /// ID is the content-addressable ID of an image.  This identifier is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image.
  #[serde(rename = "Id")]
  pub id: String,

  /// ID of the parent image.  Depending on how the image was created, this field may be empty and is only set for images that were built/created locally. This field is empty if the image was pulled from an image registry.
  #[serde(rename = "ParentId")]
  pub parent_id: String,

  /// List of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same image, and this list may be empty if no tags reference the image, in which case the image is \"untagged\", in which case it can still be referenced by its ID.
  #[serde(rename = "RepoTags")]
  #[serde(deserialize_with = "deserialize_nonoptional_vec")]
  pub repo_tags: Vec<String>,

  /// List of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated.
  #[serde(rename = "RepoDigests")]
  #[serde(deserialize_with = "deserialize_nonoptional_vec")]
  pub repo_digests: Vec<String>,

  /// Date and time at which the image was created as a Unix timestamp (number of seconds sinds EPOCH).
  #[serde(rename = "Created")]
  pub created: I64,

  /// Total size of the image including all layers it is composed of.
  #[serde(rename = "Size")]
  pub size: I64,

  /// Total size of image layers that are shared between this image and other images.  This size is not calculated by default. `-1` indicates that the value has not been set / calculated.
  #[serde(rename = "SharedSize")]
  pub shared_size: I64,

  /// Total size of the image including all layers it is composed of.  In versions of Docker before v1.10, this field was calculated from the image itself and all of its parent images. Docker v1.10 and up store images self-contained, and no longer use a parent-chain, making this field an equivalent of the Size field.  This field is kept for backward compatibility, but may be removed in a future version of the API.
  #[serde(rename = "VirtualSize")]
  pub virtual_size: Option<I64>,

  /// User-defined key/value metadata.
  #[serde(rename = "Labels")]
  #[serde(deserialize_with = "deserialize_nonoptional_map")]
  pub labels: HashMap<String, String>,

  /// Number of containers using this image. Includes both stopped and running containers.  This size is not calculated by default, and depends on which API endpoint is used. `-1` indicates that the value has not been set / calculated.
  #[serde(rename = "Containers")]
  pub containers: I64,
}

fn deserialize_nonoptional_vec<
  'de,
  D: Deserializer<'de>,
  T: DeserializeOwned,
>(
  d: D,
) -> Result<Vec<T>, D::Error> {
  serde::Deserialize::deserialize(d)
    .map(|x: Option<_>| x.unwrap_or_default())
}

fn deserialize_nonoptional_map<
  'de,
  D: Deserializer<'de>,
  T: DeserializeOwned,
>(
  d: D,
) -> Result<HashMap<String, T>, D::Error> {
  serde::Deserialize::deserialize(d)
    .map(|x: Option<_>| x.unwrap_or_default())
}

impl From<bollard::service::ImageSummary> for ImageSummary {
  fn from(value: bollard::service::ImageSummary) -> Self {
    Self {
      id: value.id,
      parent_id: value.parent_id,
      repo_tags: value.repo_tags,
      repo_digests: value.repo_digests,
      created: value.created,
      size: value.size,
      shared_size: value.shared_size,
      virtual_size: value.virtual_size,
      labels: value.labels,
      containers: value.containers,
    }
  }
}
