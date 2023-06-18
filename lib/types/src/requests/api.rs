use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::I64;

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
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
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(())]
pub struct DeleteLoginSecret {
    pub name: String,
}

//
