use monitor_client::{
  api::execute::*,
  entities::{update::Update, user::User},
};
use resolver_api::Resolve;

use crate::{
  helpers::stack::{
    deploy::deploy_stack_maybe_service, execute::execute_compose,
  },
  state::State,
};

impl Resolve<DeployStack, (User, Update)> for State {
  #[instrument(name = "DeployStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DeployStack { stack, stop_time }: DeployStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    deploy_stack_maybe_service(&stack, user, update, None).await
  }
}

impl Resolve<StartStack, (User, Update)> for State {
  #[instrument(name = "StartStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StartStack { stack }: StartStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<StartStack>(
      &stack,
      &user,
      |state| state.starting = true,
      update,
      (),
    )
    .await
  }
}

impl Resolve<RestartStack, (User, Update)> for State {
  #[instrument(name = "RestartStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RestartStack { stack }: RestartStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<RestartStack>(
      &stack,
      &user,
      |state| state.restarting = true,
      update,
      (),
    )
    .await
  }
}

impl Resolve<PauseStack, (User, Update)> for State {
  #[instrument(name = "PauseStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PauseStack { stack }: PauseStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<PauseStack>(
      &stack,
      &user,
      |state| state.pausing = true,
      update,
      (),
    )
    .await
  }
}

impl Resolve<UnpauseStack, (User, Update)> for State {
  #[instrument(name = "UnpauseStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    UnpauseStack { stack }: UnpauseStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<UnpauseStack>(
      &stack,
      &user,
      |state| state.unpausing = true,
      update,
      (),
    )
    .await
  }
}

impl Resolve<StopStack, (User, Update)> for State {
  #[instrument(name = "StopStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StopStack { stack, stop_time }: StopStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<StopStack>(
      &stack,
      &user,
      |state| state.stopping = true,
      update,
      stop_time,
    )
    .await
  }
}

impl Resolve<DestroyStack, (User, Update)> for State {
  #[instrument(name = "DestroyStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DestroyStack {
      stack,
      remove_orphans,
      stop_time,
    }: DestroyStack,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<DestroyStack>(
      &stack,
      &user,
      |state| state.destroying = true,
      update,
      (stop_time, remove_orphans),
    )
    .await
  }
}

impl Resolve<DeployStackService, (User, Update)> for State {
  #[instrument(name = "DeployStackService", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DeployStackService {
      stack,
      service,
      stop_time,
    }: DeployStackService,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    deploy_stack_maybe_service(&stack, user, update, Some(service))
      .await
  }
}

impl Resolve<StartStackService, (User, Update)> for State {
  #[instrument(name = "StartStackService", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StartStackService { stack, service }: StartStackService,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<StartStackService>(
      &stack,
      &user,
      |_| {},
      update,
      service,
    )
    .await
  }
}

impl Resolve<RestartStackService, (User, Update)> for State {
  #[instrument(name = "RestartStackService", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RestartStackService { stack, service }: RestartStackService,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<RestartStackService>(
      &stack,
      &user,
      |_| {},
      update,
      service,
    )
    .await
  }
}

impl Resolve<PauseStackService, (User, Update)> for State {
  #[instrument(name = "PauseStackService", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PauseStackService { stack, service }: PauseStackService,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<PauseStackService>(
      &stack,
      &user,
      |_| {},
      update,
      service,
    )
    .await
  }
}

impl Resolve<UnpauseStackService, (User, Update)> for State {
  #[instrument(name = "UnpauseStackService", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    UnpauseStackService { stack, service }: UnpauseStackService,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<UnpauseStackService>(
      &stack,
      &user,
      |_| {},
      update,
      service,
    )
    .await
  }
}

impl Resolve<StopStackService, (User, Update)> for State {
  #[instrument(name = "StopStackService", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StopStackService {
      stack,
      service,
      stop_time,
    }: StopStackService,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<StopStackService>(
      &stack,
      &user,
      |_| {},
      update,
      (service, stop_time),
    )
    .await
  }
}

impl Resolve<DestroyStackService, (User, Update)> for State {
  #[instrument(name = "DestroyStackService", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DestroyStackService {
      stack,
      service,
      remove_orphans,
      stop_time,
    }: DestroyStackService,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    execute_compose::<DestroyStackService>(
      &stack,
      &user,
      |_| {},
      update,
      (service, stop_time, remove_orphans),
    )
    .await
  }
}
