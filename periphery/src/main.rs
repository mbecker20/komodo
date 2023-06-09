#[macro_use]
extern crate log;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    headers::ContentType, http::StatusCode, middleware, routing::post, Extension, Json, Router,
    TypedHeader,
};
use monitor_types::api::periphery::PeripheryRequest;
use state::State;
use termination_signal::tokio::immediate_term_handle;
use uuid::Uuid;

mod api;
mod config;
mod guard;
mod helpers;
mod state;

async fn app() -> anyhow::Result<()> {
    let state = State::load().await?;

    let socket_addr = state.socket_addr()?;

    let app = Router::new()
        .route(
            "/",
            post(
                |state: Extension<Arc<State>>, Json(request): Json<PeripheryRequest>| async move {
                    let req_id = Uuid::new_v4();
                    info!("request {req_id}: {:?}", request);
                    let res = state
                        .handle_request(request)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")));
                    if let Err(e) = &res {
                        debug!("request {req_id} ERROR: {e:?}");
                    }
                    let res = res?;
                    debug!("request {req_id} RESPONSE: {res}");
                    Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
                },
            ),
        )
        .layer(middleware::from_fn(guard::guard_request_by_ip))
        .layer(middleware::from_fn(guard::guard_request_by_passkey))
        .layer(Extension(state));

    info!("starting server on {}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
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
