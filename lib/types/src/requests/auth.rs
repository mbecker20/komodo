use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetLoginOptionsResponse)]
pub struct GetLoginOptions {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLoginOptionsResponse {
    pub local: bool,
    pub github: bool,
    pub google: bool,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(ExchangeForJwtResponse)]
pub struct ExchangeForJwt {
    pub token: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeForJwtResponse {
    pub jwt: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(LoginWithSecretResponse)]
pub struct LoginWithSecret {
    pub username: String,
    pub secret: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginWithSecretResponse {
    pub jwt: String,
}

//


