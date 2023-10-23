use db_client::DbClient;
use monitor_types::entities::config::MongoConfig;
use mungos::{Collection, Mungos};
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
  pub legacy_mungos: Mungos,
  pub target: DbClient,
}

impl State {
  pub async fn load() -> anyhow::Result<State> {
    dotenv::dotenv().ok();
    let env = envy::from_env::<Env>()?;
    let legacy_mungos =
      Mungos::builder().uri(&env.legacy_uri).build().await?;
    let target_mungos =
      Mungos::builder().uri(&env.target_uri).build().await?;
    let state = State {
      legacy_mungos,
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
