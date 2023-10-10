use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::builder::{Builder, PartialBuilderConfig};

use super::MonitorWriteRequest;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Builder)]
pub struct CreateBuilder {
    pub name: String,
    pub config: PartialBuilderConfig,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Builder)]
pub struct CopyBuilder {
    pub name: String,
    pub id: String,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Builder)]
pub struct DeleteBuilder {
    pub id: String,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Builder)]
pub struct UpdateBuilder {
    pub id: String,
    pub config: PartialBuilderConfig,
}
