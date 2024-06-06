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
mod server_template;
mod sync;
mod tag;
mod toml;
mod update;
mod user;
mod user_group;
mod variable;

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
pub use server_template::*;
pub use sync::*;
pub use tag::*;
pub use toml::*;
pub use update::*;
pub use user::*;
pub use user_group::*;
pub use variable::*;

use crate::entities::Timelength;

pub trait MonitorReadRequest: HasResponse {}

//

/// Get the version of the core api.
/// Response: [GetVersionResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetVersionResponse)]
pub struct GetVersion {}

/// Response for [GetVersion].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
  /// The version of the core api.
  pub version: String,
}

//

/// Get info about the core api.
/// Response: [GetCoreInfoResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetCoreInfoResponse)]
pub struct GetCoreInfo {}

/// Response for [GetCoreInfo].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCoreInfoResponse {
  /// The title assigned to this core api.
  pub title: String,
  /// The monitoring interval of this core api.
  pub monitoring_interval: Timelength,
  /// The github webhook base url to use with github webhooks.
  pub github_webhook_base_url: String,
  /// Whether transparent mode is enabled, which gives all users read access to all resources.
  pub transparent_mode: bool,
  /// Whether UI write access should be disabled
  pub ui_write_disabled: bool,
}
