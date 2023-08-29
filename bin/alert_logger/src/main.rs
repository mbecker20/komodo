#[macro_use]
extern crate log;

use std::{net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::{routing::post, Json, Router};
use monitor_types::entities::alert::Alert;
use termination_signal::tokio::immediate_term_handle;

async fn app() -> anyhow::Result<()> {
    logger::init(log::LevelFilter::Info)?;

    let socket_addr = SocketAddr::from_str("0.0.0.0:7000").unwrap();

    info!("v {} | {socket_addr}", env!("CARGO_PKG_VERSION"));

    let app = Router::new().route(
        "/",
        post(|Json(alert): Json<Alert>| async move {
            info!("{alert:#?}");
        }),
    );

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .context("server crashed")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let term_signal = immediate_term_handle()?;

    let app = tokio::spawn(app());

    tokio::select! {
        res = app => return res?,
        _ = term_signal => {},
    }

    Ok(())
}
