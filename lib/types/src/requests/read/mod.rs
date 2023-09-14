use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

mod alert;
mod alerter;
mod build;
mod builder;
mod deployment;
mod repo;
mod search;
mod server;
mod tag;
mod update;

pub use alert::*;
pub use alerter::*;
pub use build::*;
pub use builder::*;
pub use deployment::*;
pub use repo::*;
pub use search::*;
pub use server::*;
pub use tag::*;
pub use update::*;

use crate::entities::{user::User, Timelength};

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

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<User>)]
pub struct GetUsers {}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetUsernameResponse)]
pub struct GetUsername {
    pub user_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUsernameResponse {
    pub username: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetCoreInfoResponse)]
pub struct GetCoreInfo {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCoreInfoResponse {
    pub title: String,
    pub monitoring_interval: Timelength,
}
