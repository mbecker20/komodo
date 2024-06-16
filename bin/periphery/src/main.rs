#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::{middleware, routing::post, Router};

mod api;
mod config;
mod docker;
mod guard;
mod handler;
mod helpers;
mod stats;

struct State;

async fn app() -> anyhow::Result<()> {
  dotenv::dotenv().ok();
  let config = config::periphery_config();
  logger::init(&config.logging)?;

  info!("version: v{}", env!("CARGO_PKG_VERSION"));

  stats::spawn_system_stats_polling_threads();

  let socket_addr =
    SocketAddr::from_str(&format!("0.0.0.0:{}", config.port))
      .context("failed to parse socket addr")?;

  let app = Router::new()
    .route("/", post(handler::handler))
    .layer(middleware::from_fn(guard::guard_request_by_ip))
    .layer(middleware::from_fn(guard::guard_request_by_passkey));

  info!("starting server on {}", socket_addr);

  let listener = tokio::net::TcpListener::bind(&socket_addr)
    .await
    .context("failed to bind tcp listener")?;

  axum::serve(
    listener,
    app.into_make_service_with_connect_info::<SocketAddr>(),
  )
  .await?;

  Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut term_signal = tokio::signal::unix::signal(
    tokio::signal::unix::SignalKind::terminate(),
  )?;

  let app = tokio::spawn(app());

  tokio::select! {
    res = app => return res?,
    _ = term_signal.recv() => {},
  }

  Ok(())
}
