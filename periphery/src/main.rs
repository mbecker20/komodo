// #![allow(unused)]

use std::{net::SocketAddr, sync::Arc};

use ::helpers::get_socket_addr;
use axum::Extension;
use types::PeripheryConfig;

mod api;
mod config;
mod helpers;

type PeripheryConfigExtension = Extension<Arc<PeripheryConfig>>;
type HomeDirExtension = Extension<Arc<String>>;

fn main() -> anyhow::Result<()> {
    let (port, config, home_dir) = config::load();

    run_periphery_server(port, config, home_dir)?;

    Ok(())
}

#[tokio::main]
async fn run_periphery_server(
    port: u16,
    config: PeripheryConfigExtension,
    home_dir: HomeDirExtension,
) -> anyhow::Result<()> {
    let app = api::router(config, home_dir);

    axum::Server::bind(&get_socket_addr(port))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
