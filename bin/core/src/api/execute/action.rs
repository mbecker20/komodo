use komodo_client::{
  api::execute::RunAction,
  entities::{update::Update, user::User},
};
use resolver_api::Resolve;

use crate::state::State;

impl Resolve<RunAction, (User, Update)> for State {
  async fn resolve(
    &self,
    RunAction { action }: RunAction,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    Ok(update)
  }
}
