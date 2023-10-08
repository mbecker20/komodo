use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
    deployment::{Deployment, PartialDeploymentConfig},
    update::Update,
};

use super::MonitorWriteRequest;

#[typeshare(serialized_as = "Partial<DeploymentConfig>")]
type _PartialDeploymentConfig = PartialDeploymentConfig;

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request, EmptyTraits)]
#[empty_traits(MonitorWriteRequest)]
#[response(Deployment)]
pub struct CreateDeployment {
    pub name: String,
    pub config: _PartialDeploymentConfig,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request, EmptyTraits)]
#[empty_traits(MonitorWriteRequest)]
#[response(Deployment)]
pub struct CopyDeployment {
    pub name: String,
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request, EmptyTraits)]
#[empty_traits(MonitorWriteRequest)]
#[response(Deployment)]
pub struct DeleteDeployment {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request, EmptyTraits)]
#[empty_traits(MonitorWriteRequest)]
#[response(Deployment)]
pub struct UpdateDeployment {
    pub id: String,
    pub config: _PartialDeploymentConfig,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request, EmptyTraits)]
#[empty_traits(MonitorWriteRequest)]
#[response(Update)]
pub struct RenameDeployment {
    pub id: String,
    pub name: String,
}
