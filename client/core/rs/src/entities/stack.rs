use std::collections::HashMap;

use bson::{doc, Document};
use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::Display;
use typeshare::typeshare;

use super::{
  deployment::ContainerSummary,
  resource::{Resource, ResourceListItem, ResourceQuery},
  to_monitor_name, EnvironmentVar,
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
  /// Whether the compose file is missing on the host.
  /// If true, this is an unhealthy state.
  pub file_missing: bool,
  /// Whether the compose project is missing on the host.
  /// Ie, it does not show up in `docker compose ls`.
  /// If true, and the stack is not Down, this is an unhealthy state.
  pub project_missing: bool,
  /// Latest short commit hash, or null.
  pub latest_hash: Option<String>,
  /// Latest commit message, or null.
  pub latest_message: Option<String>,
  /// Deployed short commit hash, or null.
  pub deployed_hash: Option<String>,
  /// Deployed commit message, or null.
  pub deployed_message: Option<String>,
}

#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize, Display,
)]
pub enum StackState {
  /// All containers are running.
  Running,
  /// All containers are paused
  Paused,
  /// All contianers are stopped
  Stopped,
  /// All containers are restarting
  Restarting,
  /// All containers are dead
  Dead,
  /// The containers are in a mix of states
  Unhealthy,
  /// The stack is not deployed
  Down,
  /// Server not reachable
  #[default]
  Unknown,
}

#[typeshare]
pub type Stack = Resource<StackConfig, StackInfo>;

impl Stack {
  /// If fresh is passed, it will bypass the deployed project name.
  /// and get the most up to date one from just project_name field falling back to stack name.
  pub fn project_name(&self, fresh: bool) -> String {
    if !fresh {
      if let Some(project_name) = &self.info.deployed_project_name {
        return project_name.clone();
      }
    }
    self
      .config
      .project_name
      .is_empty()
      .then(|| to_monitor_name(&self.name))
      .unwrap_or_else(|| to_monitor_name(&self.config.project_name))
  }
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StackInfo {
  /// Whether the compose file is missing on the target host.
  /// Monitor will have to redeploy the stack to fix this.
  #[serde(default)]
  pub file_missing: bool,
  /// Whether the compose project is missing on the target host.
  /// Ensure the stack project_name is correctly configured if this is true,
  /// but the stack is definitely running.
  #[serde(default)]
  pub project_missing: bool,

  /// The deployed project name.
  /// This is updated whenever Monitor successfully deploys the stack.
  /// If it is present, Monitor will use it for actions over other options,
  /// to ensure control is maintained after changing the project name (there is no rename compose project api).
  pub deployed_project_name: Option<String>,
  /// The deployed compose file.This is updated whenever Monitor successfully deploys the stack.
  pub deployed_contents: Option<String>,
  /// Deployed short commit hash, or null. Only for repo based stacks.
  pub deployed_hash: Option<String>,
  /// Deployed commit message, or null. Only for repo based stacks
  pub deployed_message: Option<String>,
  /// Cached json representation of the deployed compose file contents
  /// Obtained by calling `docker compose config`. Will be of the deployed config if it exists.
  pub deployed_json: Option<String>,
  /// If there was an error in calling `docker compose config`, the message will be here.
  pub deployed_json_error: Option<String>,
  /// The service names.
  /// This is updated whenever it is empty, or deployed contents is updated.
  #[serde(default)]
  pub services: Vec<StackServiceNames>,

  /// Cached json representation of the compose file contents.
  /// Obtained by calling `docker compose config`. Will be of the latest config, not the deployed config.
  pub latest_json: Option<String>,
  /// If there was an error in calling `docker compose config` on the latest contents, the message will be here
  pub latest_json_error: Option<String>,

  // Only for repo based stacks.
  /// If using a repo based compose file, will cache the contents here for API delivery.
  pub remote_contents: Option<String>,
  /// If there was an error in getting the remote contents, it will be here.
  pub remote_error: Option<String>,

  /// Latest commit hash, or null
  pub latest_hash: Option<String>,
  /// Latest commit message, or null
  pub latest_message: Option<String>,
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StackServiceNames {
  /// The name of the service
  pub service_name: String,
  /// Will either be the declared container_name in the compose file,
  /// or a pattern to match auto named containers.
  ///
  /// Auto named containers are composed of three parts:
  ///
  /// 1. The name of the compose project (top level name field of compose file).
  ///    This defaults to the name of the parent folder of the compose file.
  ///    Monitor will always set it to be the name of the stack, but imported stacks
  ///    will have a different name.
  /// 2. The service name
  /// 3. The replica number
  ///
  /// Example: stacko-mongo-1.
  ///
  /// This stores only 1. and 2., ie stacko-mongo.
  /// Containers will be matched via regex like `^container_name-?[0-9]*$``
  pub container_name: String,
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StackService {
  /// The service name
  pub service: String,
  /// The container
  pub container: Option<ContainerSummary>,
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

  /// Optionally specify a custom project name for the stack.
  /// If this is empty string, it will default to the stack name.
  /// Used with `docker compose -p {project_name}`.
  ///
  /// Note. Can be used to import pre-existing stacks.
  #[serde(default)]
  #[builder(default)]
  pub project_name: String,

  /// Directory to change to (`cd`) before running `docker compose up -d`.
  /// Default: `./` (the repo root)
  #[serde(default = "default_run_directory")]
  #[builder(default = "default_run_directory()")]
  #[partial_default(default_run_directory())]
  pub run_directory: String,

  /// The path of the compose file, relative to the run path.
  /// If compose file defined locally in `file_contents`, this will always be `compose.yaml`.
  /// Default: `compose.yaml`
  #[serde(default = "default_file_path")]
  #[builder(default = "default_file_path()")]
  #[partial_default(default_file_path())]
  pub file_path: String,

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
  /// They will be written to path defined in env_file_path,
  /// which is given relative to the run directory.
  ///
  /// If it is empty, no file will be written.
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
  /// Relative to the repo root.
  /// Default: .env
  #[serde(default = "default_env_file_path")]
  #[builder(default = "default_env_file_path()")]
  #[partial_default(default_env_file_path())]
  pub env_file_path: String,

  /// Whether to skip secret interpolation into the stack environment variables.
  #[serde(default)]
  #[builder(default)]
  pub skip_secret_interp: bool,

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

fn default_env_file_path() -> String {
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
  String::from("./")
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
      project_name: Default::default(),
      run_directory: default_run_directory(),
      file_path: default_file_path(),
      registry_provider: Default::default(),
      registry_account: Default::default(),
      file_contents: Default::default(),
      extra_args: Default::default(),
      environment: Default::default(),
      env_file_path: default_env_file_path(),
      skip_secret_interp: Default::default(),
      git_provider: default_git_provider(),
      git_https: default_git_https(),
      repo: Default::default(),
      branch: default_branch(),
      commit: Default::default(),
      git_account: Default::default(),
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
  pub unpausing: bool,
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

/// Keeping this minimal for now as its only needed to parse the service names / container names
#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComposeFile {
  /// If not provided, will default to the parent folder holding the compose file.
  pub name: Option<String>,
  #[serde(default)]
  pub services: HashMap<String, ComposeService>,
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComposeService {
  pub image: Option<String>,
  pub container_name: Option<String>,
  pub deploy: Option<ComposeServiceDeploy>,
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComposeServiceDeploy {
  pub replicas: u8,
}
