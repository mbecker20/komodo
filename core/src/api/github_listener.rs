use axum::{extract::Path, routing::post, Router, http::{Request, HeaderMap}, body::Body};
use helpers::handle_anyhow_error;
use mungos::Deserialize;

use crate::state::{State, StateExtension};

#[derive(Deserialize)]
struct Id {
    id: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/build/:id",
            post(|state: StateExtension, Path(Id { id }), headers: HeaderMap, body: String| async move {
				state.handle_build_webhook(&id, headers, body).await.map_err(handle_anyhow_error)
			}),
        )
        .route(
            "/deployment/:id",
            post(|state: StateExtension, Path(Id { id }), headers: HeaderMap, body: String| async move {
				state.handle_deployment_webhook(&id, headers, body).await.map_err(handle_anyhow_error)
			}),
        )
        .route(
            "/procedure/:id",
            post(|state: StateExtension, Path(Id { id }), headers: HeaderMap, body: String| async move {
				state.handle_procedure_webhook(&id, headers, body).await.map_err(handle_anyhow_error)
			}),
        )
}

impl State {
    async fn handle_build_webhook(&self, id: &str, headers: HeaderMap, body: String) -> anyhow::Result<()> {

		Ok(())
	}

	async fn handle_deployment_webhook(&self, id: &str, headers: HeaderMap, body: String) -> anyhow::Result<()> {

		Ok(())
	}

	async fn handle_procedure_webhook(&self, id: &str, headers: HeaderMap, body: String) -> anyhow::Result<()> {

		Ok(())
	}

	fn verify_gh_signature(&self, headers: HeaderMap, body: &str) -> bool {
		let signature = headers.get("x-hub-signature-256");
		if signature.is_none() {
			return false
		}
		let signature = signature.unwrap().to_str();
		if signature.is_err() {
			return false
		}
		let signature = signature.unwrap();
		todo!()
    }
}

#[derive(Deserialize)]
struct GhWebhookBody {}
