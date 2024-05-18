use anyhow::Context;
use monitor_client::{
  api::read::{
    GetVariable, GetVariableResponse, ListVariables,
    ListVariablesResponse,
  },
  entities::user::User,
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::{
  config::core_config,
  helpers::query::get_variable,
  state::{db_client, State},
};

impl Resolve<GetVariable, User> for State {
  async fn resolve(
    &self,
    GetVariable { name }: GetVariable,
    _: User,
  ) -> anyhow::Result<GetVariableResponse> {
    get_variable(&name).await
  }
}

impl Resolve<ListVariables, User> for State {
  async fn resolve(
    &self,
    ListVariables {}: ListVariables,
    _: User,
  ) -> anyhow::Result<ListVariablesResponse> {
    let variables =
      find_collect(&db_client().await.variables, None, None)
        .await
        .context("failed to query db for variables")?;
    Ok(ListVariablesResponse {
      variables,
      secrets: core_config().secrets.keys().cloned().collect(),
    })
  }
}
