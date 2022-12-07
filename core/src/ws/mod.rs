use axum::{routing::get, Router};

pub mod update;

pub fn router() -> Router {
    Router::new().route("/update", get(update::ws_handler))
}
