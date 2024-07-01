use mongo_indexed::{create_index, create_unique_index};
use monitor_client::entities::{
  alert::Alert,
  alerter::Alerter,
  api_key::ApiKey,
  build::Build,
  builder::Builder,
  config::core::MongoConfig,
  deployment::Deployment,
  permission::Permission,
  procedure::Procedure,
  repo::Repo,
  server::{stats::SystemStatsRecord, Server},
  server_template::ServerTemplate,
  sync::ResourceSync,
  tag::Tag,
  update::Update,
  user::User,
  user_group::UserGroup,
  variable::Variable,
};
use mungos::{
  init::MongoBuilder,
  mongodb::{Collection, Database},
};

pub struct DbClient {
  pub users: Collection<User>,
  pub user_groups: Collection<UserGroup>,
  pub permissions: Collection<Permission>,
  pub api_keys: Collection<ApiKey>,
  pub tags: Collection<Tag>,
  pub variables: Collection<Variable>,
  pub updates: Collection<Update>,
  pub alerts: Collection<Alert>,
  pub stats: Collection<SystemStatsRecord>,
  // RESOURCES
  pub servers: Collection<Server>,
  pub deployments: Collection<Deployment>,
  pub builds: Collection<Build>,
  pub builders: Collection<Builder>,
  pub repos: Collection<Repo>,
  pub procedures: Collection<Procedure>,
  pub alerters: Collection<Alerter>,
  pub server_templates: Collection<ServerTemplate>,
  pub resource_syncs: Collection<ResourceSync>,
  //
  pub db: Database,
}

impl DbClient {
  pub async fn new(
    MongoConfig {
      uri,
      address,
      username,
      password,
      app_name,
      db_name,
    }: &MongoConfig,
  ) -> anyhow::Result<DbClient> {
    let mut client = MongoBuilder::default().app_name(app_name);

    match (uri, address, username, password) {
      (Some(uri), _, _, _) => {
        client = client.uri(uri);
      }
      (_, Some(address), Some(username), Some(password)) => {
        client = client
          .address(address)
          .username(username)
          .password(password);
      }
      (_, Some(address), _, _) => {
        client = client.address(address);
      }
      _ => {
        error!("config.mongo not configured correctly. must pass either config.mongo.uri, or config.mongo.address + config.mongo.username? + config.mongo.password?");
        std::process::exit(1)
      }
    }

    let client = client.build().await?;
    let db = client.database(db_name);

    let client = DbClient {
      users: mongo_indexed::collection(&db, true).await?,
      user_groups: mongo_indexed::collection(&db, true).await?,
      permissions: mongo_indexed::collection(&db, true).await?,
      api_keys: mongo_indexed::collection(&db, true).await?,
      tags: mongo_indexed::collection(&db, true).await?,
      variables: mongo_indexed::collection(&db, true).await?,
      updates: mongo_indexed::collection(&db, true).await?,
      alerts: mongo_indexed::collection(&db, true).await?,
      stats: mongo_indexed::collection(&db, true).await?,
      // RESOURCES
      servers: resource_collection(&db, "Server").await?,
      deployments: resource_collection(&db, "Deployment").await?,
      builds: resource_collection(&db, "Build").await?,
      builders: resource_collection(&db, "Builder").await?,
      repos: resource_collection(&db, "Repo").await?,
      alerters: resource_collection(&db, "Alerter").await?,
      procedures: resource_collection(&db, "Procedure").await?,
      server_templates: resource_collection(&db, "ServerTemplate")
        .await?,
      resource_syncs: resource_collection(&db, "ResourceSync")
        .await?,
      //
      db,
    };
    Ok(client)
  }
}

async fn resource_collection<T: Send + Sync>(
  db: &Database,
  collection_name: &str,
) -> anyhow::Result<Collection<T>> {
  let coll = db.collection::<T>(collection_name);

  create_unique_index(&coll, "name").await?;

  create_index(&coll, "tags").await?;

  Ok(coll)
}
