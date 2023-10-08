use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::I64;

use super::MonitorWriteRequest;

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request, EmptyTraits)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateLoginSecretResponse)]
pub struct CreateLoginSecret {
    pub name: String,
    pub expires: Option<I64>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateLoginSecretResponse {
    pub secret: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request, EmptyTraits)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteLoginSecretResponse)]
pub struct DeleteLoginSecret {
    pub name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteLoginSecretResponse {}
