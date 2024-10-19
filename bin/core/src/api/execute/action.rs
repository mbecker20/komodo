use std::collections::HashSet;

use command::run_komodo_command;
use komodo_client::{
  api::{
    execute::RunAction,
    user::{CreateApiKey, CreateApiKeyResponse, DeleteApiKey},
  },
  entities::{
    action::Action,
    permission::PermissionLevel,
    update::Update,
    user::{action_user, User},
  },
};
use mungos::{by_id::update_one_by_id, mongodb::bson::to_document};
use resolver_api::Resolve;

use crate::{
  helpers::{
    interpolate::{
      add_interp_update_log,
      interpolate_variables_secrets_into_string,
    },
    query::get_variables_and_secrets,
    update::update_update,
  },
  resource::{self, refresh_action_state_cache},
  state::{action_states, db_client, State},
};

impl Resolve<RunAction, (User, Update)> for State {
  async fn resolve(
    &self,
    RunAction { action }: RunAction,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let mut action = resource::get_check_permissions::<Action>(
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

    let replacers =
      interpolate(&mut action, &mut update, key.clone(), secret)
        .await?
        .into_iter()
        .collect::<Vec<_>>();

    let mut res = run_komodo_command(
      "Execute Action",
      None,
      format!(
        "deno eval \"{}\"",
        // Escape double quotes in file contents.
        action.config.file_contents.replace("\"", "\\\"")
      ),
      false,
    )
    .await;

    res.command = String::from("deno eval '<file_contents>'");
    res.stdout = svi::replace_in_string(&res.stdout, &replacers);
    res.stderr = svi::replace_in_string(&res.stderr, &replacers);

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

async fn interpolate(
  action: &mut Action,
  update: &mut Update,
  key: String,
  secret: String,
) -> anyhow::Result<HashSet<(String, String)>> {
  let mut vars_and_secrets = get_variables_and_secrets().await?;

  vars_and_secrets
    .secrets
    .insert(String::from("ACTION_API_KEY"), key);
  vars_and_secrets
    .secrets
    .insert(String::from("ACTION_API_SECRET"), secret);

  let mut global_replacers = HashSet::new();
  let mut secret_replacers = HashSet::new();

  interpolate_variables_secrets_into_string(
    &vars_and_secrets,
    &mut action.config.file_contents,
    &mut global_replacers,
    &mut secret_replacers,
  )?;

  add_interp_update_log(update, &global_replacers, &secret_replacers);

  Ok(secret_replacers)
}
