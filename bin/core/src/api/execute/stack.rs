use monitor_client::{
  api::execute::{DeployStack, DestroyStack},
  entities::{update::Update, user::User},
};
use resolver_api::Resolve;

use crate::state::State;

impl Resolve<DeployStack, (User, Update)> for State {
  #[instrument(name = "DeployStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DeployStack { stack }: DeployStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    todo!()
  }
}

impl Resolve<DestroyStack, (User, Update)> for State {
  #[instrument(name = "DestroyStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DestroyStack { stack }: DestroyStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    todo!()
  }
}
