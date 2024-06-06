use monitor_client::{
  api::execute::RunSync,
  entities::{update::Update, user::User},
};
use resolver_api::Resolve;

use crate::state::State;

impl Resolve<RunSync, (User, Update)> for State {
  async fn resolve(
    &self,
    RunSync { sync }: RunSync,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    todo!()
  }
}
