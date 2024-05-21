#[macro_use]
extern crate tracing;

use monitor_client::entities::{
  build::Build, deployment::Deployment, server::Server,
  update::Update, user::User,
};
use mungos::{init::MongoBuilder, mongodb::Collection};
use serde::Deserialize;

mod legacy;
mod migrate;

#[derive(Deserialize)]
struct Env {
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

  let legacy_db =
    legacy::v0::DbClient::new(&env.legacy_uri, &env.legacy_db_name)
      .await;
  let target_db =
    DbClient::new(&env.target_uri, &env.target_db_name).await?;

  migrate::migrate_all(&legacy_db, &target_db).await?;

  info!("finished!");

  Ok(())
}

struct DbClient {
  pub users: Collection<User>,
  pub updates: Collection<Update>,
  pub servers: Collection<Server>,
  pub deployments: Collection<Deployment>,
  pub builds: Collection<Build>,
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
    })
  }
}
