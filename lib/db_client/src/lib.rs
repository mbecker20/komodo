use std::{sync::Arc, time::Duration};

use axum::Extension;
use collections::{
    builds_collection, deployments_collection, procedures_collection, servers_collection,
    updates_collection, users_collection,
};
use mungos::{Collection, Mungos};
use types::{Build, Deployment, Procedure, Server, Update, User, MongoConfig};

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

impl DbClient {
    pub async fn extension(config: MongoConfig) -> DbExtension {
        let db_name = &config.db_name;
        let mungos = Mungos::new(
            &config.uri,
            &config.app_name,
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
