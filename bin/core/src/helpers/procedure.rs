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
    procedure::{Procedure, ProcedureConfig},
    update::Update,
  },
};
use mungos::{
  find::find_collect,
  mongodb::bson::{doc, oid::ObjectId},
};
use resolver_api::Resolve;
use tokio::sync::Mutex;

use crate::{auth::InnerRequestUser, state::State};

impl State {
  /// ASSUMES FIRST LOG IS ALREADY CREATED
  async fn add_line_to_update(
    &self,
    update: &Mutex<Update>,
    line: &str,
  ) {
    let mut lock = update.lock().await;
    let log = &mut lock.logs[0];
    log.stdout.push('\n');
    log.stdout.push_str(line);
    let update = lock.clone();
    drop(lock);
    if let Err(e) = self.update_update(update).await {
      error!("failed to update an update during procedure | {e:#?}");
    };
  }

  #[async_recursion]
  pub async fn execute_procedure(
    &self,
    procedure: &Procedure,
    map: &HashMap<String, Procedure>,
    update: &Mutex<Update>,
  ) -> anyhow::Result<()> {
    let start_ts = monitor_timestamp();

    use ProcedureConfig::*;
    match &procedure.config {
      Execution(execution) => {
        self
          .add_line_to_update(
            update,
            &format!(
              "executing: {} ({})",
              procedure.name, procedure.id
            ),
          )
          .await;
        self
          .execute_execution(execution.to_owned())
          .await
          .with_context(|| {
            let time = Duration::from_millis(
              (monitor_timestamp() - start_ts) as u64,
            );
            format!(
              "failed execution after {time:?}. {} ({})",
              procedure.name, procedure.id
            )
          })?;
        let time = Duration::from_millis(
          (monitor_timestamp() - start_ts) as u64,
        );
        self
          .add_line_to_update(
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
        self
          .add_line_to_update(
            update,
            &format!(
              "executing sequence: {} ({})",
              procedure.name, procedure.id
            ),
          )
          .await;
        self.execute_sequence(ids, map, update).await.with_context(
          || {
            let time = Duration::from_millis(
              (monitor_timestamp() - start_ts) as u64,
            );
            format!(
              "failed sequence execution after {time:?}. {} ({})",
              procedure.name, procedure.id
            )
          },
        )?;
        let time = Duration::from_millis(
          (monitor_timestamp() - start_ts) as u64,
        );
        self
          .add_line_to_update(
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
        self
          .add_line_to_update(
            update,
            &format!(
              "executing parallel: {} ({})",
              procedure.name, procedure.id
            ),
          )
          .await;
        self.execute_parallel(ids, map, update).await.with_context(
          || {
            let time = Duration::from_millis(
              (monitor_timestamp() - start_ts) as u64,
            );
            format!(
              "failed parallel execution after {time:?}. {} ({})",
              procedure.name, procedure.id
            )
          },
        )?;
        let time = Duration::from_millis(
          (monitor_timestamp() - start_ts) as u64,
        );
        self
          .add_line_to_update(
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
    &self,
    execution: Execution,
  ) -> anyhow::Result<()> {
    let user: Arc<_> = InnerRequestUser::admin().into();
    let update =
      match execution {
        Execution::RunBuild(req) => self
          .resolve(req, user)
          .await
          .context("failed at RunBuild")?,
        Execution::Deploy(req) => {
          self.resolve(req, user).await.context("failed at Deploy")?
        }
        Execution::StartContainer(req) => self
          .resolve(req, user)
          .await
          .context("failed at StartContainer")?,
        Execution::StopContainer(req) => self
          .resolve(req, user)
          .await
          .context("failed at StopContainer")?,
        Execution::StopAllContainers(req) => self
          .resolve(req, user)
          .await
          .context("failed at StopAllContainers")?,
        Execution::RemoveContainer(req) => self
          .resolve(req, user)
          .await
          .context("failed at RemoveContainer")?,
        Execution::CloneRepo(req) => self
          .resolve(req, user)
          .await
          .context("failed at CloneRepo")?,
        Execution::PullRepo(req) => self
          .resolve(req, user)
          .await
          .context("failed at PullRepo")?,
        Execution::PruneDockerNetworks(req) => self
          .resolve(req, user)
          .await
          .context("failed at PruneDockerNetworks")?,
        Execution::PruneDockerImages(req) => self
          .resolve(req, user)
          .await
          .context("failed at PruneDockerImages")?,
        Execution::PruneDockerContainers(req) => self
          .resolve(req, user)
          .await
          .context("failed at PruneDockerContainers")?,
        Execution::RunProcedure(req) => self
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
    &self,
    ids: &[String],
    map: &HashMap<String, Procedure>,
    update: &Mutex<Update>,
  ) -> anyhow::Result<()> {
    for id in ids {
      let procedure = map.get(id).with_context(|| {
        format!("no procedure on map with id {id}")
      })?;
      self.execute_procedure(procedure, map, update).await?;
    }
    Ok(())
  }

  async fn execute_parallel(
    &self,
    ids: &[String],
    map: &HashMap<String, Procedure>,
    update: &Mutex<Update>,
  ) -> anyhow::Result<()> {
    let futures = ids.iter().map(|id| async {
      let procedure = map.get(id).context("no procedure on map")?;
      self.execute_procedure(procedure, map, update).await
    });
    join_all(futures)
      .await
      .into_iter()
      .collect::<anyhow::Result<_>>()?;
    Ok(())
  }

  pub async fn make_procedure_map(
    &self,
    procedure: &Procedure,
  ) -> anyhow::Result<HashMap<String, Procedure>> {
    let map = Default::default();
    self.make_procedure_map_rec(procedure, &map).await?;
    Ok(map.into_inner())
  }

  #[async_recursion]
  async fn make_procedure_map_rec(
    &self,
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
      .map(|id| ObjectId::from_str(id).context("id is not ObjectId"))
      .collect::<anyhow::Result<Vec<_>>>()?;

    let procedures = find_collect(
      &self.db.procedures,
      doc! { "_id": { "$in": &more_ids } },
      None,
    )
    .await
    .context("failed to find procedures from db")?
    .into_iter()
    .map(|proc| (proc.id.clone(), proc))
    .collect::<HashMap<_, _>>();

    // make sure we aren't missing any procedures
    for more in more {
      if !procedures.contains_key(more) {
        return Err(anyhow!(
          "did not find a procedure with id {more}"
        ));
      }
    }

    let futures = procedures.values().map(|procedure| async {
      self.make_procedure_map_rec(procedure, map).await
    });

    join_all(futures)
      .await
      .into_iter()
      .collect::<anyhow::Result<Vec<_>>>()?;

    map.lock().await.extend(procedures);

    Ok(())
  }
}
