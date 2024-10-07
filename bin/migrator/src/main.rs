#![allow(unused)]
#[macro_use]
extern crate tracing;

use serde::Deserialize;

mod legacy;
mod migrate;

#[derive(Deserialize)]
enum Migration {
  #[serde(alias = "v1.11")]
  V1_11,
}

#[derive(Deserialize)]
struct Env {
  migration: Migration,
  target_uri: String,
  target_db_name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();
  logger::init(&Default::default())?;

  info!("starting migrator");

  let env: Env = envy::from_env()?;

  match env.migration {
    Migration::V1_11 => {
      // let db = legacy::v1_11::DbClient::new(
      //   &env.target_uri,
      //   &env.target_db_name,
      // )
      // .await;
      // migrate::v1_11::migrate_all_in_place(&db).await?
    }
  }

  info!("finished!");

  Ok(())
}
