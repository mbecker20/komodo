use monitor_client::entities::{
  server::docker_image::ImageSummary, update::Log,
};
use periphery_client::api::build::{
  Build, GetImageList, PruneImages,
};
use resolver_api::Resolve;

use crate::{
  docker::{self, client::docker_client},
  State,
};

impl Resolve<Build> for State {
  #[instrument(name = "Build", skip(self, replacers))]
  async fn resolve(
    &self,
    Build {
      build,
      registry_token,
      replacers,
    }: Build,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    docker::build::build(&build, registry_token, replacers).await
  }
}

//

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
