// #![allow(unused)]

use std::{fs::File, net::SocketAddr, sync::Arc};

use ::helpers::get_socket_addr;
use axum::Extension;
use daemonize::Daemonize;
use types::PeripheryConfig;

mod api;
mod config;
mod helpers;

type PeripheryConfigExtension = Extension<Arc<PeripheryConfig>>;
type HomeDirExtension = Extension<Arc<String>>;

fn main() -> anyhow::Result<()> {
    let (args, port, config, home_dir) = config::load();

    if args.daemon {
        let stdout = File::create(args.stdout.replace("~", &home_dir))
            .expect("failed to create stdout log file");
        let stderr = File::create(args.stderr.replace("~", &home_dir))
            .expect("failed to create stderr log file");
        let daemon = Daemonize::new().stdout(stdout).stderr(stderr);
        match daemon.start() {
            Ok(_) => println!("monitor periphery"),
            Err(e) => eprintln!("Error, {}", e),
        }
    }

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
