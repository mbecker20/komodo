mod deployment;
mod server;

use std::collections::HashMap;

use anyhow::Context;
use monitor_client::entities::{
  server::{Server, ServerListItem},
  user::User,
};
use mungos::mongodb::bson::Document;

use crate::helpers::resource::StateResource;

// called after cache update
pub async fn check_alerts(ts: i64) {
  let servers = get_all_servers_map().await;

  if let Err(e) = servers {
    error!("{e:#?}");
    return;
  }

  let (servers, server_names) = servers.unwrap();

  tokio::join!(
    server::alert_servers(ts, servers),
    deployment::alert_deployments(ts, server_names)
  );
}

async fn get_all_servers_map() -> anyhow::Result<(
  HashMap<String, ServerListItem>,
  HashMap<String, String>,
)> {
  let servers = Server::list_resources_for_user(
    Document::new(),
    &User {
      admin: true,
      ..Default::default()
    },
  )
  .await
  .context("failed to get servers from db (in alert_servers)")?;

  let servers = servers
    .into_iter()
    .map(|server| (server.id.clone(), server))
    .collect::<HashMap<_, _>>();

  let server_names = servers
    .iter()
    .map(|(id, server)| (id.clone(), server.name.clone()))
    .collect::<HashMap<_, _>>();

  Ok((servers, server_names))
}
