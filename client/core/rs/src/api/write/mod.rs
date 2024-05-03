mod alerter;
mod api_key;
mod build;
mod builder;
mod deployment;
mod description;
mod launch;
mod permissions;
mod procedure;
mod repo;
mod server;
mod server_template;
mod tags;
mod user;
mod user_group;

pub use alerter::*;
pub use api_key::*;
pub use build::*;
pub use builder::*;
pub use deployment::*;
pub use description::*;
pub use launch::*;
pub use permissions::*;
pub use procedure::*;
pub use repo::*;
pub use server::*;
pub use server_template::*;
pub use tags::*;
pub use user::*;
pub use user_group::*;

pub trait MonitorWriteRequest: resolver_api::HasResponse {}
