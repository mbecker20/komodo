use monitor_client::entities::update::Log;
use periphery_client::api::compose::{
  ComposeDown, ComposePause, ComposeRestart, ComposeServiceDown,
  ComposeServicePause, ComposeServiceRestart, ComposeServiceStart,
  ComposeServiceStop, ComposeServiceUnpause, ComposeServiceUp,
  ComposeStart, ComposeStop, ComposeUnpause, ComposeUp,
  ComposeUpResponse,
};
use resolver_api::Resolve;

use crate::{
  compose::{docker_compose, maybe_timeout},
  helpers::run_stack_command,
  State,
};

impl Resolve<ComposeUp> for State {
  #[instrument(name = "ComposeUp", skip_all)]
  async fn resolve(
    &self,
    ComposeUp {
      stack,
      git_token,
      registry_token,
    }: ComposeUp,
    _: (),
  ) -> anyhow::Result<ComposeUpResponse> {
    crate::compose::compose_up(stack, git_token, registry_token, None)
      .await
  }
}

//

impl Resolve<ComposeStart> for State {
  #[instrument(name = "ComposeStart", skip_all)]
  async fn resolve(
    &self,
    ComposeStart { file }: ComposeStart,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    Ok(
      run_stack_command(
        &file,
        "compose start",
        format!("{docker_compose} start"),
      )
      .await,
    )
  }
}

//

impl Resolve<ComposeRestart> for State {
  #[instrument(name = "ComposeRestart", skip_all)]
  async fn resolve(
    &self,
    ComposeRestart { file }: ComposeRestart,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    Ok(
      run_stack_command(
        &file,
        "compose restart",
        format!("{docker_compose} restart"),
      )
      .await,
    )
  }
}

//

impl Resolve<ComposePause> for State {
  #[instrument(name = "ComposePause", skip_all)]
  async fn resolve(
    &self,
    ComposePause { file }: ComposePause,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    Ok(
      run_stack_command(
        &file,
        "compose pause",
        format!("{docker_compose} pause"),
      )
      .await,
    )
  }
}

impl Resolve<ComposeUnpause> for State {
  #[instrument(name = "ComposeUnpause", skip_all)]
  async fn resolve(
    &self,
    ComposeUnpause { file }: ComposeUnpause,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    Ok(
      run_stack_command(
        &file,
        "compose unpause",
        format!("{docker_compose} unpause"),
      )
      .await,
    )
  }
}

impl Resolve<ComposeStop> for State {
  #[instrument(name = "ComposeStop", skip_all)]
  async fn resolve(
    &self,
    ComposeStop { file, timeout }: ComposeStop,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    let maybe_timeout = maybe_timeout(timeout);
    Ok(
      run_stack_command(
        &file,
        "compose stop",
        format!("{docker_compose} stop{maybe_timeout}"),
      )
      .await,
    )
  }
}

impl Resolve<ComposeDown> for State {
  #[instrument(name = "ComposeDown", skip_all)]
  async fn resolve(
    &self,
    ComposeDown {
      file,
      remove_orphans,
      timeout,
    }: ComposeDown,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    let maybe_timeout = maybe_timeout(timeout);
    let maybe_remove_orphans = if remove_orphans {
      " --remove-orphans"
    } else {
      ""
    };
    Ok(
      run_stack_command(
        &file,
        "compose down",
        format!("{docker_compose} down{maybe_timeout}{maybe_remove_orphans}"),
      )
      .await,
    )
  }
}

impl Resolve<ComposeServiceUp> for State {
  #[instrument(name = "ComposeServiceUp", skip_all)]
  async fn resolve(
    &self,
    ComposeServiceUp {
      stack,
      git_token,
      registry_token,
      service,
    }: ComposeServiceUp,
    _: (),
  ) -> anyhow::Result<ComposeUpResponse> {
    crate::compose::compose_up(
      stack,
      git_token,
      registry_token,
      Some(&service),
    )
    .await
  }
}

//

impl Resolve<ComposeServiceStart> for State {
  #[instrument(name = "ComposeServiceStart", skip_all)]
  async fn resolve(
    &self,
    ComposeServiceStart { file, service }: ComposeServiceStart,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    Ok(
      run_stack_command(
        &file,
        "compose start",
        format!("{docker_compose} start {service}"),
      )
      .await,
    )
  }
}

//

impl Resolve<ComposeServiceRestart> for State {
  #[instrument(name = "ComposeServiceRestart", skip_all)]
  async fn resolve(
    &self,
    ComposeServiceRestart { file, service }: ComposeServiceRestart,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    Ok(
      run_stack_command(
        &file,
        "compose restart",
        format!("{docker_compose} restart {service}"),
      )
      .await,
    )
  }
}

//

impl Resolve<ComposeServicePause> for State {
  #[instrument(name = "ComposeServicePause", skip_all)]
  async fn resolve(
    &self,
    ComposeServicePause { file, service }: ComposeServicePause,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    Ok(
      run_stack_command(
        &file,
        "compose pause",
        format!("{docker_compose} pause {service}"),
      )
      .await,
    )
  }
}

impl Resolve<ComposeServiceUnpause> for State {
  #[instrument(name = "ComposeServiceUnpause", skip_all)]
  async fn resolve(
    &self,
    ComposeServiceUnpause { file, service }: ComposeServiceUnpause,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    Ok(
      run_stack_command(
        &file,
        "compose pause",
        format!("{docker_compose} unpause {service}"),
      )
      .await,
    )
  }
}

impl Resolve<ComposeServiceStop> for State {
  #[instrument(name = "ComposeServiceStop", skip_all)]
  async fn resolve(
    &self,
    ComposeServiceStop {
      file,
      service,
      timeout,
    }: ComposeServiceStop,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    let maybe_timeout = maybe_timeout(timeout);
    Ok(
      run_stack_command(
        &file,
        "compose stop",
        format!("{docker_compose} stop{maybe_timeout} {service}"),
      )
      .await,
    )
  }
}

impl Resolve<ComposeServiceDown> for State {
  #[instrument(name = "ComposeServiceDown", skip_all)]
  async fn resolve(
    &self,
    ComposeServiceDown {
      file,
      service,
      remove_orphans,
      timeout,
    }: ComposeServiceDown,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let docker_compose = docker_compose();
    let maybe_timeout = maybe_timeout(timeout);
    let maybe_remove_orphans = if remove_orphans {
      " --remove-orphans"
    } else {
      ""
    };
    Ok(
      run_stack_command(
        &file,
        "compose down",
        format!("{docker_compose} down{maybe_timeout}{maybe_remove_orphans} {service}"),
      )
      .await,
    )
  }
}
