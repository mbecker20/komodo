use std::{borrow::Borrow, net::SocketAddr, str::FromStr};

use anyhow::anyhow;
use axum::http::StatusCode;
use rand::{distributions::Alphanumeric, Rng};
use types::Log;

pub fn parse_comma_seperated_list<T: FromStr>(
    comma_sep_list: impl Borrow<str>,
) -> anyhow::Result<Vec<T>> {
    comma_sep_list
        .borrow()
        .split(",")
        .filter(|item| item.len() > 0)
        .map(|item| {
            let item = item
                .parse()
                .map_err(|_| anyhow!("error parsing string {item} into type T"))?;
            Ok::<T, anyhow::Error>(item)
        })
        .collect()
}

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
