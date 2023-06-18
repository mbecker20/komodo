use monitor_types::entities::{build::Build, deployment::Deployment, server::Server, user::User};
use mungos::{Collection, Indexed, Mungos};

use crate::config::CoreConfig;

pub struct DbClient {
    // mungos: Mungos,
    pub users: Collection<User>,
    pub servers: Collection<Server>,
    pub deployments: Collection<Deployment>,
    pub builds: Collection<Build>,
}

impl DbClient {
    pub async fn new(config: &CoreConfig) -> anyhow::Result<DbClient> {
        let mungos = Mungos::builder()
            .uri(&config.mongo.uri)
            .app_name(&config.mongo.app_name)
            .build()
            .await?;
        let client = DbClient {
            users: User::collection(&mungos, &config.mongo.db_name, true).await?,
            servers: Server::collection(&mungos, &config.mongo.db_name, true).await?,
            deployments: Deployment::collection(&mungos, &config.mongo.db_name, true).await?,
            builds: Build::collection(&mungos, &config.mongo.db_name, true).await?,
            // mungos,
        };
        Ok(client)
    }
}
