use std::{net::SocketAddr, str::FromStr, sync::Arc};

use anyhow::Context;
use clap::Parser;
use simple_logger::SimpleLogger;

use crate::{
    config::{CliArgs, Env, PeripheryConfig},
    helpers::stats::{InnerStatsClient, StatsClient},
};

pub struct State {
    pub config: PeripheryConfig,
    pub stats: StatsClient,
}

impl State {
    pub async fn load() -> anyhow::Result<Arc<State>> {
        let env = Env::load()?;
        let args = CliArgs::parse();
        SimpleLogger::new()
            .with_level(args.log_level)
            .env()
            .with_colors(true)
            .with_utc_timestamps()
            .init()
            .context("failed to configure logger")?;
        info!("version: {}", env!("CARGO_PKG_VERSION"));
        let config = PeripheryConfig::load(&env, &args)?;
        let state = State {
            stats: InnerStatsClient::new(config.stats_polling_rate),
            config,
        };
        Ok(state.into())
    }

    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        SocketAddr::from_str(&format!("0.0.0.0:{}", self.config.port))
            .context("failed to parse socket addr")
    }
}
