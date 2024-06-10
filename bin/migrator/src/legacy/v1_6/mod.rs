use mungos::{init::MongoBuilder, mongodb::Collection};
use serde::{Deserialize, Serialize};

pub mod build;
pub mod deployment;
pub mod resource;

pub struct DbClient {
  pub deployments: Collection<deployment::Deployment>,
  pub builds: Collection<build::Build>,
}

impl DbClient {
  pub async fn new(
    legacy_uri: &str,
    legacy_db_name: &str,
  ) -> DbClient {
    let client = MongoBuilder::default()
      .uri(legacy_uri)
      .build()
      .await
      .expect("failed to init legacy mongo client");
    let db = client.database(legacy_db_name);
    DbClient {
      deployments: db.collection("Deployment"),
      builds: db.collection("Build"),
    }
  }
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct Version {
  pub major: i32,
  pub minor: i32,
  pub patch: i32,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq,
)]
pub struct SystemCommand {
  #[serde(default)]
  pub path: String,
  #[serde(default)]
  pub command: String,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct EnvironmentVar {
  pub variable: String,
  pub value: String,
}

impl From<EnvironmentVar>
  for monitor_client::entities::EnvironmentVar
{
  fn from(value: EnvironmentVar) -> Self {
    monitor_client::entities::EnvironmentVar {
      variable: value.variable,
      value: value.value,
    }
  }
}
