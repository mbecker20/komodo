use command::run_komodo_command;
use komodo_client::{
  api::{
    execute::RunAction,
    user::{CreateApiKey, CreateApiKeyResponse, DeleteApiKey},
  },
  entities::{
    action::Action,
    config::core::CoreConfig,
    permission::PermissionLevel,
    update::Update,
    user::{action_user, User},
  },
};
use mungos::{by_id::update_one_by_id, mongodb::bson::to_document};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::update::update_update,
  resource::{self, refresh_action_state_cache},
  state::{action_states, db_client, State},
};

impl Resolve<RunAction, (User, Update)> for State {
  async fn resolve(
    &self,
    RunAction { action }: RunAction,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let action = resource::get_check_permissions::<Action>(
      &action,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // get the action state for the action (or insert default).
    let action_state = action_states()
      .action
      .get_or_insert_default(&action.id)
      .await;

    // This will set action state back to default when dropped.
    // Will also check to ensure action not already busy before updating.
    let _action_guard =
      action_state.update(|state| state.running = true)?;

    update_update(update.clone()).await?;

    let CreateApiKeyResponse { key, secret } = State
      .resolve(
        CreateApiKey {
          name: update.id.clone(),
          expires: 0,
        },
        action_user().to_owned(),
      )
      .await?;

    let contents =
      format_contents(&action.config.file_contents, &key, &secret);

    let mut res = run_komodo_command(
      "Execute Action",
      None,
      format!("deno eval \"{contents}\""),
    )
    .await;

    res.command = res
      .command
      .replace(&key, "<API_KEY>")
      .replace(&secret, "<API_SECRET>");
    res.stdout = res
      .stdout
      .replace(&key, "<API_KEY>")
      .replace(&secret, "<API_SECRET>");
    res.stderr = res
      .stderr
      .replace(&key, "<API_KEY>")
      .replace(&secret, "<API_SECRET>");

    if let Err(e) = State
      .resolve(DeleteApiKey { key }, action_user().to_owned())
      .await
    {
      warn!(
        "Failed to delete API key after action execution | {e:#}"
      );
    };

    update.logs.push(res);
    update.finalize();

    // Need to manually update the update before cache refresh,
    // and before broadcast with update_update.
    // The Err case of to_document should be unreachable,
    // but will fail to update cache in that case.
    if let Ok(update_doc) = to_document(&update) {
      let _ = update_one_by_id(
        &db_client().updates,
        &update.id,
        mungos::update::Update::Set(update_doc),
        None,
      )
      .await;
      refresh_action_state_cache().await;
    }

    update_update(update.clone()).await?;

    Ok(update)
  }
}

fn format_contents(
  contents: &str,
  key: &str,
  secret: &str,
) -> String {
  let CoreConfig {
    port, ssl_enabled, ..
  } = core_config();

  let protocol = if *ssl_enabled { "https" } else { "http" };

  format!(
    "
import {{ KomodoClient }} from 'npm:komodo_client';

async function main() {{
  const komodo = KomodoClient('{protocol}://localhost:{port}', {{
    type: 'api-key',
    params: {{ key: '{key}', secret: '{secret}' }}
  }});

  try {{
    {contents}
  }} catch (error) {{
		console.error('Status:', error.response?.status);
		console.error(JSON.stringify(error.response?.data, null, 2));
  }}
}}

if (import.meta.main) {{
  main().then(() => console.log('Action finished'));
}}"
  )
}
