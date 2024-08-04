use std::time::{Duration, Instant};

use anyhow::{anyhow, Context};
use formatting::{bold, colored, format_serror, muted, Color};
use futures::future::join_all;
use monitor_client::{
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
        "{}: executing stage: '{}'",
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
        "failed stage '{}' execution after {:?}",
        bold(&stage.name),
        timer.elapsed(),
      )
    })?;
    add_line_to_update(
      update,
      &format!(
        "{}: {} stage '{}' execution in {:?}",
        muted("INFO"),
        colored("finished", Color::Green),
        bold(&stage.name),
        timer.elapsed()
      ),
    )
    .await;
  }

  Ok(())
}

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
      &format!("{}: executing: {execution:?}", muted("INFO")),
    )
    .await;
    let fail_log = format!(
      "{}: failed on {execution:?}",
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
        colored("finished", Color::Green),
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
          .context("failed at RunProcedure"),
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
          .context("failed at RunBuild"),
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
          .context("failed at Deploy"),
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
          .context("failed at StartContainer"),
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
          .context("failed at StopContainer"),
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
          .context("failed at StopAllContainers"),
        &update_id,
      )
      .await?
    }
    Execution::RemoveContainer(req) => {
      let req = ExecuteRequest::RemoveContainer(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RemoveContainer(req) = req else {
        unreachable!()
      };
      let update_id = update.id.clone();
      handle_resolve_result(
        State
          .resolve(req, (user, update))
          .await
          .context("failed at RemoveContainer"),
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
          .context("failed at CloneRepo"),
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
          .context("failed at PullRepo"),
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
          .context("failed at PruneNetworks"),
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
          .context("failed at PruneImages"),
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
          .context("failed at PruneContainers"),
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
          .context("failed at RunSync"),
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
          .context("failed at DeployStack"),
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
          .context("failed at StartStack"),
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
          .context("failed at RestartStack"),
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
          .context("failed at PauseStack"),
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
          .context("failed at UnpauseStack"),
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
          .context("failed at StopStack"),
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
          .context("failed at DestroyStack"),
        &update_id,
      )
      .await?
    }
    Execution::Sleep(req) => {
      tokio::time::sleep(Duration::from_millis(
        req.duration_ms as u64,
      ))
      .await;
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
        find_one_by_id(&db_client().await.updates, update_id)
          .await
          .context("failed to query to db")?
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
    error!("failed to update an update during procedure | {e:#}");
  };
}
