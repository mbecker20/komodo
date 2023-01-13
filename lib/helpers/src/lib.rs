use std::{fs::File, io::Read, net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::http::StatusCode;
use rand::{distributions::Alphanumeric, Rng};
use run_command::{async_run_command, CommandOutput};
use serde::de::DeserializeOwned;
use types::{monitor_timestamp, Log};

pub mod docker;
pub mod git;

pub fn parse_config_file<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
    let mut file = File::open(&path).expect(&format!("failed to find config at {path}"));
    let config = if path.ends_with("toml") {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context(format!("failed to read toml at {path}"))?;
        toml::from_str(&contents).context(format!("failed to parse toml at {path}"))?
    } else if path.ends_with("json") {
        serde_json::from_reader(file).context(format!("failed to parse json at {path}"))?
    } else {
        panic!("unsupported config file type: {}", path)
    };
    Ok(config)
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

pub fn get_socket_addr(port: u16) -> SocketAddr {
    SocketAddr::from_str(&format!("0.0.0.0:{}", port)).expect("failed to parse socket addr")
}

pub fn to_monitor_name(name: &str) -> String {
    name.to_lowercase().replace(" ", "_")
}

pub async fn run_monitor_command(stage: &str, command: String) -> Log {
    let start_ts = monitor_timestamp();
    let output = async_run_command(&command).await;
    output_into_log(stage, command, start_ts, output)
}

pub fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal Error: {err:#?}"),
    )
}

pub fn generate_secret(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn all_logs_success(logs: &Vec<Log>) -> bool {
    for log in logs {
        if !log.success {
            return false;
        }
    }
    true
}
