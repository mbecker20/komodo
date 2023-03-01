use anyhow::anyhow;
use run_command::{async_run_command, CommandOutput};
use types::{monitor_timestamp, DockerToken, GithubToken, Log, PeripheryConfig};

pub mod docker;
pub mod git;

#[macro_export]
macro_rules! response {
    ($x:expr) => {
        Ok::<_, (axum::http::StatusCode, String)>($x)
    };
}

pub fn get_github_token(
    github_account: &Option<String>,
    config: &PeripheryConfig,
) -> anyhow::Result<Option<GithubToken>> {
    match github_account {
        Some(account) => match config.github_accounts.get(account) {
            Some(token) => Ok(Some(token.to_owned())),
            None => Err(anyhow!(
                "did not find token in config for github account {account} "
            )),
        },
        None => Ok(None),
    }
}

pub fn get_docker_token(
    docker_account: &Option<String>,
    config: &PeripheryConfig,
) -> anyhow::Result<Option<DockerToken>> {
    match docker_account {
        Some(account) => match config.docker_accounts.get(account) {
            Some(token) => Ok(Some(token.to_owned())),
            None => Err(anyhow!(
                "did not find token in config for docker account {account} "
            )),
        },
        None => Ok(None),
    }
}

pub async fn run_monitor_command(stage: &str, command: String) -> Log {
    let start_ts = monitor_timestamp();
    let output = async_run_command(&command).await;
    output_into_log(stage, command, start_ts, output)
}

pub fn output_into_log(
    stage: &str,
    command: String,
    start_ts: String,
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
        end_ts: monitor_timestamp(),
    }
}
