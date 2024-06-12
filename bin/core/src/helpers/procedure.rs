use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Ok};
use futures::future::join_all;
use monitor_client::{
  api::execute::Execution,
  entities::{
    procedure::Procedure, update::Update, user::procedure_user,
  },
};
use resolver_api::Resolve;
use tokio::sync::Mutex;

use crate::{api::execute::ExecuteRequest, state::State};

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
      &format!("executing stage: {}", stage.name),
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
        stage.name,
        timer.elapsed(),
      )
    })?;
    add_line_to_update(
      update,
      &format!(
        "finished stage '{}' execution in {:?} ✅",
        stage.name,
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
    add_line_to_update(update, &format!("executing: {execution:?}"))
      .await;
    let fail_log = format!("failed on {execution:?}");
    let res =
      execute_execution(execution.clone(), parent_id, parent_name)
        .await
        .context(fail_log);
    add_line_to_update(
      update,
      &format!(
        "finished execution in {:?}: {execution:?}",
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
      State
        .resolve(req, (user, update))
        .await
        .context("failed at RunProcedure")?
    }
    Execution::RunBuild(req) => {
      let req = ExecuteRequest::RunBuild(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RunBuild(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at RunBuild")?
    }
    Execution::Deploy(req) => {
      let req = ExecuteRequest::Deploy(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::Deploy(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at Deploy")?
    }
    Execution::StartContainer(req) => {
      let req = ExecuteRequest::StartContainer(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StartContainer(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at StartContainer")?
    }
    Execution::StopContainer(req) => {
      let req = ExecuteRequest::StopContainer(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StopContainer(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at StopContainer")?
    }
    Execution::StopAllContainers(req) => {
      let req = ExecuteRequest::StopAllContainers(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::StopAllContainers(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at StopAllContainers")?
    }
    Execution::RemoveContainer(req) => {
      let req = ExecuteRequest::RemoveContainer(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RemoveContainer(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at RemoveContainer")?
    }
    Execution::CloneRepo(req) => {
      let req = ExecuteRequest::CloneRepo(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::CloneRepo(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at CloneRepo")?
    }
    Execution::PullRepo(req) => {
      let req = ExecuteRequest::PullRepo(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PullRepo(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at PullRepo")?
    }
    Execution::PruneNetworks(req) => {
      let req = ExecuteRequest::PruneNetworks(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PruneNetworks(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at PruneNetworks")?
    }
    Execution::PruneImages(req) => {
      let req = ExecuteRequest::PruneImages(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PruneImages(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at PruneImages")?
    }
    Execution::PruneContainers(req) => {
      let req = ExecuteRequest::PruneContainers(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::PruneContainers(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at PruneContainers")?
    }
    Execution::RunSync(req) => {
      let req = ExecuteRequest::RunSync(req);
      let update = init_execution_update(&req, &user).await?;
      let ExecuteRequest::RunSync(req) = req else {
        unreachable!()
      };
      State
        .resolve(req, (user, update))
        .await
        .context("failed at RunSync")?
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
      "execution not successful. see update {}",
      update.id
    ))
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
