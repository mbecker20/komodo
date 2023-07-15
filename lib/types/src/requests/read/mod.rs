use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

mod build;
mod builder;
mod deployment;
mod repo;
mod search;
mod server;
mod tag;
mod update;

pub use build::*;
pub use builder::*;
pub use deployment::*;
pub use repo::*;
pub use search::*;
pub use server::*;
pub use tag::*;
pub use update::*;

use crate::entities::user::User;

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

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(User)]
pub struct GetUser {}
