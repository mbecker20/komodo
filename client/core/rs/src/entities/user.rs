use std::sync::OnceLock;

use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::{
  doc, serde_helpers::hex_string_as_object_id, Document,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{MongoId, I64};

use super::update::ResourceTarget;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, MongoIndexed,
)]
#[doc_index(doc! { "config.type": 1 })]
#[sparse_doc_index(doc! { "config.data.google_id": 1 })]
#[sparse_doc_index(doc! { "config.data.github_id": 1 })]
pub struct User {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: MongoId,

  #[unique_index]
  pub username: String,

  #[index]
  #[serde(default)]
  pub enabled: bool,

  #[serde(default)]
  pub admin: bool,

  #[serde(default)]
  pub create_server_permissions: bool,

  #[serde(default)]
  pub create_build_permissions: bool,

  pub config: UserConfig,

  #[serde(default)]
  pub last_update_view: I64,

  #[serde(default)]
  pub recently_viewed: Vec<ResourceTarget>,

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
    matches!(user_id, "Procedure" | "Github")
  }
}

pub fn admin_service_user(user_id: &str) -> Option<User> {
  match user_id {
    "Procedure" => procedure_user().to_owned().into(),
    "Github" => github_user().to_owned().into(),
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
