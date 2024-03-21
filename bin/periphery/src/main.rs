#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr, time::Instant};

use anyhow::Context;
use axum::{middleware, routing::post, Json, Router};

use axum_extra::{headers::ContentType, TypedHeader};
use resolver_api::Resolver;
use serror::AppResult;
use termination_signal::tokio::immediate_term_handle;
use uuid::Uuid;

mod api;
mod config;
mod guard;
mod helpers;
mod system_stats;

struct State;

async fn app() -> anyhow::Result<()> {
  dotenv::dotenv().ok();
  let config = config::periphery_config();
  logger::init(config.log_level);

  info!("version: v{}", env!("CARGO_PKG_VERSION"));

  system_stats::spawn_system_stats_polling_threads();

  let socket_addr =
    SocketAddr::from_str(&format!("0.0.0.0:{}", config.port))
      .context("failed to parse socket addr")?;

  let app = Router::new()
    .route(
      "/",
      post(|Json(request): Json<api::PeripheryRequest>| async move {
        let timer = Instant::now();
        let req_id = Uuid::new_v4();
        info!("request {req_id} | {request:?}");
        let res = tokio::spawn(async move {
          let res = State.resolve_request(request, ()).await;
          if let Err(e) = &res {
            debug!("request {req_id} ERROR: {e:#?}");
          }
          let elapsed = timer.elapsed();
          info!("request {req_id} | resolve time: {elapsed:?}");
          res
        })
        .await;
        if let Err(e) = &res {
          debug!("request {req_id} SPAWN ERROR: {e:#?}");
        }
        let res = res??;
        debug!("request {req_id} RESPONSE: {res}");
        AppResult::Ok((TypedHeader(ContentType::json()), res))
      }),
    )
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
  let term_signal = immediate_term_handle()?;

  let app = tokio::spawn(app());

  tokio::select! {
      res = app => return res?,
      _ = term_signal => {},
  }

  Ok(())
}
