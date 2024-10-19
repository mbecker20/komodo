use std::time::Duration;

use anyhow::Context;
use komodo_client::entities::{
  action::{
    Action, ActionConfig, ActionConfigDiff, ActionInfo,
    ActionListItem, ActionListItemInfo, ActionQuerySpecifics,
    ActionState, PartialActionConfig,
  },
  config::core::CoreConfig,
  resource::Resource,
  update::Update,
  user::User,
  Operation, ResourceTargetVariant,
};
use mungos::{
  find::find_collect,
  mongodb::{bson::doc, options::FindOneOptions, Collection},
};

use crate::{
  config::core_config,
  state::{action_state_cache, action_states, db_client},
};

impl super::KomodoResource for Action {
  type Config = ActionConfig;
  type PartialConfig = PartialActionConfig;
  type ConfigDiff = ActionConfigDiff;
  type Info = ActionInfo;
  type ListItem = ActionListItem;
  type QuerySpecifics = ActionQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Action
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().actions
  }

  async fn to_list_item(
    action: Resource<Self::Config, Self::Info>,
  ) -> Self::ListItem {
    let state = get_action_state(&action.id).await;
    ActionListItem {
      name: action.name,
      id: action.id,
      tags: action.tags,
      resource_type: ResourceTargetVariant::Action,
      info: ActionListItemInfo {
        state,
        last_run_at: action.info.last_run_at,
      },
    }
  }

  async fn busy(id: &String) -> anyhow::Result<bool> {
    action_states()
      .action
      .get(id)
      .await
      .unwrap_or_default()
      .busy()
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateAction
  }

  fn user_can_create(user: &User) -> bool {
    user.admin
  }

  async fn validate_create_config(
    config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    if config.file_contents.is_none() {
      config.file_contents = Some(default_action_file_contents());
    }
    Ok(())
  }

  async fn post_create(
    _created: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    refresh_action_state_cache().await;
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateAction
  }

  async fn validate_update_config(
    _id: &str,
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_update(
    _updated: &Self,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    refresh_action_state_cache().await;
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteAction
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

pub fn spawn_action_state_refresh_loop() {
  tokio::spawn(async move {
    loop {
      refresh_action_state_cache().await;
      tokio::time::sleep(Duration::from_secs(60)).await;
    }
  });
}

pub async fn refresh_action_state_cache() {
  let _ = async {
    let actions = find_collect(&db_client().actions, None, None)
      .await
      .context("Failed to get Actions from db")?;
    let cache = action_state_cache();
    for action in actions {
      let state = get_action_state_from_db(&action.id).await;
      cache.insert(action.id, state).await;
    }
    anyhow::Ok(())
  }
  .await
  .inspect_err(|e| {
    error!("Failed to refresh Action state cache | {e:#}")
  });
}

async fn get_action_state(id: &String) -> ActionState {
  if action_states()
    .action
    .get(id)
    .await
    .map(|s| s.get().map(|s| s.running))
    .transpose()
    .ok()
    .flatten()
    .unwrap_or_default()
  {
    return ActionState::Running;
  }
  action_state_cache().get(id).await.unwrap_or_default()
}

async fn get_action_state_from_db(id: &str) -> ActionState {
  async {
    let state = db_client()
      .updates
      .find_one(doc! {
        "target.type": "Action",
        "target.id": id,
        "operation": "RunAction"
      })
      .with_options(
        FindOneOptions::builder()
          .sort(doc! { "start_ts": -1 })
          .build(),
      )
      .await?
      .map(|u| {
        if u.success {
          ActionState::Ok
        } else {
          ActionState::Failed
        }
      })
      .unwrap_or(ActionState::Ok);
    anyhow::Ok(state)
  }
  .await
  .inspect_err(|e| {
    warn!("Failed to get Action state for {id} | {e:#}")
  })
  .unwrap_or(ActionState::Unknown)
}

fn default_action_file_contents() -> String {
  let CoreConfig {
    port, ssl_enabled, ..
  } = core_config();
  let protocol = if *ssl_enabled { "https" } else { "http" };
  format!(
    "import {{ KomodoClient }} from 'npm:komodo_client';

const komodo = KomodoClient('{protocol}://localhost:{port}', {{
  type: 'api-key',
  // Leave these as in, actual ones will be interpolated in.
  params: {{ key: '[[ACTION_API_KEY]]', secret: '[[ACTION_API_SECRET]]' }}
}});

async function main() {{

  // 🔴 Your action code here

  const version = await komodo.read('GetVersion', {{}});
  console.log('Komodo version:', version.version);

  // 🔴 =====================
  
}}

main().catch(error => {{
  console.error('Status:', error.response?.status);
  console.error(JSON.stringify(error.response?.data, null, 2));
  Deno.exit(1)
}}).then(() => console.log('\\n🦎 Action finished 🦎'));"
  )
}
