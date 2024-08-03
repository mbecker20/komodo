use anyhow::Context;
use formatting::format_serror;
use monitor_client::entities::{to_monitor_name, update::Log};
use periphery_client::api::compose::{
  ComposeDown, ComposePause, ComposeResponse, ComposeRestart,
  ComposeServiceDown, ComposeServicePause, ComposeServiceRestart,
  ComposeServiceStart, ComposeServiceStop, ComposeServiceUnpause,
  ComposeServiceUp, ComposeStart, ComposeStop, ComposeUnpause,
  ComposeUp,
};
use resolver_api::Resolve;
use tokio::fs;

use crate::{
  compose::{maybe_timeout, run_compose_command},
  config::periphery_config,
  helpers::parse_extra_args,
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
  ) -> anyhow::Result<ComposeResponse> {
    let extra_args = parse_extra_args(&stack.config.extra_args);
    Ok(
      run_compose_command(
        &stack,
        None,
        git_token,
        registry_token,
        true,
        "compose up",
        &format!("up -d{extra_args}"),
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
    ComposeStart { stack, git_token }: ComposeStart,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    Ok(
      run_compose_command(
        &stack,
        None,
        git_token,
        None,
        false,
        "compose start",
        "start",
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
    ComposeRestart { stack, git_token }: ComposeRestart,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    Ok(
      run_compose_command(
        &stack,
        None,
        git_token,
        None,
        false,
        "compose restart",
        "restart",
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
    ComposePause { stack, git_token }: ComposePause,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    Ok(
      run_compose_command(
        &stack,
        None,
        git_token,
        None,
        false,
        "compose pause",
        "pause",
      )
      .await,
    )
  }
}

impl Resolve<ComposeUnpause> for State {
  #[instrument(name = "ComposeUnpause", skip_all)]
  async fn resolve(
    &self,
    ComposeUnpause { stack, git_token }: ComposeUnpause,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    Ok(
      run_compose_command(
        &stack,
        None,
        git_token,
        None,
        false,
        "compose unpause",
        "unpause",
      )
      .await,
    )
  }
}

impl Resolve<ComposeStop> for State {
  #[instrument(name = "ComposeStop", skip_all)]
  async fn resolve(
    &self,
    ComposeStop {
      stack,
      git_token,
      timeout,
    }: ComposeStop,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    let maybe_timeout = maybe_timeout(timeout);
    Ok(
      run_compose_command(
        &stack,
        None,
        git_token,
        None,
        false,
        "compose stop",
        &format!("stop{maybe_timeout}"),
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
      stack,
      git_token,
      remove_orphans,
      timeout,
    }: ComposeDown,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    let maybe_timeout = maybe_timeout(timeout);
    let maybe_remove_orphans = if remove_orphans {
      " --remove-orphans"
    } else {
      ""
    };
    let mut res = run_compose_command(
      &stack,
      None,
      git_token,
      None,
      false,
      "compose stop",
      &format!("down{maybe_timeout}{maybe_remove_orphans}"),
    )
    .await;

    let root = periphery_config()
      .stack_dir
      .join(to_monitor_name(&stack.name));

    // delete root
    match fs::remove_dir_all(&root).await.with_context(|| {
      format!("failed to clean up stack directory at {root:?}")
    }) {
      Ok(_) => res.logs.push(Log::simple(
        "cleanup stack directory",
        format!("deleted directory {root:?}"),
      )),
      Err(e) => res.logs.push(Log::error(
        "cleanup stack directory",
        format_serror(&e.into()),
      )),
    };

    Ok(res)
  }
}

impl Resolve<ComposeServiceUp> for State {
  #[instrument(name = "ComposeServiceUp", skip_all)]
  async fn resolve(
    &self,
    ComposeServiceUp {
      stack,
      service,
      git_token,
      registry_token,
    }: ComposeServiceUp,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    let extra_args = parse_extra_args(&stack.config.extra_args);
    Ok(
      run_compose_command(
        &stack,
        Some(&service),
        git_token,
        registry_token,
        true,
        "compose up",
        &format!("up -d{extra_args} {service}"),
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
    ComposeServiceStart {
      stack,
      service,
      git_token,
    }: ComposeServiceStart,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    Ok(
      run_compose_command(
        &stack,
        Some(&service),
        git_token,
        None,
        false,
        "compose start",
        &format!("start {service}"),
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
    ComposeServiceRestart {
      stack,
      service,
      git_token,
    }: ComposeServiceRestart,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    Ok(
      run_compose_command(
        &stack,
        Some(&service),
        git_token,
        None,
        false,
        "compose restart",
        &format!("restart {service}"),
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
    ComposeServicePause {
      stack,
      service,
      git_token,
    }: ComposeServicePause,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    Ok(
      run_compose_command(
        &stack,
        Some(&service),
        git_token,
        None,
        false,
        "compose pause",
        &format!("pause {service}"),
      )
      .await,
    )
  }
}

impl Resolve<ComposeServiceUnpause> for State {
  #[instrument(name = "ComposeServiceUnpause", skip_all)]
  async fn resolve(
    &self,
    ComposeServiceUnpause {
      stack,
      service,
      git_token,
    }: ComposeServiceUnpause,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    Ok(
      run_compose_command(
        &stack,
        Some(&service),
        git_token,
        None,
        false,
        "compose unpause",
        &format!("unpause {service}"),
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
      stack,
      service,
      git_token,
      timeout,
    }: ComposeServiceStop,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    let maybe_timeout = maybe_timeout(timeout);
    Ok(
      run_compose_command(
        &stack,
        Some(&service),
        git_token,
        None,
        false,
        "compose stop",
        &format!("stop{maybe_timeout} {service}"),
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
      stack,
      service,
      git_token,
      remove_orphans,
      timeout,
    }: ComposeServiceDown,
    _: (),
  ) -> anyhow::Result<ComposeResponse> {
    let maybe_timeout = maybe_timeout(timeout);
    let maybe_remove_orphans = if remove_orphans {
      " --remove-orphans"
    } else {
      ""
    };
    Ok(
      run_compose_command(
        &stack,
        Some(&service),
        git_token,
        None,
        false,
        "compose stop",
        &format!(
          "down{maybe_timeout}{maybe_remove_orphans} {service}"
        ),
      )
      .await,
    )
  }
}
