use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
    deployment::TerminationSignal, update::Update,
};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct Deploy {
    pub deployment_id: String,
    pub stop_signal: Option<TerminationSignal>,
    pub stop_time: Option<i32>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct StartContainer {
    pub deployment_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct StopContainer {
    pub deployment_id: String,
    pub signal: Option<TerminationSignal>,
    pub time: Option<i32>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct RemoveContainer {
    pub deployment_id: String,
    pub signal: Option<TerminationSignal>,
    pub time: Option<i32>,
}
