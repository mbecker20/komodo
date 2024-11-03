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
#[response(PullImageResponse)]
pub struct PullImage {
  /// The name of the image.
  pub name: String,
  /// Optional account to use to pull the image
  pub account: Option<String>,
  /// Override registry token for account with one sent from core.
  pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullImageResponse {
  /// The latest image id pulled to the system matching name,
  /// whether or not it was pulled by this call.
  pub image_id: Option<String>,
  /// The log associated with the pull.
  pub log: Log,
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
