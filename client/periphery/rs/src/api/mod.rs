use monitor_client::entities::{
  config::{DockerRegistry, GitProvider},
  deployment::ContainerSummary,
  server::{
    docker_image::ImageSummary, docker_network::DockerNetwork,
  },
  stack::ComposeProject,
  update::Log,
  SystemCommand,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use serror::Serror;

pub mod build;
pub mod compose;
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

/// Returns all containers, networks, images, compose projects
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetDockerListsResponse)]
pub struct GetDockerLists {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDockerListsResponse {
  pub containers: Result<Vec<ContainerSummary>, Serror>,
  pub networks: Result<Vec<DockerNetwork>, Serror>,
  pub images: Result<Vec<ImageSummary>, Serror>,
  pub projects: Result<Vec<ComposeProject>, Serror>,
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
