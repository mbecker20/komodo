use anyhow::{anyhow, Context};
use monitor_client::{
  api::write::{
    CreateVariable, CreateVariableResponse, DeleteVariable,
    DeleteVariableResponse, UpdateVariableDescription,
    UpdateVariableDescriptionResponse, UpdateVariableValue,
    UpdateVariableValueResponse,
  },
  entities::{
    update::ResourceTarget, user::User, variable::Variable, Operation,
  },
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::{
  helpers::{
    query::get_variable,
    update::{add_update, make_update},
  },
  state::{db_client, State},
};

impl Resolve<CreateVariable, User> for State {
  async fn resolve(
    &self,
    CreateVariable {
      name,
      value,
      description,
    }: CreateVariable,
    user: User,
  ) -> anyhow::Result<CreateVariableResponse> {
    if !user.admin {
      return Err(anyhow!("only admins can create variables"));
    }

    let variable = Variable {
      name,
      value,
      description,
    };

    db_client()
      .await
      .variables
      .insert_one(&variable)
      .await
      .context("failed to create variable on db")?;

    let mut update = make_update(
      ResourceTarget::system(),
      Operation::CreateVariable,
      &user,
    );

    update
      .push_simple_log("create variable", format!("{variable:#?}"));
    update.finalize();

    add_update(update).await?;

    get_variable(&variable.name).await
  }
}

impl Resolve<UpdateVariableValue, User> for State {
  async fn resolve(
    &self,
    UpdateVariableValue { name, value }: UpdateVariableValue,
    user: User,
  ) -> anyhow::Result<UpdateVariableValueResponse> {
    if !user.admin {
      return Err(anyhow!("only admins can create variables"));
    }

    let variable = get_variable(&name).await?;

    if value == variable.value {
      return Err(anyhow!("no change"));
    }

    db_client()
      .await
      .variables
      .update_one(
        doc! { "name": &name },
        doc! { "$set": { "value": &value } },
      )
      .await
      .context("failed to update variable value on db")?;

    let mut update = make_update(
      ResourceTarget::system(),
      Operation::UpdateVariableValue,
      &user,
    );

    update.push_simple_log(
      "update variable value",
      format!(
        "<span class=\"text-muted-foreground\">variable</span>: '{name}'\n<span class=\"text-muted-foreground\">from</span>: <span class=\"text-red-500\">{}</span>\n<span class=\"text-muted-foreground\">to</span>:   <span class=\"text-green-500\">{value}</span>",
        variable.value
      ),
    );
    update.finalize();

    add_update(update).await?;

    get_variable(&name).await
  }
}

impl Resolve<UpdateVariableDescription, User> for State {
  async fn resolve(
    &self,
    UpdateVariableDescription { name, description }: UpdateVariableDescription,
    user: User,
  ) -> anyhow::Result<UpdateVariableDescriptionResponse> {
    if !user.admin {
      return Err(anyhow!("only admins can create variables"));
    }
    db_client()
      .await
      .variables
      .update_one(
        doc! { "name": &name },
        doc! { "$set": { "description": &description } },
      )
      .await
      .context("failed to update variable description on db")?;
    get_variable(&name).await
  }
}

impl Resolve<DeleteVariable, User> for State {
  async fn resolve(
    &self,
    DeleteVariable { name }: DeleteVariable,
    user: User,
  ) -> anyhow::Result<DeleteVariableResponse> {
    if !user.admin {
      return Err(anyhow!("only admins can create variables"));
    }
    let variable = get_variable(&name).await?;
    db_client()
      .await
      .variables
      .delete_one(doc! { "name": &name })
      .await
      .context("failed to delete variable on db")?;

    let mut update = make_update(
      ResourceTarget::system(),
      Operation::DeleteVariable,
      &user,
    );

    update
      .push_simple_log("delete variable", format!("{variable:#?}"));
    update.finalize();

    add_update(update).await?;

    Ok(variable)
  }
}
