use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  tag::{CustomTag, TagColor, _PartialCustomTag},
  update::ResourceTarget,
};

use super::MonitorWriteRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CustomTag)]
pub struct CreateTag {
  pub name: String,

  #[serde(default)]
  pub category: String,

  #[serde(default)]
  pub color: TagColor,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CustomTag)]
pub struct DeleteTag {
  pub id: String,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CustomTag)]
pub struct UpdateTag {
  pub id: String,
  pub config: _PartialCustomTag,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateTagsOnResourceResponse)]
pub struct UpdateTagsOnResource {
  pub target: ResourceTarget,
  pub tags: Vec<String>, // custom tag ids
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTagsOnResourceResponse {}

//
