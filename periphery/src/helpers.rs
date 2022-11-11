use std::{net::SocketAddr, str::FromStr};

#[macro_export]
macro_rules! response {
    ($x:expr) => {
        Ok::<_, (axum::http::StatusCode, String)>($x)
    };
}

use axum::http::StatusCode;

pub fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal Error: {err:#?}"),
    )
}
