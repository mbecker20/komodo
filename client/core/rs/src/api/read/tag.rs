use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{tag::Tag, MongoDocument};

use super::MonitorReadRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetTagResponse)]
pub struct GetTag {
  pub id: String,
}

#[typeshare]
pub type GetTagResponse = Tag;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListTagsResponse)]
pub struct ListTags {
  pub query: Option<MongoDocument>,
}

#[typeshare]
pub type ListTagsResponse = Vec<Tag>;
