#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::Router;
use axum_server::tls_openssl::OpenSSLConfig;
use tower_http::{
  cors::{Any, CorsLayer},
  services::{ServeDir, ServeFile},
};

use crate::config::core_config;

mod alert;
mod api;
mod auth;
mod cloud;
mod config;
mod db;
mod helpers;
mod listener;
mod monitor;
mod resource;
mod stack;
mod state;
mod sync;
mod ws;

async fn app() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();
  let config = core_config();
  logger::init(&config.logging)?;

  info!("Komodo Core version: v{}", env!("CARGO_PKG_VERSION"));
  info!("{:?}", config.sanitized());

  // includes init db_client check to crash on db init failure
  helpers::startup_cleanup().await;
  // Maybe initialize default server in All In One deployment.
  helpers::ensure_first_server().await;
  // init jwt client to crash on failure
  state::jwt_client();

  // Spawn tasks
  monitor::spawn_monitor_loop();
  stack::spawn_stack_refresh_loop();
  sync::spawn_sync_refresh_loop();
  resource::spawn_build_state_refresh_loop();
  resource::spawn_repo_state_refresh_loop();
  resource::spawn_procedure_state_refresh_loop();
  resource::spawn_resource_sync_state_refresh_loop();
  helpers::build::spawn_build_refresh_loop();
  helpers::repo::spawn_repo_refresh_loop();
  helpers::prune::spawn_prune_loop();

  // Setup static frontend services
  let frontend_path = &config.frontend_path;
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
    .layer(cors()?)
    .into_make_service();

  let socket_addr =
    SocketAddr::from_str(&format!("0.0.0.0:{}", core_config().port))
      .context("failed to parse socket addr")?;

  if config.ssl_enabled {
    info!("🔒 Core SSL Enabled");
    info!("Komodo Core starting on https://{socket_addr}");
    let ssl_config = OpenSSLConfig::from_pem_file(
      &config.ssl_cert_file,
      &config.ssl_key_file,
    )
    .context("Failed to parse ssl ")?;
    axum_server::bind_openssl(socket_addr, ssl_config)
      .serve(app)
      .await?
  } else {
    info!("🔓 Core SSL Disabled");
    info!("Komodo Core starting on http://{socket_addr}");
    axum_server::bind(socket_addr).serve(app).await?
  }

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
