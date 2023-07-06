use resolver_api::derive::Request;
use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::entities::update::ResourceTarget;

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(())]
pub struct AddTags {
	pub target: ResourceTarget,
	pub tags: Vec<String>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(())]
pub struct RemoveTags {
	pub target: ResourceTarget,
	pub tags: Vec<String>,
}
