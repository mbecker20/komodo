use monitor_client::entities::update::Log;
use periphery_client::api::compose::{
  ComposeDown, ComposePause, ComposeRestart, ComposeServiceDown,
  ComposeServicePause, ComposeServiceRestart, ComposeServiceStart,
  ComposeServiceStop, ComposeServiceUp, ComposeStart, ComposeStop,
  ComposeUp,
};
use resolver_api::Resolve;

use crate::{
  config::periphery_config, helpers::run_stack_command, State,
};

impl Resolve<ComposeUp> for State {
  #[instrument(name = "ComposeUp", skip_all)]
  async fn resolve(
    &self,
    ComposeUp { file }: ComposeUp,
    _: (),
  ) -> anyhow::Result<Log> {
    let docker_compose = docker_compose();
    Ok(
      run_stack_command(
        &file,
        "compose up",
        format!("{docker_compose} up -d"),
      )
      .await,
    )
  }
}

//

impl Resolve<ComposeStart> for State {
  #[instrument(name = "ComposeStart", skip_all)]
  async fn resolve(
    &self,
    ComposeStart { file }: ComposeStart,
    _: (),
  ) -> anyhow::Result<Log> {
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
  ) -> anyhow::Result<Log> {
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
  ) -> anyhow::Result<Log> {
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

impl Resolve<ComposeStop> for State {
  #[instrument(name = "ComposeStop", skip_all)]
  async fn resolve(
    &self,
    ComposeStop { file, timeout }: ComposeStop,
    _: (),
  ) -> anyhow::Result<Log> {
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
  ) -> anyhow::Result<Log> {
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
    ComposeServiceUp { file, service }: ComposeServiceUp,
    _: (),
  ) -> anyhow::Result<Log> {
    let docker_compose = docker_compose();
    Ok(
      run_stack_command(
        &file,
        "compose up",
        format!("{docker_compose} up -d {service}"),
      )
      .await,
    )
  }
}

//

impl Resolve<ComposeServiceStart> for State {
  #[instrument(name = "ComposeServiceStart", skip_all)]
  async fn resolve(
    &self,
    ComposeServiceStart { file, service }: ComposeServiceStart,
    _: (),
  ) -> anyhow::Result<Log> {
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
  ) -> anyhow::Result<Log> {
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
  ) -> anyhow::Result<Log> {
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
  ) -> anyhow::Result<Log> {
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
  ) -> anyhow::Result<Log> {
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

//

fn docker_compose() -> &'static str {
  if periphery_config().legacy_compose_cli {
    "docker-compose"
  } else {
    "docker compose"
  }
}

fn maybe_timeout(timeout: Option<i32>) -> String {
  if let Some(timeout) = timeout {
    format!(" --timeout {timeout}")
  } else {
    String::new()
  }
}
