use monitor_client::entities::{
  config::core::AwsEcrConfig, update::Log,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(BuildResponse)]
pub struct Build {
  pub build: monitor_client::entities::build::Build,
  /// Override registry token with one sent from core.
  pub registry_token: Option<String>,
  /// Propogate AwsEcrConfig from core
  pub aws_ecr: Option<AwsEcrConfig>,
  /// Propogate any secret replacers from core interpolation.
  #[serde(default)]
  pub replacers: Vec<(String, String)>,
  /// Add more tags for this build in addition to the version tags.
  #[serde(default)]
  pub additional_tags: Vec<String>,
}

pub type BuildResponse = Vec<Log>;
