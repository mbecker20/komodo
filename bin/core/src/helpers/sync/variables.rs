use std::collections::HashMap;

use anyhow::Context;
use formatting::{bold, colored, muted, Color};
use komodo_client::{
  api::write::{
    CreateVariable, DeleteVariable, UpdateVariableDescription, UpdateVariableIsSecret, UpdateVariableValue
  },
  entities::{
    sync::SyncUpdate, update::Log, user::sync_user,
    variable::Variable,
  },
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::state::{db_client, State};

pub struct ToUpdateItem {
  pub variable: Variable,
  pub update_value: bool,
  pub update_description: bool,
  pub update_is_secret: bool,
}

pub async fn get_updates_for_view(
  variables: Vec<Variable>,
  delete: bool,
) -> anyhow::Result<Option<SyncUpdate>> {
  let map = find_collect(&db_client().await.variables, None, None)
    .await
    .context("failed to query db for variables")?
    .into_iter()
    .map(|v| (v.name.clone(), v))
    .collect::<HashMap<_, _>>();

  let mut update = SyncUpdate {
    log: String::from("Variable Updates"),
    ..Default::default()
  };

  let mut to_delete = Vec::<String>::new();

  if delete {
    for variable in map.values() {
      if !variables.iter().any(|v| v.name == variable.name) {
        update.to_delete += 1;
        to_delete.push(variable.name.clone());
      }
    }
  }

  for variable in variables {
    match map.get(&variable.name) {
      Some(original) => {
        let item = ToUpdateItem {
          update_value: original.value != variable.value,
          update_description: original.description
            != variable.description,
          update_is_secret: original.is_secret != variable.is_secret,
          variable,
        };
        if !item.update_value && !item.update_description {
          continue;
        }
        update.to_update += 1;
        update.log.push_str(&format!(
          "\n\n{}: variable: '{}'\n-------------------",
          colored("UPDATE", Color::Blue),
          bold(&item.variable.name),
        ));

        let mut lines = Vec::<String>::new();

        if item.update_value {
          let mut log = format!("{}: 'value'\n", muted("field"),);
          if item.variable.is_secret {
            log.push_str(&format!(
              "{}:  {}\n{}:    {}",
              muted("from"),
              colored(
                original.value.replace(|_| true, "#"),
                Color::Red
              ),
              muted("to"),
              colored(
                item.variable.value.replace(|_| true, "#"),
                Color::Green
              )
            ));
          } else {
            log.push_str(&format!(
              "{}:  {}\n{}:    {}",
              muted("from"),
              colored(&original.value, Color::Red),
              muted("to"),
              colored(&item.variable.value, Color::Green)
            ));
          }
          lines.push(log);
        }

        if item.update_description {
          lines.push(format!(
            "{}: 'description'\n{}:  {}\n{}:    {}",
            muted("field"),
            muted("from"),
            colored(&original.description, Color::Red),
            muted("to"),
            colored(&item.variable.description, Color::Green)
          ))
        }

        if item.update_is_secret {
          lines.push(format!(
            "{}: 'is_secret'\n{}:  {}\n{}:    {}",
            muted("field"),
            muted("from"),
            colored(original.is_secret, Color::Red),
            muted("to"),
            colored(item.variable.is_secret, Color::Green)
          ))
        }

        update.log.push('\n');
        update.log.push_str(&lines.join("\n-------------------\n"));
      }
      None => {
        update.to_create += 1;
        if variable.description.is_empty() {
          update.log.push_str(&format!(
            "\n\n{}: variable: {}",
            colored("CREATE", Color::Green),
            colored(&variable.name, Color::Green),
          ));
          if variable.is_secret {
            update
              .log
              .push_str(&format!("\n{}: true", muted("is secret"),));
          } else {
            update.log.push_str(&format!(
              "\n{}: {}",
              muted("value"),
              variable.value,
            ));
          }
        } else {
          update.log.push_str(&format!(
            "\n\n{}: variable: {}\n{}: {}",
            colored("CREATE", Color::Green),
            colored(&variable.name, Color::Green),
            muted("description"),
            variable.description,
          ));
          if variable.is_secret {
            update
              .log
              .push_str(&format!("\n{}: true", muted("is secret"),));
          } else {
            update.log.push_str(&format!(
              "\n{}: {}",
              muted("value"),
              variable.value,
            ));
          }
        }
      }
    }
  }

  for name in &to_delete {
    update.log.push_str(&format!(
      "\n\n{}: variable: '{}'\n-------------------",
      colored("DELETE", Color::Red),
      bold(name),
    ));
  }

  let any_change = update.to_create > 0
    || update.to_update > 0
    || update.to_delete > 0;

  Ok(any_change.then_some(update))
}

pub async fn get_updates_for_execution(
  variables: Vec<Variable>,
  delete: bool,
) -> anyhow::Result<(Vec<Variable>, Vec<ToUpdateItem>, Vec<String>)> {
  let map = find_collect(&db_client().await.variables, None, None)
    .await
    .context("failed to query db for variables")?
    .into_iter()
    .map(|v| (v.name.clone(), v))
    .collect::<HashMap<_, _>>();

  let mut to_create = Vec::<Variable>::new();
  let mut to_update = Vec::<ToUpdateItem>::new();
  let mut to_delete = Vec::<String>::new();

  if delete {
    for variable in map.values() {
      if !variables.iter().any(|v| v.name == variable.name) {
        to_delete.push(variable.name.clone());
      }
    }
  }

  for variable in variables {
    match map.get(&variable.name) {
      Some(original) => {
        let item = ToUpdateItem {
          update_value: original.value != variable.value,
          update_description: original.description
            != variable.description,
          update_is_secret: original.is_secret != variable.is_secret,
          variable,
        };
        if !item.update_value
          && !item.update_description
          && !item.update_is_secret
        {
          continue;
        }

        to_update.push(item);
      }
      None => to_create.push(variable),
    }
  }

  Ok((to_create, to_update, to_delete))
}

pub async fn run_updates(
  to_create: Vec<Variable>,
  to_update: Vec<ToUpdateItem>,
  to_delete: Vec<String>,
) -> Option<Log> {
  if to_create.is_empty()
    && to_update.is_empty()
    && to_delete.is_empty()
  {
    return None;
  }

  let mut has_error = false;
  let mut log = String::from("running updates on Variables");

  for variable in to_create {
    if let Err(e) = State
      .resolve(
        CreateVariable {
          name: variable.name.clone(),
          value: variable.value,
          description: variable.description,
          is_secret: variable.is_secret,
        },
        sync_user().to_owned(),
      )
      .await
    {
      has_error = true;
      log.push_str(&format!(
        "\n{}: failed to create variable '{}' | {e:#}",
        colored("ERROR", Color::Red),
        bold(&variable.name)
      ));
    } else {
      log.push_str(&format!(
        "\n{}: {} variable '{}'",
        muted("INFO"),
        colored("created", Color::Green),
        bold(&variable.name)
      ))
    };
  }

  for ToUpdateItem {
    variable,
    update_value,
    update_description,
    update_is_secret,
  } in to_update
  {
    if update_value {
      if let Err(e) = State
        .resolve(
          UpdateVariableValue {
            name: variable.name.clone(),
            value: variable.value,
          },
          sync_user().to_owned(),
        )
        .await
      {
        has_error = true;
        log.push_str(&format!(
          "\n{}: failed to update variable value for '{}' | {e:#}",
          colored("ERROR", Color::Red),
          bold(&variable.name)
        ))
      } else {
        log.push_str(&format!(
          "\n{}: {} variable '{}' value",
          muted("INFO"),
          colored("updated", Color::Blue),
          bold(&variable.name)
        ))
      };
    }
    if update_description {
      if let Err(e) = State
        .resolve(
          UpdateVariableDescription {
            name: variable.name.clone(),
            description: variable.description,
          },
          sync_user().to_owned(),
        )
        .await
      {
        has_error = true;
        log.push_str(&format!(
          "\n{}: failed to update variable description for '{}' | {e:#}",
          colored("ERROR", Color::Red),
          bold(&variable.name)
        ))
      } else {
        log.push_str(&format!(
          "\n{}: {} variable '{}' description",
          muted("INFO"),
          colored("updated", Color::Blue),
          bold(&variable.name)
        ))
      };
    }
    if update_is_secret {
      if let Err(e) = State
        .resolve(
          UpdateVariableIsSecret {
            name: variable.name.clone(),
            is_secret: variable.is_secret,
          },
          sync_user().to_owned(),
        )
        .await
      {
        has_error = true;
        log.push_str(&format!(
          "\n{}: failed to update variable is secret for '{}' | {e:#}",
          colored("ERROR", Color::Red),
          bold(&variable.name)
        ))
      } else {
        log.push_str(&format!(
          "\n{}: {} variable '{}' is secret",
          muted("INFO"),
          colored("updated", Color::Blue),
          bold(&variable.name)
        ))
      };
    }
  }

  for variable in to_delete {
    if let Err(e) = State
      .resolve(
        DeleteVariable {
          name: variable.clone(),
        },
        sync_user().to_owned(),
      )
      .await
    {
      has_error = true;
      log.push_str(&format!(
        "\n{}: failed to delete variable '{}' | {e:#}",
        colored("ERROR", Color::Red),
        bold(&variable)
      ))
    } else {
      log.push_str(&format!(
        "\n{}: {} variable '{}'",
        muted("INFO"),
        colored("deleted", Color::Red),
        bold(&variable)
      ))
    }
  }

  let stage = "Update Variables";
  Some(if has_error {
    Log::error(stage, log)
  } else {
    Log::simple(stage, log)
  })
}
