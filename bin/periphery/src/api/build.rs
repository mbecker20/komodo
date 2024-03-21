use async_trait::async_trait;
use monitor_client::entities::{
  optional_string, server::docker_image::ImageSummary, update::Log,
};
use periphery_client::api::build::{
  Build, GetImageList, PruneImages,
};
use resolver_api::Resolve;

use crate::{
  helpers::{
    docker::{self, client::docker_client},
    get_docker_token,
  },
  State,
};

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
