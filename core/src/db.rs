use monitor_types::entities::{
    build::Build, builder::Builder, deployment::Deployment, repo::Repo, server::Server,
    update::Update, user::User,
};
use mungos::{Collection, Indexed, Mungos};

use crate::config::CoreConfig;

pub struct DbClient {
    pub users: Collection<User>,
    pub servers: Collection<Server>,
    pub deployments: Collection<Deployment>,
    pub builds: Collection<Build>,
    pub builders: Collection<Builder>,
    pub repos: Collection<Repo>,
    pub updates: Collection<Update>,
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
            builders: Builder::collection(&mungos, &config.mongo.db_name, true).await?,
            repos: Repo::collection(&mungos, &config.mongo.db_name, true).await?,
            updates: Update::collection(&mungos, &config.mongo.db_name, true).await?,
        };
        Ok(client)
    }
}
