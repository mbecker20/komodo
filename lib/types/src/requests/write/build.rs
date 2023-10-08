use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::build::{Build, PartialBuildConfig};

use super::MonitorWriteRequest;

#[typeshare(serialized_as = "Partial<BuildConfig>")]
type _PartialBuildConfig = PartialBuildConfig;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Build)]
pub struct CreateBuild {
    pub name: String,
    pub config: _PartialBuildConfig,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Build)]
pub struct CopyBuild {
    pub name: String,
    pub id: String,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Build)]
pub struct DeleteBuild {
    pub id: String,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Build)]
pub struct UpdateBuild {
    pub id: String,
    pub config: _PartialBuildConfig,
}
