use monitor_client::entities::{
  deployment::{
    ContainerSummary, Deployment, DockerContainerStats,
    TerminationSignal,
  },
  update::Log,
  SearchCombinator,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<ContainerSummary>)]
pub struct GetContainerList {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct GetContainerLog {
  pub name: String,
  #[serde(default = "default_tail")]
  pub tail: u64,
}

fn default_tail() -> u64 {
  50
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct GetContainerLogSearch {
  pub name: String,
  pub terms: Vec<String>,
  #[serde(default)]
  pub combinator: SearchCombinator,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(DockerContainerStats)]
pub struct GetContainerStats {
  pub name: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<DockerContainerStats>)]
pub struct GetContainerStatsList {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct StartContainer {
  pub name: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct StopContainer {
  pub name: String,
  pub signal: Option<TerminationSignal>,
  pub time: Option<i32>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct RemoveContainer {
  pub name: String,
  pub signal: Option<TerminationSignal>,
  pub time: Option<i32>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct RenameContainer {
  pub curr_name: String,
  pub new_name: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneContainers {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct Deploy {
  pub deployment: Deployment,
  /// Override docker token with one sent from core.
  pub docker_token: Option<String>,
  pub stop_signal: Option<TerminationSignal>,
  pub stop_time: Option<i32>,
  /// Propogate any secret replacers from core interpolation.
  pub replacers: Vec<(String, String)>,
}
