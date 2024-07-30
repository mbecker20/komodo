#[macro_use]
extern crate tracing;

use monitor_client::entities::{
  build::Build, deployment::Deployment, permission::Permission,
  server::Server, update::Update, user::User,
};
use mungos::{init::MongoBuilder, mongodb::Collection};
use serde::Deserialize;

mod legacy;
mod migrate;

#[derive(Deserialize)]
enum Migration {
  #[serde(alias = "v0")]
  V0,
  #[serde(alias = "v1.6")]
  V1_6,
  #[serde(alias = "v1.11")]
  V1_11,
}

#[derive(Deserialize)]
struct Env {
  migration: Migration,
  legacy_uri: String,
  legacy_db_name: String,
  target_uri: String,
  target_db_name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv::dotenv().ok();
  logger::init(&Default::default())?;

  info!("starting migrator");

  let env: Env = envy::from_env()?;

  match env.migration {
    Migration::V0 => {
      let legacy_db = legacy::v0::DbClient::new(
        &env.legacy_uri,
        &env.legacy_db_name,
      )
      .await;
      let target_db =
        DbClient::new(&env.target_uri, &env.target_db_name).await?;
      migrate::v0::migrate_all(&legacy_db, &target_db).await?
    }
    Migration::V1_6 => {
      let db = legacy::v1_6::DbClient::new(
        &env.target_uri,
        &env.target_db_name,
      )
      .await;
      migrate::v1_6::migrate_all_in_place(&db).await?
    }
    Migration::V1_11 => {
      let db = legacy::v1_11::DbClient::new(
        &env.target_uri,
        &env.target_db_name,
      )
      .await;
      migrate::v1_11::migrate_all_in_place(&db).await?
    }
  }

  info!("finished!");

  Ok(())
}

struct DbClient {
  pub users: Collection<User>,
  pub updates: Collection<Update>,
  pub servers: Collection<Server>,
  pub deployments: Collection<Deployment>,
  pub builds: Collection<Build>,
  pub permissions: Collection<Permission>,
}

impl DbClient {
  pub async fn new(
    uri: &str,
    db_name: &str,
  ) -> anyhow::Result<DbClient> {
    let client = MongoBuilder::default().uri(uri).build().await?;
    let db = client.database(db_name);
    Ok(DbClient {
      users: db.collection("User"),
      updates: db.collection("Update"),
      servers: db.collection("Server"),
      deployments: db.collection("Deployment"),
      builds: db.collection("Build"),
      permissions: db.collection("Permission"),
    })
  }
}
