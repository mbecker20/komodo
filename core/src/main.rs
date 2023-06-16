#[macro_use]
extern crate log;

use std::{sync::Arc, time::Instant};

use axum::{
    headers::ContentType, http::StatusCode, routing::post, Extension, Json, Router, TypedHeader,
};
use termination_signal::tokio::immediate_term_handle;
use uuid::Uuid;

use crate::requests::CoreRequest;

mod config;
mod requests;
mod state;

async fn app() -> anyhow::Result<()> {
    let state = state::State::load().await?;

    info!("version: v{}", env!("CARGO_PKG_VERSION"));

    let socket_addr = state.socket_addr()?;

    let app = Router::new()
        .route(
            "/api",
            post(
                |state: Extension<Arc<state::State>>, Json(request): Json<CoreRequest>| async move {
                    let timer = Instant::now();
                    let req_id = Uuid::new_v4();
                    info!("request {req_id} | {request:?}");
                    let res = state
                        .resolve_request(request)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")));
                    if let Err(e) = &res {
                        debug!("request {req_id} ERROR: {e:?}");
                    }
                    let res = res?;
                    let elapsed = timer.elapsed();
                    info!("request {req_id} | resolve time: {elapsed:?}");
                    debug!("request {req_id} RESPONSE: {res}");
                    Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
                },
            ),
        )
        .layer(Extension(state));

    info!("starting server on {}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
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
