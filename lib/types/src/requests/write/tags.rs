use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
    tag::{CustomTag, PartialCustomTag, TagColor},
    update::ResourceTarget,
};

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
    pub color: TagColor,
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
    pub config: _PartialCustomTag,
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
