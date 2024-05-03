use async_trait::async_trait;
use monitor_client::entities::{
  server::docker_image::ImageSummary, update::Log,
};
use periphery_client::api::build::{
  Build, GetImageList, PruneImages,
};
use resolver_api::Resolve;

use crate::{
  helpers::docker::{self, client::docker_client},
  State,
};

#[async_trait]
impl Resolve<Build> for State {
  #[instrument(name = "Build", skip(self))]
  async fn resolve(
    &self,
    Build {
      build,
      docker_token,
    }: Build,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    docker::build::build(&build, docker_token).await
  }
}

//

#[async_trait::async_trait]
impl Resolve<GetImageList> for State {
  #[instrument(name = "GetImageList", level = "debug", skip(self))]
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
  #[instrument(name = "PruneImages", skip(self))]
  async fn resolve(
    &self,
    _: PruneImages,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::build::prune_images().await)
  }
}
