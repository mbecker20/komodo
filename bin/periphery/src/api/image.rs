use command::run_komodo_command;
use komodo_client::entities::{
  deployment::extract_registry_domain,
  docker::image::{Image, ImageHistoryResponseItem},
  update::Log,
};
use periphery_client::api::image::*;
use resolver_api::Resolve;

use crate::{
  docker::{docker_client, docker_login},
  State,
};

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

impl Resolve<PullImage> for State {
  #[instrument(name = "PullImage", skip(self))]
  async fn resolve(
    &self,
    PullImage {
      name,
      account,
      token,
    }: PullImage,
    _: (),
  ) -> anyhow::Result<Log> {
    docker_login(
      &extract_registry_domain(&name)?,
      account.as_deref().unwrap_or_default(),
      token.as_deref(),
    )
    .await?;
    Ok(
      run_komodo_command(
        "docker pull",
        None,
        format!("docker pull {name}"),
        false,
      )
      .await,
    )
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
    Ok(run_komodo_command("delete image", None, command, false).await)
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
    Ok(run_komodo_command("prune images", None, command, false).await)
  }
}
