use anyhow::Context;
use monitor_client::{
  api::read::*,
  entities::{
    permission::PermissionLevel,
    sync::{
      PendingSyncUpdatesData, ResourceSync, ResourceSyncActionState,
      ResourceSyncListItem, ResourceSyncState,
    },
    user::User,
  },
};
use resolver_api::Resolve;

use crate::{
  resource,
  state::{action_states, resource_sync_state_cache, State},
};

impl Resolve<GetResourceSync, User> for State {
  async fn resolve(
    &self,
    GetResourceSync { sync }: GetResourceSync,
    user: User,
  ) -> anyhow::Result<ResourceSync> {
    resource::get_check_permissions::<ResourceSync>(
      &sync,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

impl Resolve<ListResourceSyncs, User> for State {
  async fn resolve(
    &self,
    ListResourceSyncs { query }: ListResourceSyncs,
    user: User,
  ) -> anyhow::Result<Vec<ResourceSyncListItem>> {
    resource::list_for_user::<ResourceSync>(query, &user).await
  }
}

impl Resolve<ListFullResourceSyncs, User> for State {
  async fn resolve(
    &self,
    ListFullResourceSyncs { query }: ListFullResourceSyncs,
    user: User,
  ) -> anyhow::Result<ListFullResourceSyncsResponse> {
    resource::list_full_for_user::<ResourceSync>(query, &user).await
  }
}

impl Resolve<GetResourceSyncActionState, User> for State {
  async fn resolve(
    &self,
    GetResourceSyncActionState { sync }: GetResourceSyncActionState,
    user: User,
  ) -> anyhow::Result<ResourceSyncActionState> {
    let sync = resource::get_check_permissions::<ResourceSync>(
      &sync,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .resource_sync
      .get(&sync.id)
      .await
      .unwrap_or_default()
      .get()?;
    Ok(action_state)
  }
}

impl Resolve<GetResourceSyncsSummary, User> for State {
  async fn resolve(
    &self,
    GetResourceSyncsSummary {}: GetResourceSyncsSummary,
    user: User,
  ) -> anyhow::Result<GetResourceSyncsSummaryResponse> {
    let resource_syncs =
      resource::list_full_for_user::<ResourceSync>(
        Default::default(),
        &user,
      )
      .await
      .context("failed to get resource_syncs from db")?;

    let mut res = GetResourceSyncsSummaryResponse::default();

    let cache = resource_sync_state_cache();
    let action_states = action_states();

    for resource_sync in resource_syncs {
      res.total += 1;

      match resource_sync.info.pending.data {
        PendingSyncUpdatesData::Ok(data) => {
          if !data.no_updates() {
            res.pending += 1;
            continue;
          }
        }
        PendingSyncUpdatesData::Err(_) => {
          res.failed += 1;
          continue;
        }
      }

      match (
        cache.get(&resource_sync.id).await.unwrap_or_default(),
        action_states
          .resource_sync
          .get(&resource_sync.id)
          .await
          .unwrap_or_default()
          .get()?,
      ) {
        (_, action_states) if action_states.syncing => {
          res.syncing += 1;
        }
        (ResourceSyncState::Ok, _) => res.ok += 1,
        (ResourceSyncState::Failed, _) => res.failed += 1,
        (ResourceSyncState::Unknown, _) => res.unknown += 1,
        // will never come off the cache in the building state, since that comes from action states
        (ResourceSyncState::Syncing, _) => {
          unreachable!()
        }
        (ResourceSyncState::Pending, _) => {
          unreachable!()
        }
      }
    }

    Ok(res)
  }
}
