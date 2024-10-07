use std::{collections::HashMap, sync::OnceLock};

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{MongoId, I64};

use super::{permission::PermissionLevel, ResourceTargetVariant};

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
  /// `{ "_id": { "$oid": "..." }, ...(rest of User schema) }`
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

  /// Can give / take other users admin priviledges.
  #[serde(default)]
  pub super_admin: bool,

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

  /// Recently viewed ids
  #[serde(default)]
  pub recents: HashMap<ResourceTargetVariant, Vec<String>>,

  /// Give the user elevated permissions on all resources of a certain type
  #[serde(default)]
  pub all: HashMap<ResourceTargetVariant, PermissionLevel>,

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

  /// Returns whether user is an inbuilt service user
  ///
  /// NOTE: ALSO UPDATE `frontend/src/lib/utils/is_service_user` to match
  pub fn is_service_user(user_id: &str) -> bool {
    matches!(
      user_id,
      "System"
        | "Procedure"
        | "Github" // Github can be removed later, just keeping for backward compat.
        | "Git Webhook"
        | "Auto Redeploy"
        | "Resource Sync"
        | "Stack Wizard"
        | "Build Manager"
        | "Repo Manager"
    )
  }
}

pub fn admin_service_user(user_id: &str) -> Option<User> {
  match user_id {
    "System" => system_user().to_owned().into(),
    "Procedure" => procedure_user().to_owned().into(),
    // Github should be removed later, replaced by Git Webhook, just keeping for backward compat.
    "Github" => git_webhook_user().to_owned().into(),
    "Git Webhook" => git_webhook_user().to_owned().into(),
    "Auto Redeploy" => auto_redeploy_user().to_owned().into(),
    "Resource Sync" => sync_user().to_owned().into(),
    "Stack Wizard" => stack_user().to_owned().into(),
    "Build Manager" => build_user().to_owned().into(),
    "Repo Manager" => repo_user().to_owned().into(),
    _ => None,
  }
}

pub fn system_user() -> &'static User {
  static SYSTEM_USER: OnceLock<User> = OnceLock::new();
  SYSTEM_USER.get_or_init(|| {
    let id_name = String::from("System");
    User {
      id: id_name.clone(),
      username: id_name,
      admin: true,
      ..Default::default()
    }
  })
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

pub fn git_webhook_user() -> &'static User {
  static GIT_WEBHOOK_USER: OnceLock<User> = OnceLock::new();
  GIT_WEBHOOK_USER.get_or_init(|| {
    let id_name = String::from("Git Webhook");
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

pub fn sync_user() -> &'static User {
  static SYNC_USER: OnceLock<User> = OnceLock::new();
  SYNC_USER.get_or_init(|| {
    let id_name = String::from("Resource Sync");
    User {
      id: id_name.clone(),
      username: id_name,
      admin: true,
      ..Default::default()
    }
  })
}

pub fn stack_user() -> &'static User {
  static STACK_USER: OnceLock<User> = OnceLock::new();
  STACK_USER.get_or_init(|| {
    let id_name = String::from("Stack Wizard");
    User {
      id: id_name.clone(),
      username: id_name,
      admin: true,
      ..Default::default()
    }
  })
}

pub fn build_user() -> &'static User {
  static BUILD_USER: OnceLock<User> = OnceLock::new();
  BUILD_USER.get_or_init(|| {
    let id_name = String::from("Build Manager");
    User {
      id: id_name.clone(),
      username: id_name,
      admin: true,
      ..Default::default()
    }
  })
}

pub fn repo_user() -> &'static User {
  static REPO_USER: OnceLock<User> = OnceLock::new();
  REPO_USER.get_or_init(|| {
    let id_name = String::from("Repo Manager");
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

  /// User that logs in via Oidc provider
  Oidc { provider: String, user_id: String },

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
