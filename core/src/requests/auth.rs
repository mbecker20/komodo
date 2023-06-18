use async_trait::async_trait;
use monitor_types::requests::auth::{
    CreateLocalUser, ExchangeForJwt, ExchangeForJwtResponse, GetLoginOptions, LoginLocalUser,
    LoginWithSecret,
};
use resolver_api::{derive::Resolver, Resolve, ResolveToString};
use serde::{Deserialize, Serialize};

use crate::state::State;

#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum AuthRequest {
    #[to_string_resolver]
    GetLoginOptions(GetLoginOptions),
    CreateLocalUser(CreateLocalUser),
    LoginLocalUser(LoginLocalUser),
    LoginWithSecret(LoginWithSecret),
    ExchangeForJwt(ExchangeForJwt),
}

#[async_trait]
impl ResolveToString<GetLoginOptions> for State {
    async fn resolve_to_string(&self, _: GetLoginOptions, _: ()) -> anyhow::Result<String> {
        Ok(self.login_options_response.clone())
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
