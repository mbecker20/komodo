use anyhow::Context;
use mongo_indexed::doc;
use monitor_client::{
  api::read::{
    GetVariable, GetVariableResponse, ListVariables,
    ListVariablesResponse,
  },
  entities::user::User,
};
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
    find_collect(
      &db_client().await.variables,
      None,
      FindOptions::builder().sort(doc! { "name": 1 }).build(),
    )
    .await
    .context("failed to query db for variables")
  }
}
