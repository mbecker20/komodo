use anyhow::Context;
use formatting::format_serror;
use run_command::async_run_command;
use tokio::fs;

use crate::{auth::random_string, config::core_config};

pub async fn get_config_json(
  compose_contents: &str,
) -> (Option<String>, Option<String>) {
  match get_config_json_inner(compose_contents).await {
    Ok(res) => (Some(res), None),
    Err(e) => (
      None,
      Some(format_serror(
        &e.context("failed to get config json").into(),
      )),
    ),
  }
}

async fn get_config_json_inner(
  compose_contents: &str,
) -> anyhow::Result<String> {
  // create a new folder to prevent collisions
  let dir = core_config().stack_directory.join(random_string(10));

  fs::create_dir_all(&dir)
    .await
    .context("failed to create compose file directory")?;
  let file = dir.join("compose.yaml");

  fs::write(&file, compose_contents).await.with_context(|| {
    format!("failed to write compose contents to file file: {file:?}")
  })?;

  let res = async_run_command(&format!(
    "cd {} && docker compose config --format json",
    dir.display()
  ))
  .await;

  // Don't fail the function call here, just log on this maintenance related information.
  fs::remove_dir_all(&dir)
    .await
    .with_context(|| {
      format!("failed to clean up compose directory: {dir:?}")
    })
    .inspect_err(|e| error!("{e:#}"))
    .ok();

  if res.success() {
    Ok(res.stdout)
  } else {
    Err(anyhow::Error::msg(res.stderr))
  }
}
