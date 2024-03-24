use std::{
  collections::HashMap, str::FromStr, sync::Arc, time::Duration,
};

use anyhow::{anyhow, Context, Ok};
use async_recursion::async_recursion;
use futures::future::join_all;
use monitor_client::{
  api::execute::Execution,
  entities::{
    monitor_timestamp,
    procedure::{EnabledId, Procedure, ProcedureConfig},
    update::Update,
  },
};
use mungos::{
  find::find_collect,
  mongodb::bson::{doc, oid::ObjectId},
};
use resolver_api::Resolve;
use tokio::sync::Mutex;

use crate::{auth::InnerRequestUser, db::db_client, state::State};

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
    error!("failed to update an update during procedure | {e:#?}");
  };
}

#[async_recursion]
pub async fn execute_procedure(
  procedure: &Procedure,
  map: &HashMap<String, Procedure>,
  update: &Mutex<Update>,
) -> anyhow::Result<()> {
  let start_ts = monitor_timestamp();

  use ProcedureConfig::*;
  match &procedure.config {
    Execution(execution) => {
      add_line_to_update(
        update,
        &format!("executing: {} ({})", procedure.name, procedure.id),
      )
      .await;
      execute_execution(execution.to_owned()).await.with_context(
        || {
          let time = Duration::from_millis(
            (monitor_timestamp() - start_ts) as u64,
          );
          format!(
            "failed execution after {time:?}. {} ({})",
            procedure.name, procedure.id
          )
        },
      )?;
      let time = Duration::from_millis(
        (monitor_timestamp() - start_ts) as u64,
      );
      add_line_to_update(
        update,
        &format!(
          "finished execution in {time:?}: {} ({}) ✅",
          procedure.name, procedure.id
        ),
      )
      .await;
      Ok(())
    }
    Sequence(ids) => {
      add_line_to_update(
        update,
        &format!(
          "executing sequence: {} ({})",
          procedure.name, procedure.id
        ),
      )
      .await;
      execute_sequence(&filter_list_by_enabled(ids), map, update)
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
    Parallel(ids) => {
      add_line_to_update(
        update,
        &format!(
          "executing parallel: {} ({})",
          procedure.name, procedure.id
        ),
      )
      .await;
      execute_parallel(&filter_list_by_enabled(ids), map, update)
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
) -> anyhow::Result<()> {
  let user: Arc<_> = InnerRequestUser::procedure().into();
  let update =
    match execution {
      Execution::None(_) => return Ok(()),
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
      Execution::StopContainer(req) => State
        .resolve(req, user)
        .await
        .context("failed at StopContainer")?,
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
      Execution::RunProcedure(req) => State
        .resolve(req, user)
        .await
        .context("failed at RunProcedure")?,
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
  ids: &[String],
  map: &HashMap<String, Procedure>,
  update: &Mutex<Update>,
) -> anyhow::Result<()> {
  for id in ids {
    let procedure = map
      .get(id)
      .with_context(|| format!("no procedure on map with id {id}"))?;
    execute_procedure(procedure, map, update).await?;
  }
  Ok(())
}

async fn execute_parallel(
  ids: &[String],
  map: &HashMap<String, Procedure>,
  update: &Mutex<Update>,
) -> anyhow::Result<()> {
  let futures = ids.iter().map(|id| async {
    let procedure = map.get(id).context("no procedure on map")?;
    execute_procedure(procedure, map, update).await
  });
  join_all(futures)
    .await
    .into_iter()
    .collect::<anyhow::Result<_>>()?;
  Ok(())
}

pub async fn make_procedure_map(
  procedure: &Procedure,
) -> anyhow::Result<HashMap<String, Procedure>> {
  let map = Default::default();
  make_procedure_map_rec(procedure, &map).await?;
  Ok(map.into_inner())
}

#[async_recursion]
async fn make_procedure_map_rec(
  procedure: &Procedure,
  map: &Mutex<HashMap<String, Procedure>>,
) -> anyhow::Result<()> {
  use ProcedureConfig::*;
  let more = match &procedure.config {
    Execution(_) => return Ok(()),
    Sequence(sequence) => sequence,
    Parallel(parallel) => parallel,
  };

  let more_ids = more
    .iter()
    .filter(|id| id.enabled)
    .map(|id| {
      ObjectId::from_str(&id.id).context("id is not ObjectId")
    })
    .collect::<anyhow::Result<Vec<_>>>()?;

  let procedures = find_collect(
    &db_client().procedures,
    doc! { "_id": { "$in": &more_ids } },
    None,
  )
  .await
  .context("failed to find procedures from db")?
  .into_iter()
  .map(|proc| (proc.id.clone(), proc))
  .collect::<HashMap<_, _>>();

  // make sure we aren't missing any procedures
  for EnabledId { id, enabled } in more {
    if !enabled {
      continue;
    }
    if !procedures.contains_key(id) {
      return Err(anyhow!("did not find a procedure with id {id}",));
    }
  }

  let futures = procedures.values().map(|procedure| async {
    make_procedure_map_rec(procedure, map).await
  });

  join_all(futures)
    .await
    .into_iter()
    .collect::<anyhow::Result<Vec<_>>>()?;

  map.lock().await.extend(procedures);

  Ok(())
}

fn filter_list_by_enabled(list: &[EnabledId]) -> Vec<String> {
  list
    .iter()
    .filter(|item| item.enabled)
    .map(|item| item.id.clone())
    .collect()
}
