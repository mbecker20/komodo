use mungos::Mungos;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Env {
    legacy_mongo_uri: String,
    target_mongo_uri: String,
}

pub struct State {
    pub legacy_mungos: Mungos,
    pub target_mungos: Mungos,
}

impl State {
    pub async fn load() -> anyhow::Result<State> {
        dotenv::dotenv().ok();
        let env = envy::from_env::<Env>()?;
        let legacy_mungos = Mungos::builder().uri(&env.legacy_mongo_uri).build().await?;
        let target_mungos = Mungos::builder().uri(&env.target_mongo_uri).build().await?;
        let state = State {
            legacy_mungos,
            target_mungos,
        };
        Ok(state)
    }
}
