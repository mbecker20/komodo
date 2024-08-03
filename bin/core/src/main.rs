#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::Router;
use state::{db_client, jwt_client};
use tower_http::{
  cors::{Any, CorsLayer},
  services::{ServeDir, ServeFile},
};

use crate::config::{core_config, frontend_path};

mod api;
mod auth;
mod cloud;
mod config;
mod db;
mod helpers;
mod listener;
mod monitor;
mod resource;
mod state;
mod ws;

async fn app() -> anyhow::Result<()> {
  dotenv::dotenv().ok();
  let config = core_config();
  logger::init(&config.logging)?;
  info!("monitor core version: v{}", env!("CARGO_PKG_VERSION"));
  info!("config: {:?}", config.sanitized());

  // init db_client to crash on failure
  db_client().await;
  // init jwt client to crash on failure
  jwt_client();

  // Spawn tasks
  monitor::spawn_monitor_loop();
  helpers::prune::spawn_prune_loop();
  helpers::stack::spawn_stack_refresh_loop();
  helpers::sync::spawn_sync_refresh_loop();
  resource::spawn_build_state_refresh_loop();
  resource::spawn_repo_state_refresh_loop();
  resource::spawn_procedure_state_refresh_loop();
  resource::spawn_resource_sync_state_refresh_loop();

  // Setup static frontend services
  let frontend_path = frontend_path();
  let frontend_index =
    ServeFile::new(format!("{frontend_path}/index.html"));
  let serve_dir = ServeDir::new(frontend_path)
    .not_found_service(frontend_index.clone());

  let app = Router::new()
    .nest("/auth", api::auth::router())
    .nest("/user", api::user::router())
    .nest("/read", api::read::router())
    .nest("/write", api::write::router())
    .nest("/execute", api::execute::router())
    .nest("/listener", listener::router())
    .nest("/ws", ws::router())
    .nest_service("/", serve_dir)
    .fallback_service(frontend_index)
    .layer(cors()?);

  let socket_addr =
    SocketAddr::from_str(&format!("0.0.0.0:{}", core_config().port))
      .context("failed to parse socket addr")?;

  let listener = tokio::net::TcpListener::bind(&socket_addr)
    .await
    .context("failed to bind to tcp listener")?;

  info!("monitor core listening on {socket_addr}");

  axum::serve(listener, app).await.context("server crashed")?;

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

fn cors() -> anyhow::Result<CorsLayer> {
  let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
  Ok(cors)
}
