use std::{sync::OnceLock, time::Instant};

use async_trait::async_trait;
use axum::{http::HeaderMap, routing::post, Json, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use monitor_client::{api::auth::*, entities::user::User};
use resolver_api::{derive::Resolver, Resolve, Resolver};
use serde::{Deserialize, Serialize};
use serror::AppResult;
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
  auth::{
    get_user_id_from_headers,
    github::{self, client::github_oauth_client},
    google::{self, client::google_oauth_client},
    jwt::jwt_client,
  },
  config::core_config,
  helpers::get_user,
  state::State,
};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(HeaderMap)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum AuthRequest {
  GetLoginOptions(GetLoginOptions),
  CreateLocalUser(CreateLocalUser),
  LoginLocalUser(LoginLocalUser),
  ExchangeForJwt(ExchangeForJwt),
  GetUser(GetUser),
}

pub fn router() -> Router {
  let mut router = Router::new().route(
    "/",
    post(|headers: HeaderMap, Json(request): Json<AuthRequest>| async move {
      let timer = Instant::now();
      let req_id = Uuid::new_v4();
      info!(
        "/auth request {req_id} | METHOD: {}",
        request.req_type()
      );
      let res = State.resolve_request(request, headers).await;
      if let Err(resolver_api::Error::Serialization(e)) = &res {
        info!("/auth request {req_id} | serialization error: {e:?}");
      }
      if let Err(resolver_api::Error::Inner(e)) = &res {
        info!("/auth request {req_id} | error: {e:#}");
      }
      let elapsed = timer.elapsed();
      info!("/auth request {req_id} | resolve time: {elapsed:?}");
      AppResult::Ok((TypedHeader(ContentType::json()), res?))
    }),
  );

  if github_oauth_client().is_some() {
    router = router.nest("/github", github::router())
  }

  if google_oauth_client().is_some() {
    router = router.nest("/google", google::router())
  }

  router
}

fn login_options_reponse() -> &'static GetLoginOptionsResponse {
  static GET_LOGIN_OPTIONS_RESPONSE: OnceLock<
    GetLoginOptionsResponse,
  > = OnceLock::new();
  GET_LOGIN_OPTIONS_RESPONSE.get_or_init(|| {
    let config = core_config();
    GetLoginOptionsResponse {
      local: config.local_auth,
      github: config.github_oauth.enabled
        && !config.github_oauth.id.is_empty()
        && !config.github_oauth.secret.is_empty(),
      google: config.google_oauth.enabled
        && !config.google_oauth.id.is_empty()
        && !config.google_oauth.secret.is_empty(),
    }
  })
}

#[async_trait]
impl Resolve<GetLoginOptions, HeaderMap> for State {
  async fn resolve(
    &self,
    _: GetLoginOptions,
    _: HeaderMap,
  ) -> anyhow::Result<GetLoginOptionsResponse> {
    Ok(*login_options_reponse())
  }
}

#[async_trait]
impl Resolve<ExchangeForJwt, HeaderMap> for State {
  async fn resolve(
    &self,
    ExchangeForJwt { token }: ExchangeForJwt,
    _: HeaderMap,
  ) -> anyhow::Result<ExchangeForJwtResponse> {
    let jwt = jwt_client().redeem_exchange_token(&token).await?;
    let res = ExchangeForJwtResponse { jwt };
    Ok(res)
  }
}

#[async_trait]
impl Resolve<GetUser, HeaderMap> for State {
  async fn resolve(
    &self,
    GetUser {}: GetUser,
    headers: HeaderMap,
  ) -> anyhow::Result<User> {
    let user_id = get_user_id_from_headers(&headers).await?;
    get_user(&user_id).await
  }
}
