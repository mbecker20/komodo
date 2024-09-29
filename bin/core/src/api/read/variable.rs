use anyhow::Context;
use komodo_client::{
  api::read::{
    GetVariable, GetVariableResponse, ListVariables,
    ListVariablesResponse,
  },
  entities::user::User,
};
use mongo_indexed::doc;
use mungos::{find::find_collect, mongodb::options::FindOptions};
use resolver_api::Resolve;

use crate::{
  helpers::query::get_variable,
  state::{db_client, State},
};

impl Resolve<GetVariable, User> for State {
  async fn resolve(
    &self,
    GetVariable { name }: GetVariable,
    user: User,
  ) -> anyhow::Result<GetVariableResponse> {
    let mut variable = get_variable(&name).await?;
    if !variable.is_secret || user.admin {
      return Ok(variable);
    }
    variable.value = "#".repeat(variable.value.len());
    Ok(variable)
  }
}

impl Resolve<ListVariables, User> for State {
  async fn resolve(
    &self,
    ListVariables {}: ListVariables,
    user: User,
  ) -> anyhow::Result<ListVariablesResponse> {
    let variables = find_collect(
      &db_client().variables,
      None,
      FindOptions::builder().sort(doc! { "name": 1 }).build(),
    )
    .await
    .context("failed to query db for variables")?;
    if user.admin {
      return Ok(variables);
    }
    let variables = variables
      .into_iter()
      .map(|mut variable| {
        if variable.is_secret {
          variable.value = "#".repeat(variable.value.len());
        }
        variable
      })
      .collect();
    Ok(variables)
  }
}
