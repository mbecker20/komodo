use std::{net::SocketAddr, str::FromStr};

use axum::http::StatusCode;
use types::CoreConfig;

pub fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal Error: {err:#?}"),
    )
}

pub fn get_socket_addr(config: &CoreConfig) -> SocketAddr {
    SocketAddr::from_str(&format!("0.0.0.0:{}", config.port)).expect("failed to parse socket addr")
}
