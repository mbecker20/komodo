use axum::http::StatusCode;

pub fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal Error: {err:#?}"),
    )
}
