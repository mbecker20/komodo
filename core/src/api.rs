use std::time::Instant;

use axum::{headers::ContentType, middleware, routing::post, Extension, Json, Router, TypedHeader};
use reqwest::StatusCode;
use resolver_api::Resolver;
use uuid::Uuid;

use crate::{
    auth::{auth_request, RequestUserExtension},
    requests::api::ApiRequest,
    state::StateExtension,
};

pub fn router() -> Router {
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
