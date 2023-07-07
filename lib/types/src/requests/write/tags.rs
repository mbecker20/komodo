use resolver_api::derive::Request;
use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::entities::{update::ResourceTarget, tag::{CustomTag, TagColor, PartialCustomTag}};

#[typeshare(serialized_as = "Partial<CustomTag>")]
type _PartialCustomTag = PartialCustomTag;

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(CustomTag)]
pub struct CreateTag {
	pub name: String,

	#[serde(default)]
	pub category: String,

	#[serde(default)]
	pub color: TagColor
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(CustomTag)]
pub struct DeleteTag {
	pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(CustomTag)]
pub struct UpdateTag {
	pub id: String,
	pub config: _PartialCustomTag
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(())]
pub struct AddTags {
	pub target: ResourceTarget,
	pub tags: Vec<String>, // custom tag ids
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(())]
pub struct RemoveTags {
	pub target: ResourceTarget,
	pub tags: Vec<String>, // custom tag ids
}
