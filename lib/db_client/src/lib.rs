use std::{sync::Arc, time::Duration};

use axum::Extension;
use collections::{
    builds_collection, deployments_collection, procedures_collection, servers_collection,
    updates_collection, users_collection,
};
use mungos::{Collection, Mungos};
use types::{Build, CoreConfig, Deployment, Procedure, Server, Update, User};

mod collections;

pub type DbExtension = Extension<Arc<DbClient>>;

pub struct DbClient {
    pub users: Collection<User>,
    pub servers: Collection<Server>,
    pub deployments: Collection<Deployment>,
    pub builds: Collection<Build>,
    pub procedures: Collection<Procedure>,
    pub updates: Collection<Update>,
}

pub struct DbConfig {
    mongo_uri: String,
    mongo_app_name: String,
    mongo_db_name: String,
}

impl From<&CoreConfig> for DbConfig {
    fn from(config: &CoreConfig) -> DbConfig {
        DbConfig {
            mongo_uri: config.mongo_uri.clone(),
            mongo_app_name: config.mongo_app_name.clone(),
            mongo_db_name: config.mongo_db_name.clone(),
        }
    }
}

impl DbClient {
    pub async fn extension(config: DbConfig) -> DbExtension {
        let db_name = &config.mongo_db_name;
        let mungos = Mungos::new(
            &config.mongo_uri,
            &config.mongo_app_name,
            Duration::from_secs(3),
            None,
        )
        .await
        .expect("failed to initialize mungos");
        let client = DbClient {
            users: users_collection(&mungos, db_name)
                .await
                .expect("failed to make users collection"),
            servers: servers_collection(&mungos, db_name)
                .await
                .expect("failed to make servers collection"),
            deployments: deployments_collection(&mungos, db_name)
                .await
                .expect("failed to make deployments collection"),
            builds: builds_collection(&mungos, db_name)
                .await
                .expect("failed to make builds collection"),
            updates: updates_collection(&mungos, db_name)
                .await
                .expect("failed to make updates collection"),
            procedures: procedures_collection(&mungos, db_name)
                .await
                .expect("failed to make procedures collection"),
        };
        Extension(Arc::new(client))
    }
}
