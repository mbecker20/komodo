use std::time::Instant;

use axum::{
    headers::ContentType, http::StatusCode, middleware, routing::post, Extension, Json, Router,
    TypedHeader,
};
use monitor_types::requests::write::*;
use resolver_api::{derive::Resolver, Resolve, Resolver};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
    auth::{auth_request, RequestUser, RequestUserExtension},
    state::{State, StateExtension},
};

mod alerter;
mod build;
mod builder;
mod deployment;
mod permissions;
mod repo;
mod secret;
mod server;
mod tag;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(RequestUser)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
enum WriteRequest {
    // ==== SECRET ====
    CreateLoginSecret(CreateLoginSecret),
    DeleteLoginSecret(DeleteLoginSecret),

    // ==== PERMISSIONS ====
    UpdateUserPerimissions(UpdateUserPermissions),
    UpdateUserPermissionsOnTarget(UpdateUserPermissionsOnTarget),

    // ==== SERVER ====
    CreateServer(CreateServer),
    DeleteServer(DeleteServer),
    UpdateServer(UpdateServer),
    RenameServer(RenameServer),

    // ==== DEPLOYMENT ====
    CreateDeployment(CreateDeployment),
    CopyDeployment(CopyDeployment),
    DeleteDeployment(DeleteDeployment),
    UpdateDeployment(UpdateDeployment),
    RenameDeployment(RenameDeployment),

    // ==== BUILD ====
    CreateBuild(CreateBuild),
    CopyBuild(CopyBuild),
    DeleteBuild(DeleteBuild),
    UpdateBuild(UpdateBuild),

    // ==== BUILDER ====
    CreateBuilder(CreateBuilder),
    CopyBuilder(CopyBuilder),
    DeleteBuilder(DeleteBuilder),
    UpdateBuilder(UpdateBuilder),

    // ==== REPO ====
    CreateRepo(CreateRepo),
    CopyRepo(CopyRepo),
    DeleteRepo(DeleteRepo),
    UpdateRepo(UpdateRepo),

    // ==== ALERTER ====
    CreateAlerter(CreateAlerter),
    CopyAlerter(CopyAlerter),
    DeleteAlerter(DeleteAlerter),
    UpdateAlerter(UpdateAlerter),

    // ==== TAG ====
    CreateTag(CreateTag),
    DeleteTag(DeleteTag),
    UpdateTag(UpdateTag),
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            post(
                |state: StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(request): Json<WriteRequest>| async move {
                    let timer = Instant::now();
                    let req_id = Uuid::new_v4();
                    info!(
                        "/write request {req_id} | user: {} ({}) | {request:?}",
                        user.username, user.id
                    );
                    let res = tokio::spawn(async move {
                        state
                            .resolve_request(request, user)
                            .await
                            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")))
                    })
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")));
                    if let Err(e) = &res {
                        info!("/write request {req_id} SPAWN ERROR: {e:#?}");
                    }
                    let res = res?;
                    if let Err(e) = &res {
                        info!("/write request {req_id} ERROR: {e:#?}");
                    }
                    let res = res?;
                    let elapsed = timer.elapsed();
                    info!("/write request {req_id} | resolve time: {elapsed:?}");
                    Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
                },
            ),
        )
        .layer(middleware::from_fn(auth_request))
}
