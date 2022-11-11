use axum::{routing::get, Router};

mod accounts;

pub fn router() -> Router {
    Router::new().route("/accounts/:account_type", get(accounts::get_accounts))
}
