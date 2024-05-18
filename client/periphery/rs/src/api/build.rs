use monitor_client::entities::{
  server::docker_image::ImageSummary, update::Log,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(BuildResponse)]
pub struct Build {
  pub build: monitor_client::entities::build::Build,
  /// Override docker token with one sent from core.
  pub docker_token: Option<String>,
  /// Propogate any secret replacers from core interpolation.
  pub replacers: Vec<(String, String)>,
}

pub type BuildResponse = Vec<Log>;

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetImageListResponse)]
pub struct GetImageList {}

pub type GetImageListResponse = Vec<ImageSummary>;

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(PruneImagesResponse)]
pub struct PruneImages {}

pub type PruneImagesResponse = Log;

//
