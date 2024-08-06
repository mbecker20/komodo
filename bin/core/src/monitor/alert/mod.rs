use std::collections::HashMap;

use anyhow::Context;
use monitor_client::entities::{
  resource::ResourceQuery,
  server::{Server, ServerListItem},
  user::User,
};

use crate::resource;

mod deployment;
mod server;

// called after cache update
#[instrument(level = "debug")]
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

#[instrument(level = "debug")]
async fn get_all_servers_map() -> anyhow::Result<(
  HashMap<String, ServerListItem>,
  HashMap<String, String>,
)> {
  let servers = resource::list_for_user::<Server>(
    ResourceQuery::default(),
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
