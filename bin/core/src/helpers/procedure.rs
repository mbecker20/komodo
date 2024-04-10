use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Ok};
use futures::future::join_all;
use monitor_client::{
  api::execute::Execution,
  entities::{
    monitor_timestamp,
    procedure::{EnabledExecution, Procedure, ProcedureConfig},
    update::Update,
    user::procedure_user,
  },
};
use resolver_api::Resolve;
use tokio::sync::Mutex;

use crate::state::State;

use super::update_update;

/// ASSUMES FIRST LOG IS ALREADY CREATED
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

pub async fn execute_procedure(
  procedure: &Procedure,
  update: &Mutex<Update>,
) -> anyhow::Result<()> {
  let start_ts = monitor_timestamp();

  use ProcedureConfig::*;
  match &procedure.config {
    Sequence(executions) => {
      add_line_to_update(
        update,
        &format!(
          "executing sequence: {} ({})",
          procedure.name, procedure.id
        ),
      )
      .await;
      execute_sequence(
        filter_list_by_enabled(executions),
        &procedure.id,
        &procedure.name,
        update,
      )
      .await
      .with_context(|| {
        let time = Duration::from_millis(
          (monitor_timestamp() - start_ts) as u64,
        );
        format!(
          "failed sequence execution after {time:?}. {} ({})",
          procedure.name, procedure.id
        )
      })?;
      let time = Duration::from_millis(
        (monitor_timestamp() - start_ts) as u64,
      );
      add_line_to_update(
        update,
        &format!(
          "finished sequence execution in {time:?}: {} ({}) ✅",
          procedure.name, procedure.id
        ),
      )
      .await;
      Ok(())
    }
    Parallel(executions) => {
      add_line_to_update(
        update,
        &format!(
          "executing parallel: {} ({})",
          procedure.name, procedure.id
        ),
      )
      .await;
      execute_parallel(
        filter_list_by_enabled(executions),
        &procedure.id,
        &procedure.name,
        update,
      )
      .await
      .with_context(|| {
        let time = Duration::from_millis(
          (monitor_timestamp() - start_ts) as u64,
        );
        format!(
          "failed parallel execution after {time:?}. {} ({})",
          procedure.name, procedure.id
        )
      })?;
      let time = Duration::from_millis(
        (monitor_timestamp() - start_ts) as u64,
      );
      add_line_to_update(
        update,
        &format!(
          "finished parallel execution in {time:?}: {} ({}) ✅",
          procedure.name, procedure.id
        ),
      )
      .await;
      Ok(())
    }
  }
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
      State
        .resolve(req, user)
        .await
        .context("failed at RunProcedure")?
    }
    Execution::RunBuild(req) => State
      .resolve(req, user)
      .await
      .context("failed at RunBuild")?,
    Execution::Deploy(req) => {
      State.resolve(req, user).await.context("failed at Deploy")?
    }
    Execution::StartContainer(req) => State
      .resolve(req, user)
      .await
      .context("failed at StartContainer")?,
    Execution::StopContainer(req) => {
      State
        .resolve(req, user)
        .await
        .context("failed at StopContainer")?
    }
    Execution::StopAllContainers(req) => State
      .resolve(req, user)
      .await
      .context("failed at StopAllContainers")?,
    Execution::RemoveContainer(req) => State
      .resolve(req, user)
      .await
      .context("failed at RemoveContainer")?,
    Execution::CloneRepo(req) => State
      .resolve(req, user)
      .await
      .context("failed at CloneRepo")?,
    Execution::PullRepo(req) => State
      .resolve(req, user)
      .await
      .context("failed at PullRepo")?,
    Execution::PruneDockerNetworks(req) => State
      .resolve(req, user)
      .await
      .context("failed at PruneDockerNetworks")?,
    Execution::PruneDockerImages(req) => State
      .resolve(req, user)
      .await
      .context("failed at PruneDockerImages")?,
    Execution::PruneDockerContainers(req) => State
      .resolve(req, user)
      .await
      .context("failed at PruneDockerContainers")?,
  };
  if update.success {
    Ok(())
  } else {
    Err(anyhow!(
      "execution not successful. see update {}",
      update.id
    ))
  }
}

async fn execute_sequence(
  executions: Vec<Execution>,
  parent_id: &str,
  parent_name: &str,
  update: &Mutex<Update>,
) -> anyhow::Result<()> {
  for execution in executions {
    let now = Instant::now();
    add_line_to_update(
      update,
      &format!("executing stage: {execution:?}"),
    )
    .await;
    let fail_log = format!("failed on {execution:?}");
    execute_execution(execution.clone(), parent_id, parent_name)
      .await
      .context(fail_log)?;
    add_line_to_update(
      update,
      &format!(
        "finished stage in {:?}: {execution:?}",
        now.elapsed()
      ),
    )
    .await;
  }
  Ok(())
}

async fn execute_parallel(
  executions: Vec<Execution>,
  parent_id: &str,
  parent_name: &str,
  update: &Mutex<Update>,
) -> anyhow::Result<()> {
  let futures = executions.into_iter().map(|execution| async move {
    let now = Instant::now();
    add_line_to_update(
      update,
      &format!("executing stage: {execution:?}"),
    )
    .await;
    let fail_log = format!("failed on {execution:?}");
    let res =
      execute_execution(execution.clone(), parent_id, parent_name)
        .await
        .context(fail_log);
    add_line_to_update(
      update,
      &format!(
        "finished stage in {:?}: {execution:?}",
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

fn filter_list_by_enabled(
  list: &[EnabledExecution],
) -> Vec<Execution> {
  list
    .iter()
    .filter(|item| item.enabled)
    .map(|item| item.execution.clone())
    .collect()
}
