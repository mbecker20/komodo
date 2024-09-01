use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::server_template::{
  ServerTemplate, ServerTemplateListItem, ServerTemplateQuery,
};

use super::KomodoReadRequest;

//

/// Get a specific server template by id or name. Response: [ServerTemplate].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(GetServerTemplateResponse)]
pub struct GetServerTemplate {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server_template: String,
}

#[typeshare]
pub type GetServerTemplateResponse = ServerTemplate;

//

/// List server templates matching structured query. Response: [ListServerTemplatesResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(ListServerTemplatesResponse)]
pub struct ListServerTemplates {
  #[serde(default)]
  pub query: ServerTemplateQuery,
}

#[typeshare]
pub type ListServerTemplatesResponse = Vec<ServerTemplateListItem>;

//

/// List server templates matching structured query. Response: [ListFullServerTemplatesResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(ListFullServerTemplatesResponse)]
pub struct ListFullServerTemplates {
  #[serde(default)]
  pub query: ServerTemplateQuery,
}

#[typeshare]
pub type ListFullServerTemplatesResponse = Vec<ServerTemplate>;

//

/// Gets a summary of data relating to all server templates.
/// Response: [GetServerTemplatesSummaryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(GetServerTemplatesSummaryResponse)]
pub struct GetServerTemplatesSummary {}

/// Response for [GetServerTemplatesSummary].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServerTemplatesSummaryResponse {
  /// The total number of server templates.
  pub total: u32,
}
