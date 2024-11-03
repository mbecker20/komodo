use std::{
  collections::HashMap,
  sync::{Arc, OnceLock},
};

use anyhow::anyhow;
use cache::TimeoutCache;
use command::run_komodo_command;
use komodo_client::entities::{
  deployment::extract_registry_domain,
  docker::image::{Image, ImageHistoryResponseItem},
  komodo_timestamp,
  update::Log,
};
use periphery_client::api::image::*;
use resolver_api::Resolve;
use tokio::sync::Mutex;

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

/// Wait this long after a pull to allow another pull through
const PULL_TIMEOUT: i64 = 5_000;

fn pull_cache() -> &'static TimeoutCache<String, Log> {
  static PULL_CACHE: OnceLock<TimeoutCache<String, Log>> =
    OnceLock::new();
  PULL_CACHE.get_or_init(|| Default::default())
}

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
    // Acquire the image lock
    let lock = pull_cache().get_lock(name.clone()).await;

    // Lock the image lock, prevents simultaneous pulls by
    // ensuring simultaneous pulls will wait for first to finish
    // and checking cached results.
    let mut locked = lock.lock().await;

    // Early return from cache if lasted pulled with PULL_TIMEOUT
    if locked.last_ts + PULL_TIMEOUT > komodo_timestamp() {
      return locked.clone_res();
    }

    let res = async {
      docker_login(
        &extract_registry_domain(&name)?,
        account.as_deref().unwrap_or_default(),
        token.as_deref(),
      )
      .await?;
      anyhow::Ok(
        run_komodo_command(
          "docker pull",
          None,
          format!("docker pull {name}"),
          false,
        )
        .await,
      )
    }
    .await;

    // Set the cache with results. Any other calls waiting on the lock will
    // then immediately also use this same result.
    locked.set(&res, komodo_timestamp());

    res
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
