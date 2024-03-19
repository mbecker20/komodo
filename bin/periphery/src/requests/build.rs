use async_trait::async_trait;
use monitor_client::entities::{
  optional_string, server::docker_image::ImageSummary, update::Log,
};
use resolver_api::{derive::Request, Resolve};
use serde::{Deserialize, Serialize};

use crate::{
  helpers::{
    docker::{self, client::docker_client},
    get_docker_token,
  },
  State,
};

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Log>)]
pub struct Build {
  pub build: monitor_client::entities::build::Build,
}

#[async_trait]
impl Resolve<Build> for State {
  async fn resolve(
    &self,
    Build { build }: Build,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let log = match get_docker_token(&optional_string(
      &build.config.docker_account,
    )) {
      Ok(docker_token) => {
        match docker::build::build(&build, docker_token).await {
          Ok(logs) => logs,
          Err(e) => {
            vec![Log::error("build", format!("{e:#?}"))]
          }
        }
      }
      Err(e) => vec![Log::error("build", format!("{e:#?}"))],
    };
    Ok(log)
  }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<ImageSummary>)]
pub struct GetImageList {}

#[async_trait::async_trait]
impl Resolve<GetImageList> for State {
  async fn resolve(
    &self,
    _: GetImageList,
    _: (),
  ) -> anyhow::Result<Vec<ImageSummary>> {
    docker_client().list_images().await
  }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneImages {}

#[async_trait]
impl Resolve<PruneImages> for State {
  async fn resolve(
    &self,
    _: PruneImages,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::prune_images().await)
  }
}
