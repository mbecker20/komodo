#[macro_use]
extern crate log;

use std::sync::Arc;

use axum::{
    extract::State, headers::ContentType, http::StatusCode, routing::post, Json, Router,
    TypedHeader,
};
use monitor_types::periphery_api::PeripheryRequest;
use state::AppState;
use termination_signal::tokio::immediate_term_handle;
use uuid::Uuid;

mod api;
mod config;
mod state;

async fn app() -> anyhow::Result<()> {
    let state = AppState::load().await?;

    let socket_addr = state.socket_addr()?;

    let app = Router::new()
        .route(
            "/api",
            post(
                |state: State<Arc<AppState>>, Json(request): Json<PeripheryRequest>| async move {
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
        .with_state(state);

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
