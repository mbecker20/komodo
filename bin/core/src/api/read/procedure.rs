use std::str::FromStr;

use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::read::{
    GetProcedure, GetProcedureActionState,
    GetProcedureActionStateResponse, GetProcedureResponse,
    GetProceduresSummary, GetProceduresSummaryResponse,
    ListProcedures, ListProceduresResponse,
  },
  entities::{
    permission::PermissionLevel, procedure::Procedure,
    update::ResourceTargetVariant, user::User,
  },
};
use mungos::mongodb::bson::{doc, oid::ObjectId};
use resolver_api::Resolve;

use crate::{
  helpers::query::get_resource_ids_for_non_admin,
  resource,
  state::{action_states, db_client, State},
};

#[async_trait]
impl Resolve<GetProcedure, User> for State {
  async fn resolve(
    &self,
    GetProcedure { procedure }: GetProcedure,
    user: User,
  ) -> anyhow::Result<GetProcedureResponse> {
    resource::get_check_permissions::<Procedure>(
      &procedure,
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
    resource::list_for_user::<Procedure>(query, &user).await
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
    GetProcedureActionState { procedure }: GetProcedureActionState,
    user: User,
  ) -> anyhow::Result<GetProcedureActionStateResponse> {
    let procedure = resource::get_check_permissions::<Procedure>(
      &procedure,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .procedure
      .get(&procedure.id)
      .await
      .unwrap_or_default()
      .get()?;
    Ok(action_state)
  }
}
