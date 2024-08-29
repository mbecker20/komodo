use command::run_monitor_command;
use monitor_client::entities::{
  docker::image::{Image, ImageHistoryResponseItem, ImageListItem},
  update::Log,
};
use periphery_client::api::image::*;
use resolver_api::Resolve;

use crate::{docker::docker_client, State};

//

impl Resolve<GetImageList> for State {
  #[instrument(name = "GetImageList", level = "debug", skip(self))]
  async fn resolve(
    &self,
    _: GetImageList,
    _: (),
  ) -> anyhow::Result<Vec<ImageListItem>> {
    docker_client().list_images().await
  }
}

//

impl Resolve<InspectImage> for State {
  #[instrument(name = "InspectImage", level = "debug", skip(self))]
  async fn resolve(
    &self,
    InspectImage { name }: InspectImage,
    _: (),
  ) -> anyhow::Result<Image> {
    docker_client().inspect_image(&name).await
  }
}

//

impl Resolve<ImageHistory> for State {
  #[instrument(name = "ImageHistory", level = "debug", skip(self))]
  async fn resolve(
    &self,
    ImageHistory { name }: ImageHistory,
    _: (),
  ) -> anyhow::Result<Vec<ImageHistoryResponseItem>> {
    docker_client().image_history(&name).await
  }
}

//

impl Resolve<DeleteImage> for State {
  #[instrument(name = "DeleteImage", skip(self))]
  async fn resolve(
    &self,
    DeleteImage { name }: DeleteImage,
    _: (),
  ) -> anyhow::Result<Log> {
    let command = format!("docker image rm {name}");
    Ok(run_monitor_command("delete image", command).await)
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
    let command = String::from("docker image prune -a -f");
    Ok(run_monitor_command("prune images", command).await)
  }
}
