use monitor_client::entities::{
  config::{DockerRegistry, GitProvider},
  update::Log,
  SystemCommand,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

pub mod build;
pub mod container;
pub mod git;
pub mod network;
pub mod stats;

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetHealthResponse)]
pub struct GetHealth {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHealthResponse {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetVersionResponse)]
pub struct GetVersion {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
  pub version: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(ListGitProvidersResponse)]
pub struct ListGitProviders {}

pub type ListGitProvidersResponse = Vec<GitProvider>;

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(ListDockerRegistriesResponse)]
pub struct ListDockerRegistries {}

pub type ListDockerRegistriesResponse = Vec<DockerRegistry>;

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<String>)]
pub struct ListSecrets {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneSystem {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct RunCommand {
  pub command: SystemCommand,
}
