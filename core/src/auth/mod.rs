use std::time::{Duration, Instant};

use axum::{headers::ContentType, routing::post, Json, Router, TypedHeader};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::StatusCode;
use resolver_api::Resolver;
use uuid::Uuid;

mod github;
mod google;
mod jwt;
mod secret;

use crate::{requests::auth::AuthRequest, state::StateExtension};

pub use self::jwt::JwtClient;
pub use github::client::GithubOauthClient;
pub use google::client::GoogleOauthClient;

pub fn router() -> Router {
    Router::new().route(
        "/",
        post(
            |state: StateExtension, Json(request): Json<AuthRequest>| async move {
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
}

fn random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn random_duration(min_ms: u64, max_ms: u64) -> Duration {
    Duration::from_millis(thread_rng().gen_range(min_ms..max_ms))
}
