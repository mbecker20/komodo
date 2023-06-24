//

use monitor_macros::derive_crud_requests;
use resolver_api::derive::Request;
use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::{MongoDocument, entities::{deployment::{Deployment, PartialDeploymentConfig, TerminationSignal}, update::Update}};

//

derive_crud_requests!(Deployment);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct RenameDeployment {
	pub id: String,
	pub name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct Deploy {
	pub deployment_id: String,
	pub stop_signal: Option<TerminationSignal>,
    pub stop_time: Option<i32>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct StartContainer {
	pub deployment_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct StopContainer {
	pub deployment_id: String,
	pub signal: Option<TerminationSignal>,
    pub time: Option<i32>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct RemoveContainer {
	pub deployment_id: String,
    pub signal: Option<TerminationSignal>,
    pub time: Option<i32>,
}