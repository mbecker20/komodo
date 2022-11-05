use std::time::Duration;

use mungos::{Collection, Mungos};
use types::{Build, Deployment, Server, Update, User};

mod collections;

pub struct DbClient {
    pub users: Collection<User>,
    pub servers: Collection<Server>,
    pub deployments: Collection<Deployment>,
    pub builds: Collection<Build>,
    pub updates: Collection<Update>,
}

impl DbClient {
    pub async fn new(mongo_uri: &str, app_name: &str) -> anyhow::Result<DbClient> {
        let mungos = Mungos::new(mongo_uri, app_name, Duration::from_secs(3), None).await?;
        todo!()
    }
}
