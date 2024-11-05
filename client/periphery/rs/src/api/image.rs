use komodo_client::entities::{
  docker::image::{Image, ImageHistoryResponseItem},
  update::Log,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Image)]
pub struct InspectImage {
  pub name: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<ImageHistoryResponseItem>)]
pub struct ImageHistory {
  pub name: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct PullImage {
  /// The name of the image.
  pub name: String,
  /// Optional account to use to pull the image
  pub account: Option<String>,
  /// Override registry token for account with one sent from core.
  pub token: Option<String>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct DeleteImage {
  /// Id or name
  pub name: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneImages {}
