use monitor_client::entities::{update::Log, CloneArgs, SystemCommand};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Log>)]
pub struct CloneRepo {
  pub args: CloneArgs,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Log>)]
pub struct PullRepo {
  pub name: String,
  pub branch: Option<String>,
  pub on_pull: Option<SystemCommand>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct DeleteRepo {
  pub name: String,
}