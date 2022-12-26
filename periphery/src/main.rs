// #![allow(unused)]

use std::{env, fs::File, sync::Arc};

use ::helpers::get_socket_addr;
use axum::Extension;
use daemonize::Daemonize;
use types::PeripheryConfig;

mod api;
mod config;
mod helpers;

type PeripheryConfigExtension = Extension<Arc<PeripheryConfig>>;

fn main() {
    let (args, port, config) = config::load();

    let home = env::var("HOME").unwrap();

    if args.daemon {
        let stdout = File::create(args.stdout.replace("~", &home)).unwrap();
        let stderr = File::create(args.stderr.replace("~", &home)).unwrap();
        let daemon = Daemonize::new().stdout(stdout).stderr(stderr);
        match daemon.start() {
            Ok(_) => println!("process sucessfully started"),
            Err(e) => eprintln!("Error, {}", e),
        }
    }

    run_periphery_server(port, config)
}

#[tokio::main]
async fn run_periphery_server(port: u16, config: PeripheryConfigExtension) {
    let app = api::router(&config).layer(config);

    axum::Server::bind(&get_socket_addr(port))
        .serve(app.into_make_service())
        .await
        .expect("monitor periphery axum server crashed");
}
