use anyhow::anyhow;
use async_timing_util::unix_timestamp_ms;
use axum::{http::StatusCode, TypedHeader, headers::ContentType};
use monitor_types::entities::update::Log;
use run_command::{async_run_command, CommandOutput};
use serror::serialize_error_pretty;

use crate::state::State;

pub mod docker;
pub mod git;
pub mod stats;

impl State {
    pub fn get_github_token(
        &self,
        github_account: &Option<String>,
    ) -> anyhow::Result<Option<String>> {
        match github_account {
            Some(account) => match self.config.github_accounts.get(account) {
                Some(token) => Ok(Some(token.to_owned())),
                None => Err(anyhow!(
                    "did not find token in config for github account {account} "
                )),
            },
            None => Ok(None),
        }
    }

    pub fn get_docker_token(
        &self,
        docker_account: &Option<String>,
    ) -> anyhow::Result<Option<String>> {
        match docker_account {
            Some(account) => match self.config.docker_accounts.get(account) {
                Some(token) => Ok(Some(token.to_owned())),
                None => Err(anyhow!(
                    "did not find token in config for docker account {account} "
                )),
            },
            None => Ok(None),
        }
    }
}

pub async fn run_monitor_command(stage: &str, command: String) -> Log {
    let start_ts = unix_timestamp_ms() as i64;
    let output = async_run_command(&command).await;
    output_into_log(stage, command, start_ts, output)
}

pub fn output_into_log(stage: &str, command: String, start_ts: i64, output: CommandOutput) -> Log {
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

pub fn into_response_error(e: anyhow::Error) -> (StatusCode, TypedHeader<ContentType>, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        TypedHeader(ContentType::json()),
        serialize_error_pretty(e),
    )
}
