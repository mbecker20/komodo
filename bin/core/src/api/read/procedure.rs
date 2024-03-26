use std::str::FromStr;

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
  entities::{
    permission::PermissionLevel, procedure::Procedure,
    resource::AddFilters, update::ResourceTargetVariant, user::User,
  },
};
use mungos::mongodb::bson::{doc, oid::ObjectId, Document};
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::resource::{
    get_resource_ids_for_non_admin, StateResource,
  },
  state::{action_states, State},
};

#[async_trait]
impl Resolve<GetProcedure, User> for State {
  async fn resolve(
    &self,
    GetProcedure { id }: GetProcedure,
    user: User,
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
impl Resolve<ListProcedures, User> for State {
  async fn resolve(
    &self,
    ListProcedures { query }: ListProcedures,
    user: User,
  ) -> anyhow::Result<ListProceduresResponse> {
    let mut filters = Document::new();
    query.add_filters(&mut filters);
    <State as StateResource<Procedure>>::list_resources_for_user(
      self, filters, &user,
    )
    .await
  }
}

#[async_trait]
impl Resolve<ListProceduresByIds, User> for State {
  async fn resolve(
    &self,
    ListProceduresByIds { ids }: ListProceduresByIds,
    user: User,
  ) -> anyhow::Result<ListProceduresByIdsResponse> {
    <State as StateResource<Procedure>>::list_resources_for_user(
      self,
      doc! { "_id": { "$in": ids } },
      &user,
    )
    .await
  }
}

#[async_trait]
impl Resolve<GetProceduresSummary, User> for State {
  async fn resolve(
    &self,
    GetProceduresSummary {}: GetProceduresSummary,
    user: User,
  ) -> anyhow::Result<GetProceduresSummaryResponse> {
    let query = if user.admin {
      None
    } else {
      let ids = get_resource_ids_for_non_admin(
        &user.id,
        ResourceTargetVariant::Procedure,
      )
      .await?
      .into_iter()
      .flat_map(|id| ObjectId::from_str(&id))
      .collect::<Vec<_>>();
      let query = doc! {
        "_id": { "$in": ids }
      };
      Some(query)
    };
    let total = db_client()
      .await
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
impl Resolve<GetProcedureActionState, User> for State {
  async fn resolve(
    &self,
    GetProcedureActionState { id }: GetProcedureActionState,
    user: User,
  ) -> anyhow::Result<GetProcedureActionStateResponse> {
    let _: Procedure = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Read,
      )
      .await?;
    let action_state =
      action_states().procedure.get(&id).await.unwrap_or_default();
    Ok(action_state)
  }
}
