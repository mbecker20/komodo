use resolver_api::derive::Resolver;
use serde::{Deserialize, Serialize};

use crate::{auth::RequestUser, state::State};

mod secret;

#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(RequestUser)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum ApiRequest {}
