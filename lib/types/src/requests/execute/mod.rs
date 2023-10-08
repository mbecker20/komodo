mod build;
mod deployment;
mod repo;
mod server;

pub use build::*;
pub use deployment::*;
pub use repo::*;
use resolver_api::HasResponse;
pub use server::*;

pub trait MonitorExecuteRequest: HasResponse {}