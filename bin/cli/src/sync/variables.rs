use colored::Colorize;
use monitor_client::{
  api::write::{
    CreateVariable, UpdateVariableDescription, UpdateVariableValue,
  },
  entities::variable::Variable,
};

use crate::{cli_args, maps::name_to_variable, monitor_client};

pub struct ToUpdateItem {
  pub variable: Variable,
  pub update_value: bool,
  pub update_description: bool,
}

pub async fn get_updates(
  variables: Vec<Variable>,
) -> anyhow::Result<(Vec<Variable>, Vec<ToUpdateItem>)> {
  let map = name_to_variable();

  let mut to_create = Vec::<Variable>::new();
  let mut to_update = Vec::<ToUpdateItem>::new();

  let quiet = cli_args().quiet;

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
        if !quiet {
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
        }
        to_update.push(item);
      }
      None => {
        if !quiet {
          println!(
            "\n{}: variable: {}\n{}: {}\n{}: {}",
            "CREATE".green(),
            variable.name.bold().green(),
            "description".dimmed(),
            variable.description,
            "value".dimmed(),
            variable.value,
          )
        }
        to_create.push(variable)
      }
    }
  }

  if quiet && !to_create.is_empty() {
    println!(
      "\nVARIABLES TO CREATE: {}",
      to_create
        .iter()
        .map(|item| item.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    );
  }

  if quiet && !to_update.is_empty() {
    println!(
      "\nVARIABLES TO UPDATE: {}",
      to_update
        .iter()
        .map(|item| item.variable.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    );
  }

  Ok((to_create, to_update))
}

pub async fn run_updates(
  to_create: Vec<Variable>,
  to_update: Vec<ToUpdateItem>,
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
      };
    }
  }
}
