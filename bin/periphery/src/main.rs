#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr};

use anyhow::Context;

mod api;
mod compose;
mod config;
mod docker;
mod helpers;
mod router;
mod stats;

struct State;

async fn app() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();
  let config = config::periphery_config();
  logger::init(&config.logging)?;

  info!("Komodo Periphery version: v{}", env!("CARGO_PKG_VERSION"));
  info!("config: {:?}", config.sanitized());

  stats::spawn_system_stats_polling_threads();

  let socket_addr =
    SocketAddr::from_str(&format!("0.0.0.0:{}", config.port))
      .context("failed to parse socket addr")?;

  let listener = tokio::net::TcpListener::bind(&socket_addr)
    .await
    .context("failed to bind tcp listener")?;

  info!("Komodo Periphery started on {}", socket_addr);

  axum::serve(
    listener,
    router::router()
      .into_make_service_with_connect_info::<SocketAddr>(),
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
