use monitor_client::{
  api::execute::*,
  entities::{update::Update, user::User},
};
use periphery_client::api::compose::*;
use resolver_api::Resolve;

use crate::{
  helpers::{
    periphery_client,
    stack::{
      deploy::deploy_stack_maybe_service, setup_stack_execution,
    },
    update::update_update,
  },
  monitor::update_cache_for_server,
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
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        state.starting = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeStart { file })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<RestartStack, (User, Update)> for State {
  #[instrument(name = "RestartStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RestartStack { stack }: RestartStack,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        state.restarting = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeRestart { file })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<PauseStack, (User, Update)> for State {
  #[instrument(name = "PauseStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PauseStack { stack }: PauseStack,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        state.pausing = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposePause { file })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<UnpauseStack, (User, Update)> for State {
  #[instrument(name = "UnpauseStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    UnpauseStack { stack }: UnpauseStack,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        state.unpausing = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeUnpause { file })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<StopStack, (User, Update)> for State {
  #[instrument(name = "StopStack", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StopStack { stack, stop_time }: StopStack,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        state.stopping = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeStop {
        file,
        timeout: stop_time,
      })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
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
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        state.destroying = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeDown {
        file,
        remove_orphans,
        timeout: stop_time,
      })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
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
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        // state.starting = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeServiceStart { file, service })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<RestartStackService, (User, Update)> for State {
  #[instrument(name = "RestartStackService", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RestartStackService { stack, service }: RestartStackService,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        state.restarting = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeServiceRestart { file, service })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<PauseStackService, (User, Update)> for State {
  #[instrument(name = "PauseStackService", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PauseStackService { stack, service }: PauseStackService,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        // TODO handle service level state cache
        // state.pausing = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeServicePause { file, service })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<UnpauseStackService, (User, Update)> for State {
  #[instrument(name = "UnpauseStackService", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    UnpauseStackService { stack, service }: UnpauseStackService,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        // state.unpausing = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeServiceUnpause { file, service })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
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
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        // state.stopping = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeServiceStop {
        file,
        service,
        timeout: stop_time,
      })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
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
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (server, file) = setup_stack_execution(
      &stack,
      &user,
      |state| {
        state.destroying = true;
      },
      &mut update,
    )
    .await?;

    let logs = periphery_client(&server)?
      .request(ComposeServiceDown {
        file,
        service,
        remove_orphans,
        timeout: stop_time,
      })
      .await?;

    update.logs.extend(logs);

    // Ensure cached stack state up to date by updating server cache
    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}
