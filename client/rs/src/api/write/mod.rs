mod alerter;
mod build;
mod builder;
mod deployment;
mod description;
mod launch;
mod permissions;
mod procedure;
mod repo;
mod secret;
mod server;
mod tags;
mod user;

pub use alerter::*;
pub use build::*;
pub use builder::*;
pub use deployment::*;
pub use description::*;
pub use launch::*;
pub use permissions::*;
pub use procedure::*;
pub use repo::*;
pub use secret::*;
pub use server::*;
pub use tags::*;
pub use user::*;

pub trait MonitorWriteRequest: resolver_api::HasResponse {}
