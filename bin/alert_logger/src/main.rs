#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::{routing::post, Json, Router};
use monitor_client::entities::alert::Alert;
use serde::Deserialize;
use termination_signal::tokio::immediate_term_handle;

#[derive(Deserialize)]
struct Env {
  #[serde(default = "default_port")]
  port: u16,
}

fn default_port() -> u16 {
  7000
}

async fn app() -> anyhow::Result<()> {
  dotenv::dotenv().ok();
  logger::init(tracing::Level::INFO);

  let Env { port } =
    envy::from_env().context("failed to parse env")?;

  let socket_addr = SocketAddr::from_str(&format!("0.0.0.0:{port}"))
    .context("invalid socket addr")?;

  info!("v {} | {socket_addr}", env!("CARGO_PKG_VERSION"));

  let app = Router::new().route(
    "/",
    post(|Json(alert): Json<Alert>| async move {
      info!("{alert:#?}");
    }),
  );

  let listener = tokio::net::TcpListener::bind(socket_addr)
    .await
    .context("failed to bind tcp listener")?;
  
  axum::serve(listener, app).await.context("server crashed")
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
