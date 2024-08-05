use std::sync::OnceLock;

use anyhow::Context;
use merge_config_files::parse_config_file;
use monitor_client::entities::{
  config::core::{
    AwsCredentials, CoreConfig, Env, GithubWebhookAppConfig,
    GithubWebhookAppInstallationConfig, HetznerCredentials,
    MongoConfig, OauthCredentials,
  },
  logger::LogConfig,
};
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
    let env: Env = match envy::from_env()
      .context("failed to parse core Env") {
        Ok(env) => env,
        Err(e) => {
          panic!("{e:#?}");
        }
      };
    let config_path = &env.monitor_config_path;
    let config =
      parse_config_file::<CoreConfig>(config_path.as_str())
        .unwrap_or_else(|e| {
          panic!("failed at parsing config at {config_path} | {e:#}")
        });
    let installations = match (env.monitor_github_webhook_app_installations_ids, env.monitor_github_webhook_app_installations_namespaces) {
      (Some(ids), Some(namespaces)) => {
        if ids.len() != namespaces.len() {
          panic!("MONITOR_GITHUB_WEBHOOK_APP_INSTALLATIONS_IDS length and MONITOR_GITHUB_WEBHOOK_APP_INSTALLATIONS_NAMESPACES length mismatch. Got {ids:?} and {namespaces:?}")
        }
        ids
          .into_iter()
          .zip(namespaces)
          .map(|(id, namespace)| GithubWebhookAppInstallationConfig {
            id,
            namespace
          })
          .collect()
      },
      (Some(_), None) | (None, Some(_)) => {
        panic!("Got only one of MONITOR_GITHUB_WEBHOOK_APP_INSTALLATIONS_IDS or MONITOR_GITHUB_WEBHOOK_APP_INSTALLATIONS_NAMESPACES, both MUST be provided");
      }
      (None, None) => {
        config.github_webhook_app.installations
      }
    };
    // recreating CoreConfig here makes sure we apply all env overrides.
    CoreConfig {
      title: env.monitor_title.unwrap_or(config.title),
      host: env.monitor_host.unwrap_or(config.host),
      port: env.monitor_port.unwrap_or(config.port),
      passkey: env.monitor_passkey.unwrap_or(config.passkey),
      jwt_secret: env.monitor_jwt_secret.unwrap_or(config.jwt_secret),
      jwt_valid_for: env
        .monitor_jwt_valid_for
        .unwrap_or(config.jwt_valid_for),
      sync_directory: env
        .monitor_sync_directory
        .map(|dir|
          dir.parse()
            .context("failed to parse env MONITOR_SYNC_DIRECTORY as valid path").unwrap())
        .unwrap_or(config.sync_directory),
      stack_directory: env
        .monitor_stack_directory
        .map(|dir|
          dir.parse()
            .context("failed to parse env MONITOR_STACK_DIRECTORY as valid path").unwrap())
        .unwrap_or(config.stack_directory),
      stack_poll_interval: env
        .monitor_stack_poll_interval
        .unwrap_or(config.stack_poll_interval),
      sync_poll_interval: env
        .monitor_sync_poll_interval
        .unwrap_or(config.sync_poll_interval),
      monitoring_interval: env
        .monitor_monitoring_interval
        .unwrap_or(config.monitoring_interval),
      keep_stats_for_days: env
        .monitor_keep_stats_for_days
        .unwrap_or(config.keep_stats_for_days),
      keep_alerts_for_days: env
        .monitor_keep_alerts_for_days
        .unwrap_or(config.keep_alerts_for_days),
      webhook_secret: env
        .monitor_webhook_secret
        .unwrap_or(config.webhook_secret),
      webhook_base_url: env
        .monitor_webhook_base_url
        .or(config.webhook_base_url),
      transparent_mode: env
        .monitor_transparent_mode
        .unwrap_or(config.transparent_mode),
      ui_write_disabled: env
        .monitor_ui_write_disabled
        .unwrap_or(config.ui_write_disabled),
      enable_new_users: env.monitor_enable_new_users
        .unwrap_or(config.enable_new_users),
      local_auth: env.monitor_local_auth.unwrap_or(config.local_auth),
      google_oauth: OauthCredentials {
        enabled: env
          .monitor_google_oauth_enabled
          .unwrap_or(config.google_oauth.enabled),
        id: env
          .monitor_google_oauth_id
          .unwrap_or(config.google_oauth.id),
        secret: env
          .monitor_google_oauth_secret
          .unwrap_or(config.google_oauth.secret),
      },
      github_oauth: OauthCredentials {
        enabled: env
          .monitor_github_oauth_enabled
          .unwrap_or(config.github_oauth.enabled),
        id: env
          .monitor_github_oauth_id
          .unwrap_or(config.github_oauth.id),
        secret: env
          .monitor_github_oauth_secret
          .unwrap_or(config.github_oauth.secret),
      },
      github_webhook_app: GithubWebhookAppConfig {
        app_id: env
          .monitor_github_webhook_app_app_id
          .unwrap_or(config.github_webhook_app.app_id),
        pk_path: env
          .monitor_github_webhook_app_pk_path
          .unwrap_or(config.github_webhook_app.pk_path),
        installations,
      },
      aws: AwsCredentials {
        access_key_id: env
          .monitor_aws_access_key_id
          .unwrap_or(config.aws.access_key_id),
        secret_access_key: env
          .monitor_aws_secret_access_key
          .unwrap_or(config.aws.secret_access_key),
      },
      hetzner: HetznerCredentials {
        token: env
          .monitor_hetzner_token
          .unwrap_or(config.hetzner.token),
      },
      mongo: MongoConfig {
        uri: env.monitor_mongo_uri.or(config.mongo.uri),
        address: env.monitor_mongo_address.or(config.mongo.address),
        username: env
          .monitor_mongo_username
          .or(config.mongo.username),
        password: env
          .monitor_mongo_password
          .or(config.mongo.password),
        app_name: env
          .monitor_mongo_app_name
          .unwrap_or(config.mongo.app_name),
        db_name: env
          .monitor_mongo_db_name
          .unwrap_or(config.mongo.db_name),
      },
      logging: LogConfig {
        level: env
          .monitor_logging_level
          .unwrap_or(config.logging.level),
        stdio: env
          .monitor_logging_stdio
          .unwrap_or(config.logging.stdio),
        otlp_endpoint: env
          .monitor_logging_otlp_endpoint
          .or(config.logging.otlp_endpoint),
        opentelemetry_service_name: env
          .monitor_logging_opentelemetry_service_name
          .unwrap_or(config.logging.opentelemetry_service_name),
      },

      // These can't be overridden on env
      secrets: config.secrets,
      git_providers: config.git_providers,
      docker_registries: config.docker_registries,
      aws_ecr_registries: config.aws_ecr_registries,
    }
  })
}
