use derive_empty_traits::EmptyTraits;
use resolver_api::{derive::Request, HasResponse};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

mod alert;
mod alerter;
mod build;
mod builder;
mod deployment;
mod permission;
mod procedure;
mod repo;
mod search;
mod server;
mod tag;
mod toml;
mod update;
mod user;
mod user_group;

pub use alert::*;
pub use alerter::*;
pub use build::*;
pub use builder::*;
pub use deployment::*;
pub use permission::*;
pub use procedure::*;
pub use repo::*;
pub use search::*;
pub use server::*;
pub use tag::*;
pub use toml::*;
pub use update::*;
pub use user::*;
pub use user_group::*;

use crate::entities::Timelength;

pub trait MonitorReadRequest: HasResponse {}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetVersionResponse)]
pub struct GetVersion {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
  pub version: String,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetCoreInfoResponse)]
pub struct GetCoreInfo {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCoreInfoResponse {
  pub title: String,
  pub monitoring_interval: Timelength,
  pub github_webhook_base_url: String,
}
