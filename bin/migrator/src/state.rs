use anyhow::Context;
use db_client::DbClient;
use monitor_client::entities::config::MongoConfig;
use mungos::{
  init::MongoBuilder,
  mongodb::{Client, Collection, Database},
};
use serde::Deserialize;

use crate::legacy::v0::{Build, Deployment, Server, Update, User};

#[derive(Deserialize, Debug)]
struct Env {
  legacy_uri: String,
  legacy_db_name: String,
  target_uri: String,
  target_db_name: String,
}

pub struct State {
  pub legacy: LegacyDbClient,
  pub target: DbClient,
}

impl State {
  pub async fn load() -> anyhow::Result<State> {
    dotenv::dotenv().ok();
    let env = envy::from_env::<Env>()?;
    let legacy_client = MongoBuilder::default()
      .uri(&env.legacy_uri)
      .build()
      .await
      .context("failed to init legacy mongo client")?;
    let target_client = MongoBuilder::default()
      .uri(&env.target_uri)
      .build()
      .await
      .context("failed to init target mongo client")?;
    let state = State {
      legacy: LegacyDbClient::new(
        &legacy_client.database(&env.legacy_db_name),
      ),
      target: DbClient::new(&MongoConfig {
        uri: Some(env.target_uri),
        db_name: env.target_db_name,
        app_name: "migrator".to_string(),
        ..Default::default()
      })
      .await?,
    };
    Ok(state)
  }
}

pub struct LegacyDbClient {
  pub users: Collection<User>,
  pub servers: Collection<Server>,
  pub deployments: Collection<Deployment>,
  pub builds: Collection<Build>,
  pub updates: Collection<Update>,
}

impl LegacyDbClient {
  pub fn new(db: &Database) -> LegacyDbClient {
    LegacyDbClient {
      users: db.collection("users"),
      servers: db.collection("servers"),
      deployments: db.collection("deployments"),
      builds: db.collection("builds"),
      updates: db.collection("updates"),
    }
  }
}
