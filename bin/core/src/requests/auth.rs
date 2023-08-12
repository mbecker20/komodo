use async_trait::async_trait;
use monitor_types::requests::auth::{
    CreateLocalUser, ExchangeForJwt, ExchangeForJwtResponse, GetLoginOptions,
    GetLoginOptionsResponse, LoginLocalUser, LoginWithSecret,
};
use resolver_api::{derive::Resolver, Resolve};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::state::State;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum AuthRequest {
    GetLoginOptions(GetLoginOptions),
    CreateLocalUser(CreateLocalUser),
    LoginLocalUser(LoginLocalUser),
    LoginWithSecret(LoginWithSecret),
    ExchangeForJwt(ExchangeForJwt),
}

#[async_trait]
impl Resolve<GetLoginOptions> for State {
    async fn resolve(&self, _: GetLoginOptions, _: ()) -> anyhow::Result<GetLoginOptionsResponse> {
        Ok(GetLoginOptionsResponse {
            local: self.config.local_auth,
            github: self.config.github_oauth.enabled
                && !self.config.github_oauth.id.is_empty()
                && !self.config.github_oauth.secret.is_empty(),
            google: self.config.google_oauth.enabled
                && !self.config.google_oauth.id.is_empty()
                && !self.config.google_oauth.secret.is_empty(),
        })
    }
}

#[async_trait]
impl Resolve<ExchangeForJwt> for State {
    async fn resolve(
        &self,
        ExchangeForJwt { token }: ExchangeForJwt,
        _: (),
    ) -> anyhow::Result<ExchangeForJwtResponse> {
        let jwt = self.jwt.redeem_exchange_token(&token).await?;
        let res = ExchangeForJwtResponse { jwt };
        Ok(res)
    }
}
