use std::{sync::Arc, net::SocketAddr, str::FromStr};

use anyhow::Context;
use simple_logger::SimpleLogger;

use crate::config::{CoreConfig, Env};

pub struct State {
    pub env: Env,
    pub config: CoreConfig,
}

impl State {
    pub async fn load() -> anyhow::Result<Arc<State>> {
        let env = Env::load()?;
        let config = CoreConfig::load(&env.config_path);

        SimpleLogger::new()
            .with_level(config.log_level.into())
            .env()
            .with_colors(true)
            .with_utc_timestamps()
            .init()
            .context("failed to configure logger")?;

        let state = State { env, config };

        Ok(state.into())
    }

    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        SocketAddr::from_str(&format!("0.0.0.0:{}", self.config.port))
            .context("failed to parse socket addr")
    }
}
