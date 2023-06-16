use resolver_api::derive::Resolver;
use serde::{Deserialize, Serialize};

use crate::state::State;

#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[serde(tag = "type", content = "params")]
#[resolver_target(State)]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum CoreRequest {}
