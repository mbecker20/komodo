use std::sync::Arc;

use anyhow::anyhow;
use monitor_client::{
  busy::Busy,
  entities::{
    build::BuildActionState, deployment::DeploymentActionState,
    procedure::ProcedureActionState, repo::RepoActionState,
    server::ServerActionState,
  },
};
use tokio::sync::Mutex;

use super::cache::Cache;

#[derive(Default)]
pub struct ActionStates {
  pub build: Cache<String, Arc<ActionState<BuildActionState>>>,
  pub deployment:
    Cache<String, Arc<ActionState<DeploymentActionState>>>,
  pub server: Cache<String, Arc<ActionState<ServerActionState>>>,
  pub repo: Cache<String, Arc<ActionState<RepoActionState>>>,
  pub procedure:
    Cache<String, Arc<ActionState<ProcedureActionState>>>,
}

/// Need to be able to check "busy" with write lock acquired.
#[derive(Default)]
pub struct ActionState<States: Default>(Mutex<States>);

impl<States: Default + Busy + Copy> ActionState<States> {
  pub async fn get(&self) -> States {
    *self.0.lock().await
  }

  pub async fn busy(&self) -> bool {
    self.0.lock().await.busy()
  }

  /// Will acquire lock, check busy, and if not will
  /// run the provided update function on the states.
  /// Returns a guard that returns the states to default (not busy) when dropped.
  pub async fn update(
    &self,
    handler: impl Fn(&mut States),
  ) -> anyhow::Result<UpdateGuard<States>> {
    let mut lock = self.0.lock().await;
    if lock.busy() {
      return Err(anyhow!("resource is busy"));
    }
    handler(&mut *lock);
    Ok(UpdateGuard(&self.0))
  }
}

/// When dropped will return the inner state to default.
/// The inner mutex guard must already be dropped BEFORE this is dropped,
/// which is guaranteed as the inner guard is dropped by all public methods before
/// user could drop UpdateGuard.
pub struct UpdateGuard<'a, States: Default>(&'a Mutex<States>);

impl<'a, States: Default> Drop for UpdateGuard<'a, States> {
  fn drop(&mut self) {
    let mut lock = self.0.blocking_lock();
    *lock = Default::default();
  }
}
