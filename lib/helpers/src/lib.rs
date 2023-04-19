use std::{net::SocketAddr, str::FromStr};

use axum::http::StatusCode;
use rand::{distributions::Alphanumeric, Rng};
use types::Log;

pub fn get_socket_addr(port: u16) -> SocketAddr {
    SocketAddr::from_str(&format!("0.0.0.0:{}", port)).expect("failed to parse socket addr")
}

pub fn to_monitor_name(name: &str) -> String {
    name.to_lowercase().replace(" ", "_")
}

pub fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:#?}"))
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
