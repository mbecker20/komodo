use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::entities::I64;

#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct ImageListItem {
  /// The first tag in `repo_tags`, or Id if no tags.
  #[serde(default)]
  pub name: String,

  /// ID is the content-addressable ID of an image.  This identifier is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image.
  #[serde(default)]
  pub id: String,

  /// ID of the parent image.  Depending on how the image was created, this field may be empty and is only set for images that were built/created locally. This field is empty if the image was pulled from an image registry.
  #[serde(default)]
  pub parent_id: String,

  /// List of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same image, and this list may be empty if no tags reference the image, in which case the image is \"untagged\", in which case it can still be referenced by its ID.
  #[serde(default)]
  pub repo_tags: Vec<String>,

  /// List of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated.
  #[serde(default)]
  pub repo_digests: Vec<String>,

  /// Date and time at which the image was created as a Unix timestamp (number of seconds sinds EPOCH).
  #[serde(default)]
  pub created: I64,

  /// Total size of the image including all layers it is composed of.
  #[serde(default)]
  pub size: I64,

  /// Total size of image layers that are shared between this image and other images.  This size is not calculated by default. `-1` indicates that the value has not been set / calculated.
  #[serde(default)]
  pub shared_size: I64,

  /// User-defined key/value metadata.
  #[serde(default)]
  pub labels: HashMap<String, String>,

  /// Number of containers using this image. Includes both stopped and running containers.  This size is not calculated by default, and depends on which API endpoint is used. `-1` indicates that the value has not been set / calculated.
  #[serde(default)]
  pub containers: I64,
}

/// Information about an image in the local image cache.
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct Image {
  /// ID is the content-addressable ID of an image.  This identifier is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image.
  #[serde(rename = "Id")]
  pub id: Option<String>,

  /// List of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same image, and this list may be empty if no tags reference the image, in which case the image is \"untagged\", in which case it can still be referenced by its ID.
  #[serde(default, rename = "RepoTags")]
  pub repo_tags: Vec<String>,

  /// List of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated.
  #[serde(default, rename = "RepoDigests")]
  pub repo_digests: Vec<String>,

  /// ID of the parent image.  Depending on how the image was created, this field may be empty and is only set for images that were built/created locally. This field is empty if the image was pulled from an image registry.
  #[serde(rename = "Parent")]
  pub parent: Option<String>,

  /// Optional message that was set when committing or importing the image.
  #[serde(rename = "Comment")]
  pub comment: Option<String>,

  /// Date and time at which the image was created, formatted in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  This information is only available if present in the image, and omitted otherwise.
  #[serde(rename = "Created")]
  pub created: Option<String>,

  /// The version of Docker that was used to build the image.  Depending on how the image was created, this field may be empty.
  #[serde(rename = "DockerVersion")]
  pub docker_version: Option<String>,

  /// Name of the author that was specified when committing the image, or as specified through MAINTAINER (deprecated) in the Dockerfile.
  #[serde(rename = "Author")]
  pub author: Option<String>,

  /// Hardware CPU architecture that the image runs on.
  #[serde(rename = "Architecture")]
  pub architecture: Option<String>,

  /// CPU architecture variant (presently ARM-only).
  #[serde(rename = "Variant")]
  pub variant: Option<String>,

  /// Operating System the image is built to run on.
  #[serde(rename = "Os")]
  pub os: Option<String>,

  /// Operating System version the image is built to run on (especially for Windows).
  #[serde(rename = "OsVersion")]
  pub os_version: Option<String>,

  /// Total size of the image including all layers it is composed of.
  #[serde(rename = "Size")]
  pub size: Option<I64>,

  #[serde(rename = "GraphDriver")]
  pub graph_driver: Option<GraphDriverData>,

  #[serde(rename = "RootFS")]
  pub root_fs: Option<ImageInspectRootFs>,

  #[serde(rename = "Metadata")]
  pub metadata: Option<ImageInspectMetadata>,
}

/// Information about the storage driver used to store the container's and image's filesystem.
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct GraphDriverData {
  /// Name of the storage driver.
  #[serde(default, rename = "Name")]
  pub name: String,
  /// Low-level storage metadata, provided as key/value pairs.  This information is driver-specific, and depends on the storage-driver in use, and should be used for informational purposes only.
  #[serde(default, rename = "Data")]
  pub data: HashMap<String, String>,
}

/// Information about the image's RootFS, including the layer IDs.
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct ImageInspectRootFs {
  #[serde(default, rename = "Type")]
  pub typ: String,

  #[serde(default, rename = "Layers")]
  pub layers: Vec<String>,
}

/// Additional metadata of the image in the local cache. This information is local to the daemon, and not part of the image itself.
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct ImageInspectMetadata {
  /// Date and time at which the image was last tagged in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  This information is only available if the image was tagged locally, and omitted otherwise.
  #[serde(rename = "LastTagTime")]
  pub last_tag_time: Option<String>,
}
