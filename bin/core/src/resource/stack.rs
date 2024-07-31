use std::time::Duration;

use anyhow::Context;
use mongo_indexed::doc;
use monitor_client::entities::{
  resource::Resource,
  stack::{
    PartialStackConfig, Stack, StackConfig, StackConfigDiff,
    StackInfo, StackListItem, StackListItemInfo, StackQuerySpecifics,
    StackState,
  },
  update::{ResourceTargetVariant, Update},
  user::User,
  Operation,
};
use mungos::{
  find::find_collect,
  mongodb::{options::FindOneOptions, Collection},
};

use crate::state::{
  action_states, db_client, resource_sync_state_cache,
};

impl super::MonitorResource for Stack {
  type Config = StackConfig;
  type PartialConfig = PartialStackConfig;
  type ConfigDiff = StackConfigDiff;
  type Info = StackInfo;
  type ListItem = StackListItem;
  type QuerySpecifics = StackQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Stack
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().await.stacks
  }

  async fn to_list_item(
    stack: Resource<Self::Config, Self::Info>,
  ) -> Self::ListItem {
    // let state = get_resource_sync_state(
    //   &resource_sync.id,
    //   &resource_sync.info.pending.data,
    // )
    // .await;
    // StackListItem {
    //   id: resource_sync.id,
    //   name: resource_sync.name,
    //   tags: resource_sync.tags,
    //   resource_type: ResourceTargetVariant::Stack,
    //   info: StackListItemInfo {
    //     state,
    //   },
    // }
    todo!()
  }

  async fn busy(id: &String) -> anyhow::Result<bool> {
    action_states()
      .stack
      .get(id)
      .await
      .unwrap_or_default()
      .busy()
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateStack
  }

  fn user_can_create(user: &User) -> bool {
    user.admin
  }

  async fn validate_create_config(
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_create(
    _created: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateStack
  }

  async fn validate_update_config(
    _id: &str,
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_update(
    _updated: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteStack
  }

  async fn pre_delete(
    _resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_delete(
    _resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }
}

// pub fn spawn_resource_sync_state_refresh_loop() {
//   tokio::spawn(async move {
//     loop {
//       refresh_resource_sync_state_cache().await;
//       tokio::time::sleep(Duration::from_secs(60)).await;
//     }
//   });
// }

// pub async fn refresh_resource_sync_state_cache() {
//   let _ = async {
//     let resource_syncs =
//       find_collect(&db_client().await.resource_syncs, None, None)
//         .await
//         .context("failed to get resource_syncs from db")?;
//     let cache = resource_sync_state_cache();
//     for resource_sync in resource_syncs {
//       let state =
//         get_resource_sync_state_from_db(&resource_sync.id).await;
//       cache.insert(resource_sync.id, state).await;
//     }
//     anyhow::Ok(())
//   }
//   .await
//   .inspect_err(|e| {
//     error!("failed to refresh resource_sync state cache | {e:#}")
//   });
// }

// async fn get_resource_sync_state(
//   id: &String,
//   data: &PendingSyncUpdatesData,
// ) -> StackState {
//   if let Some(state) = action_states()
//     .resource_sync
//     .get(id)
//     .await
//     .and_then(|s| {
//       s.get()
//         .map(|s| {
//           if s.syncing {
//             Some(StackState::Syncing)
//           } else {
//             None
//           }
//         })
//         .ok()
//     })
//     .flatten()
//   {
//     return state;
//   }
//   let data = match data {
//     PendingSyncUpdatesData::Err(_) => return StackState::Failed,
//     PendingSyncUpdatesData::Ok(data) => data,
//   };
//   if !data.no_updates() {
//     return StackState::Pending;
//   }
//   resource_sync_state_cache()
//     .get(id)
//     .await
//     .unwrap_or_default()
// }

// async fn get_resource_sync_state_from_db(id: &str) -> StackState {
//   async {
//     let state = db_client()
//       .await
//       .updates
//       .find_one(doc! {
//         "target.type": "Stack",
//         "target.id": id,
//         "operation": "RunSync"
//       })
//       .with_options(
//         FindOneOptions::builder()
//           .sort(doc! { "start_ts": -1 })
//           .build(),
//       )
//       .await?
//       .map(|u| {
//         if u.success {
//           StackState::Ok
//         } else {
//           StackState::Failed
//         }
//       })
//       .unwrap_or(StackState::Ok);
//     anyhow::Ok(state)
//   }
//   .await
//   .inspect_err(|e| {
//     warn!(
//       "failed to get resource sync state from db for {id} | {e:#}"
//     )
//   })
//   .unwrap_or(StackState::Unknown)
// }
