use anyhow::Context;
use monitor_types::entities::{
    alerter::Alerter,
    build::Build,
    builder::Builder,
    deployment::Deployment,
    repo::Repo,
    server::{stats::SystemStatsRecord, Server},
    tag::CustomTag,
    update::Update,
    user::User,
};
use mungos::{Collection, Indexed, Mungos};

use crate::config::{CoreConfig, MongoConfig};

pub struct DbClient {
    pub users: Collection<User>,
    pub servers: Collection<Server>,
    pub stats: Collection<SystemStatsRecord>,
    pub deployments: Collection<Deployment>,
    pub builds: Collection<Build>,
    pub builders: Collection<Builder>,
    pub repos: Collection<Repo>,
    pub tags: Collection<CustomTag>,
    pub alerters: Collection<Alerter>,
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
            tags: CustomTag::collection(&mungos, db_name, true).await?,
            updates: Update::collection(&mungos, db_name, true).await?,
            stats: SystemStatsRecord::collection(&mungos, db_name, true).await?,
            servers: resource_collection(&mungos, db_name, "Server").await?,
            deployments: resource_collection(&mungos, db_name, "Deployment").await?,
            builds: resource_collection(&mungos, db_name, "Build").await?,
            builders: resource_collection(&mungos, db_name, "Builder").await?,
            repos: resource_collection(&mungos, db_name, "Repo").await?,
            alerters: resource_collection(&mungos, db_name, "Alerter").await?,
        };
        Ok(client)
    }
}

async fn resource_collection<T>(
    mungos: &Mungos,
    db_name: &str,
    collection_name: &str,
) -> anyhow::Result<Collection<T>> {
    let coll = mungos.collection::<T>(db_name, collection_name);

    coll.create_unique_index("name")
        .await
        .context("failed to create name index")?;

    coll.create_index("tags")
        .await
        .context("failed to create tags index")?;

    Ok(coll)
}
