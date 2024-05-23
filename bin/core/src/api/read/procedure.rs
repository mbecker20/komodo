use anyhow::Context;
use monitor_client::{
  api::read::{
    GetProcedure, GetProcedureActionState,
    GetProcedureActionStateResponse, GetProcedureResponse,
    GetProceduresSummary, GetProceduresSummaryResponse,
    ListProcedures, ListProceduresResponse,
  },
  entities::{
    permission::PermissionLevel,
    procedure::{Procedure, ProcedureState},
    user::User,
  },
};
use resolver_api::Resolve;

use crate::{
  resource,
  state::{action_states, procedure_state_cache, State},
};

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

impl Resolve<ListProcedures, User> for State {
  async fn resolve(
    &self,
    ListProcedures { query }: ListProcedures,
    user: User,
  ) -> anyhow::Result<ListProceduresResponse> {
    resource::list_for_user::<Procedure>(query, &user).await
  }
}

impl Resolve<GetProceduresSummary, User> for State {
  async fn resolve(
    &self,
    GetProceduresSummary {}: GetProceduresSummary,
    user: User,
  ) -> anyhow::Result<GetProceduresSummaryResponse> {
    let procedures = resource::list_full_for_user::<Procedure>(
      Default::default(),
      &user,
    )
    .await
    .context("failed to get procedures from db")?;

    let mut res = GetProceduresSummaryResponse::default();

    let cache = procedure_state_cache();
    let action_states = action_states();

    for procedure in procedures {
      res.total += 1;

      match (
        cache.get(&procedure.id).await.unwrap_or_default(),
        action_states
          .procedure
          .get(&procedure.id)
          .await
          .unwrap_or_default()
          .get()?,
      ) {
        (_, action_states) if action_states.running => {
          res.running += 1;
        }
        (ProcedureState::Ok, _) => res.ok += 1,
        (ProcedureState::Failed, _) => res.failed += 1,
        (ProcedureState::Unknown, _) => res.unknown += 1,
        // will never come off the cache in the running state, since that comes from action states
        (ProcedureState::Running, _) => unreachable!(),
      }
    }

    Ok(res)
  }
}

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
