use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::I64;

mod server;
pub use server::*;

mod deployment;
pub use deployment::*;

mod build;
pub use build::*;

mod builder;
pub use builder::*;

mod permissions;
pub use permissions::*;

mod repo;
pub use repo::*;

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

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetVersionResponse)]
pub struct GetVersion {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
    pub version: String,
}
