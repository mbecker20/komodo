use anyhow::Context;
use monitor_client::entities::{
  permission::PermissionLevel,
  resource::Resource,
  server::Server,
  stack::{
    PartialStackConfig, Stack, StackConfig, StackConfigDiff,
    StackInfo, StackListItem, StackListItemInfo, StackQuerySpecifics,
    StackState,
  },
  update::{ResourceTargetVariant, Update},
  user::User,
  Operation,
};
use mungos::mongodb::Collection;

use crate::{
  monitor::update_cache_for_server,
  resource,
  state::{action_states, db_client},
};

use super::get_check_permissions;

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
    StackListItem {
      id: stack.id,
      name: stack.name,
      tags: stack.tags,
      resource_type: ResourceTargetVariant::Stack,
      info: StackListItemInfo {
        state: StackState::Unknown,
        server_id: stack.config.server_id,
        git_provider: stack.config.git_provider,
        repo: stack.config.repo,
        branch: stack.config.branch,
        latest_hash: stack.info.latest_hash,
        latest_message: stack.info.latest_message,
      },
    }
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
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user).await
  }

  async fn post_create(
    created: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    if !created.config.server_id.is_empty() {
      let server =
        resource::get::<Server>(&created.config.server_id).await?;
      update_cache_for_server(&server).await;
    }
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateStack
  }

  async fn validate_update_config(
    _id: &str,
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user).await
  }

  async fn post_update(
    updated: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    if !updated.config.server_id.is_empty() {
      let server =
        resource::get::<Server>(&updated.config.server_id).await?;
      update_cache_for_server(&server).await;
    }
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteStack
  }

  async fn pre_delete(
    stack: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()> {
    // If it is Up, it should be taken down
    Ok(())
  }

  async fn post_delete(
    _resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }
}

#[instrument(skip(user))]
async fn validate_config(
  config: &mut PartialStackConfig,
  user: &User,
) -> anyhow::Result<()> {
  if let Some(server_id) = &config.server_id {
    if !server_id.is_empty() {
      let server = get_check_permissions::<Server>(server_id, user, PermissionLevel::Write)
          .await
          .context("cannot create stack on this server. user must have update permissions on the server to perform this action.")?;
      // in case it comes in as name
      config.server_id = Some(server.id);
    }
  }
  Ok(())
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
