mod deployment;
mod server;

use std::collections::HashMap;

use anyhow::Context;
use monitor_types::entities::server::{Server, ServerListItem};

use crate::{
    auth::InnerRequestUser, helpers::resource::StateResource,
    state::State,
};

impl State {
    // called after cache update
    pub async fn check_alerts(&self, ts: i64) {
        let servers = self.get_all_servers_map().await;

        if let Err(e) = servers {
            error!("{e:#?}");
            return;
        }

        let (servers, server_names) = servers.unwrap();

        tokio::join!(
            self.alert_servers(ts, servers),
            self.alert_deployments(ts, server_names)
        );
    }

    async fn get_all_servers_map(
        &self,
    ) -> anyhow::Result<(
        HashMap<String, ServerListItem>,
        HashMap<String, String>,
    )> {
        let servers = <State as StateResource<Server>>::list_resources_for_user(
            self,
            None,
            &InnerRequestUser {
                is_admin: true,
                ..Default::default()
            }
            .into(),
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
}
