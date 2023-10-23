use monitor_types::entities::{
  deployment::{Deployment, DeploymentConfig},
  update::ResourceTarget,
};
use mungos::Indexed;

use crate::{
  legacy::{self, v0},
  state::State,
};

impl State {
  pub async fn migrate_servers(
    &self,
    legacy_db: &str,
    target_db: &str,
  ) -> anyhow::Result<()> {
    let source = self
      .legacy_mungos
      .collection::<v0::Deployment>(legacy_db, "deployments");

    todo!()
  }
}
