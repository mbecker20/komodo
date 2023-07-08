use monitor_types::entities::{
    build::Build, builder::Builder, deployment::Deployment, repo::Repo, server::Server,
    tag::CustomTag, update::Update, user::User,
};
use mungos::{Collection, Indexed, Mungos};

use crate::config::{CoreConfig, MongoConfig};

pub struct DbClient {
    pub users: Collection<User>,
    pub servers: Collection<Server>,
    pub deployments: Collection<Deployment>,
    pub builds: Collection<Build>,
    pub builders: Collection<Builder>,
    pub repos: Collection<Repo>,
    pub tags: Collection<CustomTag>,
    pub updates: Collection<Update>,
}

impl DbClient {
    pub async fn new(
        CoreConfig {
            mongo:
                MongoConfig {
                    uri,
                    address,
                    username,
                    password,
                    app_name,
                    db_name,
                },
            ..
        }: &CoreConfig,
    ) -> anyhow::Result<DbClient> {
        let mut mungos = Mungos::builder().app_name(app_name);

        match (uri, address, username, password) {
            (Some(uri), _, _, _) => {
                mungos = mungos.uri(uri);
            }
            (_, Some(address), Some(username), Some(password)) => {
                mungos = mungos
                    .address(address)
                    .username(username)
                    .password(password);
            }
            (_, Some(address), _, _) => {
                mungos = mungos.address(address);
            }
            _ => {
                error!("config.mongo not configured correctly. must pass either config.mongo.uri, or config.mongo.address + config.mongo.username? + config.mongo.password?");
                std::process::exit(1)
            }
        }

        let mungos = mungos.build().await?;

        let client = DbClient {
            users: User::collection(&mungos, db_name, true).await?,
            servers: Server::collection(&mungos, db_name, true).await?,
            deployments: Deployment::collection(&mungos, db_name, true).await?,
            builds: Build::collection(&mungos, db_name, true).await?,
            builders: Builder::collection(&mungos, db_name, true).await?,
            repos: Repo::collection(&mungos, db_name, true).await?,
            tags: CustomTag::collection(&mungos, db_name, true).await?,
            updates: Update::collection(&mungos, db_name, true).await?,
        };
        Ok(client)
    }
}
