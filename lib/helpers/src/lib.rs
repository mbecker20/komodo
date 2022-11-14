use std::{fs::File, io::Read, net::SocketAddr, str::FromStr};

use anyhow::Context;
use async_timing_util::unix_timestamp_ms;
use axum::http::StatusCode;
use run_command::{async_run_command, CommandOutput};
use serde::de::DeserializeOwned;
use types::Log;

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

pub fn get_socket_addr(port: u16) -> SocketAddr {
    SocketAddr::from_str(&format!("0.0.0.0:{}", port)).expect("failed to parse socket addr")
}

pub fn to_monitor_name(name: &str) -> String {
    name.to_lowercase().replace(" ", "_")
}

pub async fn run_monitor_command(stage: &str, command: String) -> Log {
    let start_ts = unix_timestamp_ms() as i64;
    let output = async_run_command(&command).await;
    output_into_log(stage, command, start_ts, output)
}

pub fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal Error: {err:#?}"),
    )
}
