use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

mod build;
mod builder;
mod deployment;
mod repo;
mod server;

pub use build::*;
pub use builder::*;
pub use deployment::*;
pub use repo::*;
pub use server::*;

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetVersionResponse)]
pub struct GetVersion {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
    pub version: String,
}
