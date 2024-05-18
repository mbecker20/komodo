use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{MongoId, I64};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(
  feature = "mongo",
  derive(mongo_indexed::derive::MongoIndexed)
)]
#[cfg_attr(feature = "mongo", doc_index({ "config.type": 1 }))]
#[cfg_attr(feature = "mongo", sparse_doc_index({ "config.data.google_id": 1 }))]
#[cfg_attr(feature = "mongo", sparse_doc_index({ "config.data.github_id": 1 }))]
pub struct User {
  /// The Mongo ID of the User.
  /// This field is de/serialized from/to JSON as
  /// `{ "_id": { "$oid": "..." }, ...(rest of serialized User) }`
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "bson::serde_helpers::hex_string_as_object_id"
  )]
  pub id: MongoId,

  /// The globally unique username for the user.
  #[cfg_attr(feature = "mongo", unique_index)]
  pub username: String,

  /// Whether user is enabled / able to access the api.
  #[cfg_attr(feature = "mongo", index)]
  #[serde(default)]
  pub enabled: bool,

  /// Whether the user has global admin permissions.
  #[serde(default)]
  pub admin: bool,

  /// Whether the user has permission to create servers.
  #[serde(default)]
  pub create_server_permissions: bool,

  /// Whether the user has permission to create builds
  #[serde(default)]
  pub create_build_permissions: bool,

  /// The user-type specific config.
  pub config: UserConfig,

  /// When the user last opened updates dropdown.
  #[serde(default)]
  pub last_update_view: I64,

  /// Recently viewed server ids
  #[serde(default)]
  pub recent_servers: Vec<String>,

  /// Recently viewed deployment ids
  #[serde(default)]
  pub recent_deployments: Vec<String>,

  /// Recently viewed build ids
  #[serde(default)]
  pub recent_builds: Vec<String>,

  /// Recently viewed repo ids
  #[serde(default)]
  pub recent_repos: Vec<String>,

  /// Recently viewed procedure ids
  #[serde(default)]
  pub recent_procedures: Vec<String>,

  #[serde(default)]
  pub updated_at: I64,
}

impl User {
  /// Prepares user object for transport by removing any sensitive fields
  pub fn sanitize(&mut self) {
    if let UserConfig::Local { .. } = &self.config {
      self.config = UserConfig::default();
    }
  }

  pub fn is_service_user(user_id: &str) -> bool {
    matches!(user_id, "Procedure" | "Github" | "Auto Redeploy")
  }
}

pub fn admin_service_user(user_id: &str) -> Option<User> {
  match user_id {
    "Procedure" => procedure_user().to_owned().into(),
    "Github" => github_user().to_owned().into(),
    "Auto Redeploy" => auto_redeploy_user().to_owned().into(),
    _ => None,
  }
}

pub fn procedure_user() -> &'static User {
  static PROCEDURE_USER: OnceLock<User> = OnceLock::new();
  PROCEDURE_USER.get_or_init(|| {
    let id_name = String::from("Procedure");
    User {
      id: id_name.clone(),
      username: id_name,
      admin: true,
      ..Default::default()
    }
  })
}

pub fn github_user() -> &'static User {
  static PROCEDURE_USER: OnceLock<User> = OnceLock::new();
  PROCEDURE_USER.get_or_init(|| {
    let id_name = String::from("Github");
    User {
      id: id_name.clone(),
      username: id_name,
      admin: true,
      ..Default::default()
    }
  })
}

pub fn auto_redeploy_user() -> &'static User {
  static AUTO_REDEPLOY_USER: OnceLock<User> = OnceLock::new();
  AUTO_REDEPLOY_USER.get_or_init(|| {
    let id_name = String::from("Auto Redeploy");
    User {
      id: id_name.clone(),
      username: id_name,
      admin: true,
      ..Default::default()
    }
  })
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum UserConfig {
  /// User that logs in with username / password
  Local { password: String },

  /// User that logs in via Google Oauth
  Google { google_id: String, avatar: String },

  /// User that logs in via Github Oauth
  Github { github_id: String, avatar: String },

  /// Non-human managed user, can have it's own permissions / api keys
  Service { description: String },
}

impl Default for UserConfig {
  fn default() -> Self {
    Self::Local {
      password: String::new(),
    }
  }
}
