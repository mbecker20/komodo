use bson::{doc, Document};
use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::Display;
use typeshare::typeshare;

use super::{
  resource::{Resource, ResourceListItem, ResourceQuery},
  EnvironmentVar,
};

#[typeshare]
pub type StackListItem = ResourceListItem<StackListItemInfo>;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackListItemInfo {
  /// The server that stack is deployed on.
  pub server_id: String,
  /// The git provider domain
  pub git_provider: String,
  /// The configured repo
  pub repo: String,
  /// The configured branch
  pub branch: String,
  /// The stack state
  pub state: StackState,
  /// The service names that are part of the stack
  pub services: Vec<String>,
  /// Latest short commit hash, or null.
  pub latest_hash: Option<String>,
  /// Latest commit message, or null.
  pub latest_message: Option<String>,
}

#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize, Display,
)]
pub enum StackState {
  /// The stack is deployed
  Healthy,
  /// Some containers are up, some are down.
  Unhealthy,
  /// The stack is not deployed
  Down,
  /// Last deploy failed
  Failed,
  /// Server not reachable
  #[default]
  Unknown,
}

#[typeshare]
pub type Stack = Resource<StackConfig, StackInfo>;

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StackInfo {
  /// The deployed compose file.
  pub deployed_contents: Option<String>,
  /// The service / container names.
  /// These only need to be matched against the start of the container name,
  /// since the name is postfixed with a number.
  ///
  /// This is updated whenever deployed contents is updated
  #[serde(default)]
  pub services: Vec<StackServiceNames>,

  /// Cached json representation of the compose file contents, info about a parsing failure, or empty string.
  /// Obtained by calling `docker compose config`. Will be of the latest config, not the deployed config.
  #[serde(default)]
  pub json: String,
  /// There was an error in calling `docker compose config`
  #[serde(default)]
  pub json_error: bool,

  // Only for repo based stacks.
  /// If using a repo based compose file, will cache the contents here
  /// for API delivery. Deploys will always pull directly from the repo.
  ///
  /// Could be:
  ///   - The file contents or null (remote_error: false)
  ///   - An error message (remote_error: true)
  pub remote_contents: Option<String>,
  /// There was an error in getting the remote contents.
  #[serde(default)]
  pub remote_error: bool,
  /// Deployed short commit hash, or empty string.
  pub deployed_hash: Option<String>,
  /// Deployed commit message, or null
  pub deployed_message: Option<String>,
  /// Latest commit hash, or null
  pub latest_hash: Option<String>,
  /// Latest commit message, or null
  pub latest_message: Option<String>,
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StackServiceNames {
  pub container_name: String,
  pub service_name: String,
}

#[typeshare(serialized_as = "Partial<StackConfig>")]
pub type _PartialStackConfig = PartialStackConfig;

/// The compose file configuration.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Partial)]
#[partial_derive(Debug, Clone, Default, Serialize, Deserialize)]
#[partial(skip_serializing_none, from, diff)]
pub struct StackConfig {
  /// The server to deploy the stack on.
  #[serde(default)]
  #[builder(default)]
  pub server_id: String,

  /// Used with `registry_account` to login to a registry before docker compose up.
  #[serde(default)]
  #[builder(default)]
  pub registry_provider: String,

  /// Used with `registry_provider` to login to a registry before docker compose up.
  #[serde(default)]
  #[builder(default)]
  pub registry_account: String,

  /// The extra arguments to pass after `docker compose up -d`.
  /// If empty, no extra arguments will be passed.
  #[serde(default)]
  #[builder(default)]
  pub extra_args: Vec<String>,

  /// The environment variables passed to the compose file.
  /// They will be written to local '.env'
  #[serde(
    default,
    deserialize_with = "super::env_vars_deserializer"
  )]
  #[partial_attr(serde(
    default,
    deserialize_with = "super::option_env_vars_deserializer"
  ))]
  #[builder(default)]
  pub environment: Vec<EnvironmentVar>,

  /// The name of the written environment file before `docker compose up`.
  /// Default: .env
  #[serde(default = "default_env_file_name")]
  #[builder(default = "default_env_file_name()")]
  #[partial_default(default_env_file_name())]
  pub env_file_name: String,

  /// The contents of the file directly, for management in the UI.
  /// If this is empty, it will fall back to checking git config for
  /// repo based compose file.
  #[serde(default)]
  #[builder(default)]
  pub file_contents: String,

  /// The git provider domain. Default: github.com
  #[serde(default = "default_git_provider")]
  #[builder(default = "default_git_provider()")]
  #[partial_default(default_git_provider())]
  pub git_provider: String,

  /// Whether to use https to clone the repo (versus http). Default: true
  ///
  /// Note. Monitor does not currently support cloning repos via ssh.
  #[serde(default = "default_git_https")]
  #[builder(default = "default_git_https()")]
  #[partial_default(default_git_https())]
  pub git_https: bool,

  /// The Github repo used as the source of the build.
  #[serde(default)]
  #[builder(default)]
  pub repo: String,

  /// The branch of the repo.
  #[serde(default = "default_branch")]
  #[builder(default = "default_branch()")]
  #[partial_default(default_branch())]
  pub branch: String,

  /// Optionally set a specific commit hash.
  #[serde(default)]
  #[builder(default)]
  pub commit: String,

  /// The git account used to access private repos.
  /// Passing empty string can only clone public repos.
  ///
  /// Note. A token for the account must be available in the core config or the builder server's periphery config
  /// for the configured git provider.
  #[serde(default)]
  #[builder(default)]
  pub git_account: String,

  /// Directory to change to (`cd`) before running `docker compose up -d`.
  /// Default: `.` (the repo root)
  #[serde(default = "default_run_directory")]
  #[builder(default = "default_run_directory()")]
  #[partial_default(default_run_directory())]
  pub run_directory: String,

  /// The path of the compose file, relative to the run path.
  /// Default: `compose.yaml`
  #[serde(default = "default_file_path")]
  #[builder(default = "default_file_path()")]
  #[partial_default(default_file_path())]
  pub file_path: String,

  /// Whether incoming webhooks actually trigger action.
  #[serde(default = "default_webhook_enabled")]
  #[builder(default = "default_webhook_enabled()")]
  #[partial_default(default_webhook_enabled())]
  pub webhook_enabled: bool,
}

impl StackConfig {
  pub fn builder() -> StackConfigBuilder {
    StackConfigBuilder::default()
  }
}

fn default_env_file_name() -> String {
  String::from(".env")
}

fn default_git_provider() -> String {
  String::from("github.com")
}

fn default_git_https() -> bool {
  true
}

fn default_branch() -> String {
  String::from("main")
}

fn default_run_directory() -> String {
  String::from(".")
}

fn default_file_path() -> String {
  String::from("compose.yaml")
}

fn default_webhook_enabled() -> bool {
  true
}

impl Default for StackConfig {
  fn default() -> Self {
    Self {
      server_id: Default::default(),
      registry_provider: Default::default(),
      registry_account: Default::default(),
      file_contents: Default::default(),
      extra_args: Default::default(),
      environment: Default::default(),
      env_file_name: default_env_file_name(),
      git_provider: default_git_provider(),
      git_https: default_git_https(),
      repo: Default::default(),
      branch: default_branch(),
      commit: Default::default(),
      git_account: Default::default(),
      run_directory: default_run_directory(),
      file_path: default_file_path(),
      webhook_enabled: default_webhook_enabled(),
    }
  }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct StackActionState {
  pub deploying: bool,
  pub starting: bool,
  pub restarting: bool,
  pub pausing: bool,
  pub stopping: bool,
  pub destroying: bool,
}

#[typeshare]
pub type StackQuery = ResourceQuery<StackQuerySpecifics>;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct StackQuerySpecifics {
  /// Filter syncs by their repo.
  pub repos: Vec<String>,
}

impl super::resource::AddFilters for StackQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    if !self.repos.is_empty() {
      filters.insert("config.repo", doc! { "$in": &self.repos });
    }
  }
}
