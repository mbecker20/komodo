use std::time::{Duration, Instant};

use anyhow::{anyhow, Context};
use formatting::{bold, colored, format_serror, muted, Color};
use futures::future::join_all;
use komodo_client::{
  api::execute::Execution,
  entities::{
    procedure::Procedure,
    update::{Log, Update},
    user::procedure_user,
  },
};
use mungos::by_id::find_one_by_id;
use resolver_api::Resolve;
use tokio::sync::Mutex;

use crate::{
  api::execute::ExecuteRequest,
  state::{db_client, State},
};

use super::update::{init_execution_update, update_update};

#[instrument(skip_all)]
pub async fn execute_procedure(
  procedure: &Procedure,
  update: &Mutex<Update>,
) -> anyhow::Result<()> {
  for stage in &procedure.config.stages {
    if !stage.enabled {
      continue;
    }
    add_line_to_update(
      update,
      &format!(
        "{}: Executing stage: '{}'",
        muted("INFO"),
        bold(&stage.name)
      ),
    )
    .await;
    let timer = Instant::now();
    execute_stage(
      stage
        .executions
        .iter()
        .filter(|item| item.enabled)
        .map(|item| item.execution.clone())
        .collect(),
      &procedure.id,
      &procedure.name,
      update,
    )
    .await
    .with_context(|| {
      format!(
        "Failed stage '{}' execution after {:?}",
        bold(&stage.name),
        timer.elapsed(),
      )
    })?;
    add_line_to_update(
      update,
      &format!(
        "{}: {} stage '{}' execution in {:?}",
        muted("INFO"),
        colored("Finished", Color::Green),
        bold(&stage.name),
        timer.elapsed()
      ),
    )
    .await;
  }

  Ok(())
}

#[allow(dependency_on_unit_never_type_fallback)]
#[instrument(skip(update))]
async fn execute_stage(
  executions: Vec<Execution>,
  parent_id: &str,
  parent_name: &str,
  update: &Mutex<Update>,
) -> anyhow::Result<()> {
  let futures = executions.into_iter().map(|execution| async move {
    let now = Instant::now();
    add_line_to_update(
      update,
      &format!("{}: Executing: {execution:?}", muted("INFO")),
    )
    .await;
    let fail_log = format!(
      "{}: Failed on {execution:?}",
      colored("ERROR", Color::Red)
    );
    let res =
      execute_execution(execution.clone(), parent_id, parent_name)
        .await
        .context(fail_log);
    add_line_to_update(
      update,
      &format!(
        "{}: {} execution in {:?}: {execution:?}",
        muted("INFO"),
        colored("Finished", Color::Green),
        now.elapsed()
      ),
    )
    .await;
    res
  });
  join_all(futures)
    .await
    .into_iter()
    .collect::<anyhow::Result<_>>()?;
  Ok(())
}

async fn execute_execution(
  execution: Execution,
  // used to prevent recursive procedure
  parent_id: &str,
  parent_name: &str,
) -> anyhow::Result<()> {
  let user = procedure_user().to_owned();
  let update = match execution {
    Execution::None(_) => return Ok(()),
    Execution::RunProcedure(req) => {
      if req.procedure == parent_id || req.procedure == parent_name {
        return Err(anyhow!("Self referential procedure detected"));
      }
      let req = ExecuteRequest::RunProcedure(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RunProcedure(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at RunProcedure"),
        &update_id,
      )
      .await?
    }
    Execution::RunBuild(req) => {
      let req = ExecuteRequest::RunBuild(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RunBuild(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at RunBuild"),
        &update_id,
      )
      .await?
    }
    Execution::CancelBuild(req) => {
      let req = ExecuteRequest::CancelBuild(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::CancelBuild(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at CancelBuild"),
        &update_id,
      )
      .await?
    }
    Execution::Deploy(req) => {
      let req = ExecuteRequest::Deploy(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::Deploy(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at Deploy"),
        &update_id,
      )
      .await?
    }
    Execution::StartDeployment(req) => {
      let req = ExecuteRequest::StartDeployment(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StartDeployment(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at StartDeployment"),
        &update_id,
      )
      .await?
    }
    Execution::RestartDeployment(req) => {
      let req = ExecuteRequest::RestartDeployment(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RestartDeployment(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at RestartDeployment"),
        &update_id,
      )
      .await?
    }
    Execution::PauseDeployment(req) => {
      let req = ExecuteRequest::PauseDeployment(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PauseDeployment(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PauseDeployment"),
        &update_id,
      )
      .await?
    }
    Execution::UnpauseDeployment(req) => {
      let req = ExecuteRequest::UnpauseDeployment(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::UnpauseDeployment(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at UnpauseDeployment"),
        &update_id,
      )
      .await?
    }
    Execution::StopDeployment(req) => {
      let req = ExecuteRequest::StopDeployment(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StopDeployment(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at StopDeployment"),
        &update_id,
      )
      .await?
    }
    Execution::DestroyDeployment(req) => {
      let req = ExecuteRequest::DestroyDeployment(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::DestroyDeployment(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at RemoveDeployment"),
        &update_id,
      )
      .await?
    }
    Execution::CloneRepo(req) => {
      let req = ExecuteRequest::CloneRepo(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::CloneRepo(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at CloneRepo"),
        &update_id,
      )
      .await?
    }
    Execution::PullRepo(req) => {
      let req = ExecuteRequest::PullRepo(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PullRepo(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PullRepo"),
        &update_id,
      )
      .await?
    }
    Execution::BuildRepo(req) => {
      let req = ExecuteRequest::BuildRepo(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::BuildRepo(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at BuildRepo"),
        &update_id,
      )
      .await?
    }
    Execution::CancelRepoBuild(req) => {
      let req = ExecuteRequest::CancelRepoBuild(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::CancelRepoBuild(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at CancelRepoBuild"),
        &update_id,
      )
      .await?
    }
    Execution::StartContainer(req) => {
      let req = ExecuteRequest::StartContainer(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StartContainer(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at StartContainer"),
        &update_id,
      )
      .await?
    }
    Execution::RestartContainer(req) => {
      let req = ExecuteRequest::RestartContainer(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RestartContainer(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at RestartContainer"),
        &update_id,
      )
      .await?
    }
    Execution::PauseContainer(req) => {
      let req = ExecuteRequest::PauseContainer(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PauseContainer(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PauseContainer"),
        &update_id,
      )
      .await?
    }
    Execution::UnpauseContainer(req) => {
      let req = ExecuteRequest::UnpauseContainer(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::UnpauseContainer(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at UnpauseContainer"),
        &update_id,
      )
      .await?
    }
    Execution::StopContainer(req) => {
      let req = ExecuteRequest::StopContainer(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StopContainer(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at StopContainer"),
        &update_id,
      )
      .await?
    }
    Execution::DestroyContainer(req) => {
      let req = ExecuteRequest::DestroyContainer(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::DestroyContainer(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at RemoveContainer"),
        &update_id,
      )
      .await?
    }
    Execution::StartAllContainers(req) => {
      let req = ExecuteRequest::StartAllContainers(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StartAllContainers(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at StartAllContainers"),
        &update_id,
      )
      .await?
    }
    Execution::RestartAllContainers(req) => {
      let req = ExecuteRequest::RestartAllContainers(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RestartAllContainers(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at RestartAllContainers"),
        &update_id,
      )
      .await?
    }
    Execution::PauseAllContainers(req) => {
      let req = ExecuteRequest::PauseAllContainers(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PauseAllContainers(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PauseAllContainers"),
        &update_id,
      )
      .await?
    }
    Execution::UnpauseAllContainers(req) => {
      let req = ExecuteRequest::UnpauseAllContainers(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::UnpauseAllContainers(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at UnpauseAllContainers"),
        &update_id,
      )
      .await?
    }
    Execution::StopAllContainers(req) => {
      let req = ExecuteRequest::StopAllContainers(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StopAllContainers(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at StopAllContainers"),
        &update_id,
      )
      .await?
    }
    Execution::PruneContainers(req) => {
      let req = ExecuteRequest::PruneContainers(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PruneContainers(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PruneContainers"),
        &update_id,
      )
      .await?
    }
    Execution::DeleteNetwork(req) => {
      let req = ExecuteRequest::DeleteNetwork(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::DeleteNetwork(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at DeleteNetwork"),
        &update_id,
      )
      .await?
    }
    Execution::PruneNetworks(req) => {
      let req = ExecuteRequest::PruneNetworks(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PruneNetworks(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PruneNetworks"),
        &update_id,
      )
      .await?
    }
    Execution::DeleteImage(req) => {
      let req = ExecuteRequest::DeleteImage(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::DeleteImage(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at DeleteImage"),
        &update_id,
      )
      .await?
    }
    Execution::PruneImages(req) => {
      let req = ExecuteRequest::PruneImages(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PruneImages(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PruneImages"),
        &update_id,
      )
      .await?
    }
    Execution::DeleteVolume(req) => {
      let req = ExecuteRequest::DeleteVolume(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::DeleteVolume(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at DeleteVolume"),
        &update_id,
      )
      .await?
    }
    Execution::PruneVolumes(req) => {
      let req = ExecuteRequest::PruneVolumes(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PruneVolumes(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PruneVolumes"),
        &update_id,
      )
      .await?
    }
    Execution::PruneDockerBuilders(req) => {
      let req = ExecuteRequest::PruneDockerBuilders(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PruneDockerBuilders(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PruneDockerBuilders"),
        &update_id,
      )
      .await?
    }
    Execution::PruneBuildx(req) => {
      let req = ExecuteRequest::PruneBuildx(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PruneBuildx(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PruneBuildx"),
        &update_id,
      )
      .await?
    }
    Execution::PruneSystem(req) => {
      let req = ExecuteRequest::PruneSystem(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PruneSystem(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PruneSystem"),
        &update_id,
      )
      .await?
    }
    Execution::RunSync(req) => {
      let req = ExecuteRequest::RunSync(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RunSync(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at RunSync"),
        &update_id,
      )
      .await?
    }
    Execution::DeployStack(req) => {
      let req = ExecuteRequest::DeployStack(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::DeployStack(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at DeployStack"),
        &update_id,
      )
      .await?
    }
    Execution::StartStack(req) => {
      let req = ExecuteRequest::StartStack(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StartStack(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at StartStack"),
        &update_id,
      )
      .await?
    }
    Execution::RestartStack(req) => {
      let req = ExecuteRequest::RestartStack(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RestartStack(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at RestartStack"),
        &update_id,
      )
      .await?
    }
    Execution::PauseStack(req) => {
      let req = ExecuteRequest::PauseStack(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PauseStack(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at PauseStack"),
        &update_id,
      )
      .await?
    }
    Execution::UnpauseStack(req) => {
      let req = ExecuteRequest::UnpauseStack(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::UnpauseStack(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at UnpauseStack"),
        &update_id,
      )
      .await?
    }
    Execution::StopStack(req) => {
      let req = ExecuteRequest::StopStack(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StopStack(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at StopStack"),
        &update_id,
      )
      .await?
    }
    Execution::DestroyStack(req) => {
      let req = ExecuteRequest::DestroyStack(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::DestroyStack(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("Failed at DestroyStack"),
        &update_id,
      )
      .await?
    }
    Execution::Sleep(req) => {
      let duration = Duration::from_millis(req.duration_ms as u64);
      tokio::time::sleep(duration).await;
      Update {
        success: true,
        ..Default::default()
      }
    }
  };
  if update.success {
    Ok(())
  } else {
    Err(anyhow!(
      "{}: execution not successful. see update '{}'",
      colored("ERROR", Color::Red),
      bold(&update.id),
    ))
  }
}

/// If the call to .resolve returns Err, the update may not be closed.
/// This will ensure it is closed with error log attached.
async fn handle_resolve_result(
  res: anyhow::Result<Update>,
  update_id: &str,
) -> anyhow::Result<Update> {
  match res {
    Ok(res) => Ok(res),
    Err(e) => {
      let log =
        Log::error("execution error", format_serror(&e.into()));
      let mut update =
        find_one_by_id(&db_client().updates, update_id)
          .await
          .context("Failed to query to db")?
          .context("no update exists with given id")?;
      update.logs.push(log);
      update.finalize();
      update_update(update.clone()).await?;
      Ok(update)
    }
  }
}

/// ASSUMES FIRST LOG IS ALREADY CREATED
#[instrument(level = "debug")]
async fn add_line_to_update(update: &Mutex<Update>, line: &str) {
  let mut lock = update.lock().await;
  let log = &mut lock.logs[0];
  log.stdout.push('\n');
  log.stdout.push_str(line);
  let update = lock.clone();
  drop(lock);
  if let Err(e) = update_update(update).await {
    error!("Failed to update an update during procedure | {e:#}");
  };
}
