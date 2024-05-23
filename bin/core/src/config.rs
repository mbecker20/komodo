use std::sync::OnceLock;

use anyhow::Context;
use merge_config_files::parse_config_file;
use monitor_client::entities::config::core::{CoreConfig, Env};
use serde::Deserialize;

pub fn frontend_path() -> &'static String {
  #[derive(Deserialize)]
  struct FrontendEnv {
    #[serde(default = "default_frontend_path")]
    monitor_frontend_path: String,
  }

  fn default_frontend_path() -> String {
    "/frontend".to_string()
  }

  static FRONTEND_PATH: OnceLock<String> = OnceLock::new();
  FRONTEND_PATH.get_or_init(|| {
    let FrontendEnv {
      monitor_frontend_path,
    } = envy::from_env()
      .context("failed to parse FrontendEnv")
      .unwrap();
    monitor_frontend_path
  })
}

pub fn core_config() -> &'static CoreConfig {
  static CORE_CONFIG: OnceLock<CoreConfig> = OnceLock::new();
  CORE_CONFIG.get_or_init(|| {
    let env: Env = envy::from_env()
      .context("failed to parse core Env")
      .unwrap();
    let config_path = &env.monitor_config_path;
    let mut config =
      parse_config_file::<CoreConfig>(config_path.as_str())
        .unwrap_or_else(|e| {
          panic!("failed at parsing config at {config_path} | {e:#}")
        });

    // Overrides
    config.title = env.monitor_title.unwrap_or(config.title);
    config.host = env.monitor_host.unwrap_or(config.host);
    config.port = env.monitor_port.unwrap_or(config.port);
    config.passkey = env.monitor_passkey.unwrap_or(config.passkey);
    config.jwt_valid_for =
      env.monitor_jwt_valid_for.unwrap_or(config.jwt_valid_for);
    config.monitoring_interval = env
      .monitor_monitoring_interval
      .unwrap_or(config.monitoring_interval);
    config.keep_stats_for_days = env
      .monitor_keep_stats_for_days
      .unwrap_or(config.keep_stats_for_days);
    config.keep_alerts_for_days = env
      .monitor_keep_alerts_for_days
      .unwrap_or(config.keep_alerts_for_days);
    config.github_webhook_secret = env
      .monitor_github_webhook_secret
      .unwrap_or(config.github_webhook_secret);
    config.github_webhook_base_url = env
      .monitor_github_webhook_base_url
      .or(config.github_webhook_base_url);
    config.docker_organizations = env
      .monitor_docker_organizations
      .unwrap_or(config.docker_organizations);

    config.logging.level =
      env.monitor_logging_level.unwrap_or(config.logging.level);
    config.logging.stdio =
      env.monitor_logging_stdio.unwrap_or(config.logging.stdio);
    config.logging.otlp_endpoint = env
      .monitor_logging_otlp_endpoint
      .or(config.logging.otlp_endpoint);
    config.logging.opentelemetry_service_name = env
      .monitor_logging_opentelemetry_service_name
      .unwrap_or(config.logging.opentelemetry_service_name);

    config.transparent_mode = env
      .monitor_transparent_mode
      .unwrap_or(config.transparent_mode);

    config.local_auth =
      env.monitor_local_auth.unwrap_or(config.local_auth);

    config.github_oauth.enabled = env
      .monitor_github_oauth_enabled
      .unwrap_or(config.github_oauth.enabled);
    config.github_oauth.id = env
      .monitor_github_oauth_id
      .unwrap_or(config.github_oauth.id);
    config.github_oauth.secret = env
      .monitor_github_oauth_secret
      .unwrap_or(config.github_oauth.secret);

    config.google_oauth.enabled = env
      .monitor_google_oauth_enabled
      .unwrap_or(config.google_oauth.enabled);
    config.google_oauth.id = env
      .monitor_google_oauth_id
      .unwrap_or(config.google_oauth.id);
    config.google_oauth.secret = env
      .monitor_google_oauth_secret
      .unwrap_or(config.google_oauth.secret);

    config.mongo.uri = env.monitor_mongo_uri.or(config.mongo.uri);
    config.mongo.address =
      env.monitor_mongo_address.or(config.mongo.address);
    config.mongo.username =
      env.monitor_mongo_username.or(config.mongo.username);
    config.mongo.password =
      env.monitor_mongo_password.or(config.mongo.password);
    config.mongo.app_name =
      env.monitor_mongo_app_name.unwrap_or(config.mongo.app_name);
    config.mongo.db_name =
      env.monitor_mongo_db_name.unwrap_or(config.mongo.db_name);

    config.aws.access_key_id = env
      .monitor_aws_access_key_id
      .unwrap_or(config.aws.access_key_id);
    config.aws.secret_access_key = env
      .monitor_aws_secret_access_key
      .unwrap_or(config.aws.secret_access_key);

    config
  })
}
