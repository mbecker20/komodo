use axum::{routing::get, Router};

mod get_accounts;

pub fn router() -> Router {
    Router::new().route(
        "/accounts/:account_type",
        get(get_accounts::get_accounts),
    )
}
