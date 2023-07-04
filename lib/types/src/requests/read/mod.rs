use resolver_api::derive::Request;
use serde::{Serialize, Deserialize};

mod build;
pub use build::*;
mod builder;
pub use builder::*;
mod deployment;
pub use deployment::*;
mod repo;
pub use repo::*;
mod server;
pub use server::*;
use typeshare::typeshare;

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
