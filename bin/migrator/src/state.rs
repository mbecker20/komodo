use anyhow::Context;
use mongo_indexed::{create_index, create_unique_index, Indexed};
use monitor_client::entities::{
  build::Build, deployment::Deployment, server::Server,
  update::Update, user::User,
};
use mungos::{
  init::MongoBuilder,
  mongodb::{Client, Collection, Database},
};
use serde::Deserialize;

use crate::legacy::v0;

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
      target: DbClient::new(
        &target_client.database(&env.target_db_name),
      )
      .await?,
    };
    Ok(state)
  }
}

pub struct DbClient {
  pub users: Collection<User>,
  pub updates: Collection<Update>,
  pub servers: Collection<Server>,
  pub deployments: Collection<Deployment>,
  pub builds: Collection<Build>,
}

impl DbClient {
  pub async fn new(db: &Database) -> anyhow::Result<DbClient> {
    Ok(DbClient {
      users: mongo_indexed::collection::<User>(db, true).await?,
      updates: mongo_indexed::collection::<Update>(db, true).await?,
      servers: resource_collection(db, "Server").await?,
      deployments: resource_collection(db, "Deployment").await?,
      builds: resource_collection(db, "Build").await?,
    })
  }
}

async fn resource_collection<T>(
  db: &Database,
  collection_name: &str,
) -> anyhow::Result<Collection<T>> {
  let coll = db.collection::<T>(collection_name);

  create_unique_index(&coll, "name").await?;

  create_index(&coll, "tags").await?;

  Ok(coll)
}

pub struct LegacyDbClient {
  pub users: Collection<v0::User>,
  pub servers: Collection<v0::Server>,
  pub deployments: Collection<v0::Deployment>,
  pub builds: Collection<v0::Build>,
  pub updates: Collection<v0::Update>,
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
