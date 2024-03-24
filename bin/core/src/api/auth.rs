use std::sync::OnceLock;

use async_trait::async_trait;
use monitor_client::api::auth::*;
use resolver_api::{derive::Resolver, Resolve};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  auth::jwt::jwt_client, config::core_config, state::State,
};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum AuthRequest {
  GetLoginOptions(GetLoginOptions),
  CreateLocalUser(CreateLocalUser),
  LoginLocalUser(LoginLocalUser),
  ExchangeForJwt(ExchangeForJwt),
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
impl Resolve<GetLoginOptions> for State {
  async fn resolve(
    &self,
    _: GetLoginOptions,
    _: (),
  ) -> anyhow::Result<GetLoginOptionsResponse> {
    Ok(*login_options_reponse())
  }
}

#[async_trait]
impl Resolve<ExchangeForJwt> for State {
  async fn resolve(
    &self,
    ExchangeForJwt { token }: ExchangeForJwt,
    _: (),
  ) -> anyhow::Result<ExchangeForJwtResponse> {
    let jwt = jwt_client().redeem_exchange_token(&token).await?;
    let res = ExchangeForJwtResponse { jwt };
    Ok(res)
  }
}
