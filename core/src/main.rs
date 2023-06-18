#[macro_use]
extern crate log;

use std::time::Instant;

use auth::{auth_request, RequestUserExtension};
use axum::{
    headers::ContentType, http::StatusCode, middleware, routing::post, Extension, Json, Router,
    TypedHeader,
};
use resolver_api::Resolver;
use state::StateExtension;
use termination_signal::tokio::immediate_term_handle;
use uuid::Uuid;

use crate::requests::api::ApiRequest;

mod auth;
mod config;
mod db;
mod helpers;
mod requests;
mod state;

async fn app() -> anyhow::Result<()> {
    let state = state::State::load().await?;

    info!("version: v{}", env!("CARGO_PKG_VERSION"));

    let socket_addr = state.socket_addr()?;

    let app = Router::new()
        .nest("/auth", auth::router(&state))
        .nest("/api", api())
        .layer(Extension(state));

    info!("starting server on {}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn api() -> Router {
    Router::new()
        .route(
            "/",
            post(
                |state: StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(request): Json<ApiRequest>| async move {
                    let timer = Instant::now();
                    let req_id = Uuid::new_v4();
                    info!("/auth request {req_id} | {request:?}");
                    let res = state
                        .resolve_request(request, user)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")));
                    if let Err(e) = &res {
                        info!("/auth request {req_id} ERROR: {e:?}");
                    }
                    let res = res?;
                    let elapsed = timer.elapsed();
                    info!("/auth request {req_id} | resolve time: {elapsed:?}");
                    debug!("/auth request {req_id} RESPONSE: {res}");
                    Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
                },
            ),
        )
        .layer(middleware::from_fn(auth_request))
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
