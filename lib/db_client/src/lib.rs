use std::time::Duration;

use collections::{
    builds_collection, deployments_collection, procedures_collection, servers_collection,
    updates_collection, users_collection,
};
use mungos::{Collection, Mungos};
use types::{Build, Deployment, Procedure, Server, Update, User};

mod collections;

pub struct DbClient {
    pub users: Collection<User>,
    pub servers: Collection<Server>,
    pub deployments: Collection<Deployment>,
    pub builds: Collection<Build>,
    pub procedures: Collection<Procedure>,
    pub updates: Collection<Update>,
}

impl DbClient {
    pub async fn new(mongo_uri: &str, app_name: &str, db_name: &str) -> anyhow::Result<DbClient> {
        let mungos = Mungos::new(mongo_uri, app_name, Duration::from_secs(3), None).await?;
        let client = DbClient {
            users: users_collection(&mungos, db_name).await?,
            servers: servers_collection(&mungos, db_name).await?,
            deployments: deployments_collection(&mungos, db_name).await?,
            builds: builds_collection(&mungos, db_name).await?,
            updates: updates_collection(&mungos, db_name).await?,
            procedures: procedures_collection(&mungos, db_name).await?,
        };
        Ok(client)
    }
}
