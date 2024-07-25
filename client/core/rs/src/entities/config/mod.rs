use serde::{Deserialize, Serialize};
use typeshare::typeshare;

pub mod core;
pub mod periphery;

#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Eq,
  Hash,
  PartialOrd,
  Ord,
  Serialize,
  Deserialize,
)]
pub struct GitAccount {
  /// The git provider domain. Default: `github.com`.
  #[serde(default = "default_git_provider")]
  pub provider: String,
  /// The account username. Required.
  #[serde(alias = "account")]
  pub username: String,
  /// The account access token for private repos. Required.
  #[serde(default, skip_serializing)]
  pub token: String,
}

fn default_git_provider() -> String {
  String::from("github.com")
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Eq,
  Hash,
  PartialOrd,
  Ord,
  Serialize,
  Deserialize,
)]
pub struct DockerAccount {
  /// The docker provider domain. Default: `docker.io`.
  #[serde(default = "default_docker_provider")]
  pub provider: String,
  /// The account username. Required.
  #[serde(alias = "account")]
  pub username: String,
  /// The account access token for private images.
  #[serde(default, skip_serializing)]
  pub token: String,
}

fn default_docker_provider() -> String {
  String::from("docker.io")
}
