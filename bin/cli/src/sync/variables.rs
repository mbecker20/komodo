use colored::Colorize;
use monitor_client::{
  api::write::{
    CreateVariable, DeleteVariable, UpdateVariableDescription,
    UpdateVariableValue,
  },
  entities::variable::Variable,
};

use crate::{maps::name_to_variable, state::monitor_client};

pub struct ToUpdateItem {
  pub variable: Variable,
  pub update_value: bool,
  pub update_description: bool,
}

pub fn get_updates(
  variables: Vec<Variable>,
  delete: bool,
) -> anyhow::Result<(Vec<Variable>, Vec<ToUpdateItem>, Vec<String>)> {
  let map = name_to_variable();

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
        println!(
          "\n{}: variable: '{}'\n-------------------",
          "UPDATE".blue(),
          item.variable.name.bold(),
        );

        let mut lines = Vec::<String>::new();

        if item.update_value {
          lines.push(format!(
            "{}: 'value'\n{}:  {}\n{}:    {}",
            "field".dimmed(),
            "from".dimmed(),
            original.value.red(),
            "to".dimmed(),
            item.variable.value.green()
          ))
        }

        if item.update_description {
          lines.push(format!(
            "{}: 'description'\n{}:  {}\n{}:    {}",
            "field".dimmed(),
            "from".dimmed(),
            original.description.red(),
            "to".dimmed(),
            item.variable.description.green()
          ))
        }

        println!("{}", lines.join("\n-------------------\n"));

        to_update.push(item);
      }
      None => {
        if variable.description.is_empty() {
          println!(
            "\n{}: variable: {}\n{}: {}",
            "CREATE".green(),
            variable.name.bold().green(),
            "value".dimmed(),
            variable.value,
          );
        } else {
          println!(
            "\n{}: variable: {}\n{}: {}\n{}: {}",
            "CREATE".green(),
            variable.name.bold().green(),
            "description".dimmed(),
            variable.description,
            "value".dimmed(),
            variable.value,
          );
        }
        to_create.push(variable)
      }
    }
  }

  for name in &to_delete {
    println!(
      "\n{}: variable: '{}'\n-------------------",
      "DELETE".red(),
      name.bold(),
    );
  }

  Ok((to_create, to_update, to_delete))
}

pub async fn run_updates(
  to_create: Vec<Variable>,
  to_update: Vec<ToUpdateItem>,
  to_delete: Vec<String>,
) {
  for variable in to_create {
    if let Err(e) = monitor_client()
      .write(CreateVariable {
        name: variable.name.clone(),
        value: variable.value,
        description: variable.description,
      })
      .await
    {
      warn!("failed to create variable {} | {e:#}", variable.name);
    } else {
      info!(
        "{} variable '{}'",
        "created".green().bold(),
        variable.name.bold(),
      );
    };
  }

  for ToUpdateItem {
    variable,
    update_value,
    update_description,
  } in to_update
  {
    if update_value {
      if let Err(e) = monitor_client()
        .write(UpdateVariableValue {
          name: variable.name.clone(),
          value: variable.value,
        })
        .await
      {
        warn!(
          "failed to update variable value for {} | {e:#}",
          variable.name
        );
      } else {
        info!(
          "{} variable '{}' value",
          "updated".blue().bold(),
          variable.name.bold(),
        );
      };
    }
    if update_description {
      if let Err(e) = monitor_client()
        .write(UpdateVariableDescription {
          name: variable.name.clone(),
          description: variable.description,
        })
        .await
      {
        warn!(
          "failed to update variable description for {} | {e:#}",
          variable.name
        );
      } else {
        info!(
          "{} variable '{}' description",
          "updated".blue().bold(),
          variable.name.bold(),
        );
      };
    }
  }

  for variable in to_delete {
    if let Err(e) = crate::state::monitor_client()
      .write(DeleteVariable {
        name: variable.clone(),
      })
      .await
    {
      warn!("failed to delete variable {variable} | {e:#}",);
    } else {
      info!(
        "{} variable '{}'",
        "deleted".red().bold(),
        variable.bold(),
      );
    }
  }
}
