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

  pub fn admin_service_user(id_name: impl Into<String>) -> User {
    let id_name: String = id_name.into();
    User {
      id: id_name.clone(),
      username: id_name,
      admin: true,
      create_build_permissions: true,
      create_server_permissions: true,
      ..Default::default()
    }
  }
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
