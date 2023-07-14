use std::{sync::Arc, time::Instant};

use axum::{
    body::Body, headers::ContentType, http::Request, middleware::Next, response::Response,
    routing::post, Json, Router, TypedHeader,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::StatusCode;
use resolver_api::Resolver;
use uuid::Uuid;

mod github;
mod google;
mod jwt;
mod local;
mod secret;

use crate::{
    requests::auth::AuthRequest,
    state::{State, StateExtension},
};

pub use self::jwt::{InnerRequestUser, JwtClient, RequestUser, RequestUserExtension};
pub use github::client::GithubOauthClient;
pub use google::client::GoogleOauthClient;

pub async fn auth_request(
    mut req: Request<Body>,
    next: Next<Body>,
) -> Result<Response, (StatusCode, String)> {
    let state = req.extensions().get::<Arc<State>>().ok_or((
        StatusCode::UNAUTHORIZED,
        "failed to get jwt client extension".to_string(),
    ))?;
    let user = state
        .authenticate_check_enabled(&req)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("{e:#?}")))?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub fn router(state: &State) -> Router {
    let mut router = Router::new().route(
        "/",
        post(
            |state: StateExtension, Json(request): Json<AuthRequest>| async move {
                let timer = Instant::now();
                let req_id = Uuid::new_v4();
                info!("/auth request {req_id} | METHOD: {}", request.req_type());
                let res = state
                    .resolve_request(request, ())
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")));
                if let Err(e) = &res {
                    info!("/auth request {req_id} | ERROR: {e:?}");
                }
                let res = res?;
                let elapsed = timer.elapsed();
                info!("/auth request {req_id} | resolve time: {elapsed:?}");
                debug!("/auth request {req_id} | RESPONSE: {res}");
                Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
            },
        ),
    );

    if state.github_auth.is_some() {
        router = router.nest("/github", github::router())
    }

    if state.google_auth.is_some() {
        router = router.nest("/google", google::router())
    }

    router
}

pub fn random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

// fn random_duration(min_ms: u64, max_ms: u64) -> Duration {
//     Duration::from_millis(thread_rng().gen_range(min_ms..max_ms))
// }
