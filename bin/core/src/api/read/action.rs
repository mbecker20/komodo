use anyhow::Context;
use komodo_client::{
  api::read::*,
  entities::{
    action::{
      Action, ActionActionState, ActionListItem, ActionState,
    },
    permission::PermissionLevel,
    user::User,
  },
};
use resolver_api::Resolve;

use crate::{
  helpers::query::get_all_tags,
  resource,
  state::{action_state_cache, action_states, State},
};

impl Resolve<GetAction, User> for State {
  async fn resolve(
    &self,
    GetAction { action }: GetAction,
    user: User,
  ) -> anyhow::Result<Action> {
    resource::get_check_permissions::<Action>(
      &action,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

impl Resolve<ListActions, User> for State {
  async fn resolve(
    &self,
    ListActions { query }: ListActions,
    user: User,
  ) -> anyhow::Result<Vec<ActionListItem>> {
    let all_tags = if query.tags.is_empty() {
      vec![]
    } else {
      get_all_tags(None).await?
    };
    resource::list_for_user::<Action>(query, &user, &all_tags).await
  }
}

impl Resolve<ListFullActions, User> for State {
  async fn resolve(
    &self,
    ListFullActions { query }: ListFullActions,
    user: User,
  ) -> anyhow::Result<ListFullActionsResponse> {
    let all_tags = if query.tags.is_empty() {
      vec![]
    } else {
      get_all_tags(None).await?
    };
    resource::list_full_for_user::<Action>(query, &user, &all_tags)
      .await
  }
}

impl Resolve<GetActionActionState, User> for State {
  async fn resolve(
    &self,
    GetActionActionState { action }: GetActionActionState,
    user: User,
  ) -> anyhow::Result<ActionActionState> {
    let action = resource::get_check_permissions::<Action>(
      &action,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .action
      .get(&action.id)
      .await
      .unwrap_or_default()
      .get()?;
    Ok(action_state)
  }
}

impl Resolve<GetActionsSummary, User> for State {
  async fn resolve(
    &self,
    GetActionsSummary {}: GetActionsSummary,
    user: User,
  ) -> anyhow::Result<GetActionsSummaryResponse> {
    let actions = resource::list_full_for_user::<Action>(
      Default::default(),
      &user,
      &[],
    )
    .await
    .context("failed to get actions from db")?;

    let mut res = GetActionsSummaryResponse::default();

    let cache = action_state_cache();
    let action_states = action_states();

    for action in actions {
      res.total += 1;

      match (
        cache.get(&action.id).await.unwrap_or_default(),
        action_states
          .action
          .get(&action.id)
          .await
          .unwrap_or_default()
          .get()?,
      ) {
        (_, action_states) if action_states.running => {
          res.running += 1;
        }
        (ActionState::Ok, _) => res.ok += 1,
        (ActionState::Failed, _) => res.failed += 1,
        (ActionState::Unknown, _) => res.unknown += 1,
        // will never come off the cache in the running state, since that comes from action states
        (ActionState::Running, _) => unreachable!(),
      }
    }

    Ok(res)
  }
}
