use std::collections::HashMap;

use anyhow::Context;
use monitor_client::{
  api::write::{
    CreateVariable, DeleteVariable, UpdateVariableDescription,
    UpdateVariableValue,
  },
  entities::{
    sync::SyncUpdate, update::Log, user::sync_user,
    variable::Variable,
  },
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::state::{db_client, State};

use super::{bold, colored, muted};

pub struct ToUpdateItem {
  pub variable: Variable,
  pub update_value: bool,
  pub update_description: bool,
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
          variable,
        };
        if !item.update_value && !item.update_description {
          continue;
        }
        update.to_update += 1;
        update.log.push_str(&format!(
          "\n\n{}: variable: '{}'\n-------------------",
          colored("UPDATE", "blue"),
          bold(&item.variable.name),
        ));

        let mut lines = Vec::<String>::new();

        if item.update_value {
          lines.push(format!(
            "{}: 'value'\n{}:  {}\n{}:    {}",
            muted("field"),
            muted("from"),
            colored(&original.value, "red"),
            muted("to"),
            colored(&item.variable.value, "green")
          ))
        }

        if item.update_description {
          lines.push(format!(
            "{}: 'description'\n{}:  {}\n{}:    {}",
            muted("field"),
            muted("from"),
            colored(&original.description, "red"),
            muted("to"),
            colored(&item.variable.description, "green")
          ))
        }

        update.log.push('\n');
        update.log.push_str(&lines.join("\n-------------------\n"));
      }
      None => {
        update.to_create += 1;
        update.log.push_str(&format!(
          "\n\n{}: variable: {}\n{}: {}\n{}: {}",
          colored("CREATE", "green"),
          colored(&variable.name, "green"),
          muted("description"),
          variable.description,
          muted("value"),
          variable.value,
        ));
      }
    }
  }

  for name in &to_delete {
    update.log.push_str(&format!(
      "\n\n{}: variable: '{}'\n-------------------",
      colored("DELETE", "red"),
      bold(name),
    ));
  }

  Ok(Some(update))
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
          variable,
        };
        if !item.update_value && !item.update_description {
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

  let mut log = String::from("running updates on Variables");

  for variable in to_create {
    if let Err(e) = State
      .resolve(
        CreateVariable {
          name: variable.name.clone(),
          value: variable.value,
          description: variable.description,
        },
        sync_user().to_owned(),
      )
      .await
    {
      log.push_str(&format!(
        "\n{}: failed to create variable '{}' | {e:#}",
        colored("ERROR", "red"),
        bold(&variable.name)
      ));
    } else {
      log.push_str(&format!(
        "\n{}: {} variable '{}'",
        muted("INFO"),
        colored("created", "green"),
        bold(&variable.name)
      ))
    };
  }

  for ToUpdateItem {
    variable,
    update_value,
    update_description,
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
        log.push_str(&format!(
          "\n{}: failed to update variable value for '{}' | {e:#}",
          colored("ERROR", "red"),
          bold(&variable.name)
        ))
      } else {
        log.push_str(&format!(
          "\n{}: {} variable '{}' value",
          muted("INFO"),
          colored("updated", "blue"),
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
        log.push_str(&format!(
          "\n{}: failed to update variable description for '{}' | {e:#}",
          colored("ERROR", "red"),
          bold(&variable.name)
        ))
      } else {
        log.push_str(&format!(
          "\n{}: {} variable '{}' description",
          muted("INFO"),
          colored("updated", "blue"),
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
      log.push_str(&format!(
        "\n{}: failed to delete variable '{}' | {e:#}",
        colored("ERROR", "red"),
        bold(&variable)
      ))
    } else {
      log.push_str(&format!(
        "\n{}: {} variable '{}'",
        muted("INFO"),
        colored("deleted", "red"),
        bold(&variable)
      ))
    }
  }

  Some(Log::simple("Update Variables", log))
}
