use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::{
    GetProcedure, GetProcedureActionState,
    GetProcedureActionStateResponse, GetProcedureResponse,
    GetProceduresSummary, GetProceduresSummaryResponse,
    ListProcedures, ListProceduresByIds, ListProceduresByIdsResponse,
    ListProceduresResponse,
  },
  entities::{procedure::Procedure, PermissionLevel},
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{
  auth::RequestUser, helpers::resource::StateResource, state::State,
};

#[async_trait]
impl Resolve<GetProcedure, RequestUser> for State {
  async fn resolve(
    &self,
    GetProcedure { id }: GetProcedure,
    user: RequestUser,
  ) -> anyhow::Result<GetProcedureResponse> {
    self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Read,
      )
      .await
  }
}

#[async_trait]
impl Resolve<ListProcedures, RequestUser> for State {
  async fn resolve(
    &self,
    ListProcedures { query }: ListProcedures,
    user: RequestUser,
  ) -> anyhow::Result<ListProceduresResponse> {
    <State as StateResource<Procedure>>::list_resources_for_user(
      self, query, &user,
    )
    .await
  }
}

#[async_trait]
impl Resolve<ListProceduresByIds, RequestUser> for State {
  async fn resolve(
    &self,
    ListProceduresByIds { ids }: ListProceduresByIds,
    user: RequestUser,
  ) -> anyhow::Result<ListProceduresByIdsResponse> {
    <State as StateResource<Procedure>>::list_resources_for_user(
      self,
      doc! { "_id": { "$in": ids } }.into(),
      &user,
    )
    .await
  }
}

#[async_trait]
impl Resolve<GetProceduresSummary, RequestUser> for State {
  async fn resolve(
    &self,
    GetProceduresSummary {}: GetProceduresSummary,
    user: RequestUser,
  ) -> anyhow::Result<GetProceduresSummaryResponse> {
    let query = if user.is_admin {
      None
    } else {
      let query = doc! {
          format!("permissions.{}", user.id): { "$in": ["read", "execute", "update"] }
      };
      Some(query)
    };
    let total = self
      .db
      .procedures
      .count_documents(query, None)
      .await
      .context("failed to count all procedure documents")?;
    let res = GetProceduresSummaryResponse {
      total: total as u32,
    };
    Ok(res)
  }
}

#[async_trait]
impl Resolve<GetProcedureActionState, RequestUser> for State {
  async fn resolve(
    &self,
    GetProcedureActionState { id }: GetProcedureActionState,
    user: RequestUser,
  ) -> anyhow::Result<GetProcedureActionStateResponse> {
    let _: Procedure = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Read,
      )
      .await?;
    let action_state = self
      .action_states
      .procedure
      .get(&id)
      .await
      .unwrap_or_default();
    Ok(action_state)
  }
}
