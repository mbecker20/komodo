use anyhow::anyhow;
use async_timing_util::unix_timestamp_ms;
use monitor_client::entities::update::Log;
use run_command::{async_run_command, CommandOutput};

use crate::config::periphery_config;

pub mod docker;
pub mod git;

pub fn get_github_token(
  github_account: &Option<String>,
) -> anyhow::Result<Option<String>> {
  match github_account {
    Some(account) => {
      match periphery_config().github_accounts.get(account) {
        Some(token) => Ok(Some(token.to_owned())),
        None => Err(anyhow!(
          "did not find token in config for github account {account}"
        )),
      }
    }
    None => Ok(None),
  }
}

pub fn get_docker_token(
  docker_account: &Option<String>,
) -> anyhow::Result<Option<String>> {
  match docker_account {
    Some(account) => {
      match periphery_config().docker_accounts.get(account) {
        Some(token) => Ok(Some(token.to_owned())),
        None => Err(anyhow!(
        "did not find token in config for docker account {account}"
      )),
      }
    }
    None => Ok(None),
  }
}

pub async fn run_monitor_command(
  stage: &str,
  command: String,
) -> Log {
  let start_ts = unix_timestamp_ms() as i64;
  let output = async_run_command(&command).await;
  output_into_log(stage, command, start_ts, output)
}

pub fn output_into_log(
  stage: &str,
  command: String,
  start_ts: i64,
  output: CommandOutput,
) -> Log {
  let success = output.success();
  Log {
    stage: stage.to_string(),
    stdout: output.stdout,
    stderr: output.stderr,
    command,
    success,
    start_ts,
    end_ts: unix_timestamp_ms() as i64,
  }
}
