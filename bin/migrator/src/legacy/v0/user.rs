use anyhow::anyhow;
use monitor_client::entities::user::UserConfig;
use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

use super::unix_from_monitor_ts;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: String,

  pub username: String,

  #[serde(default)]
  pub enabled: bool,

  #[serde(default)]
  pub admin: bool,

  #[serde(default)]
  pub create_server_permissions: bool,

  #[serde(default)]
  pub create_build_permissions: bool,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub avatar: Option<String>,

  // used with auth
  #[serde(default)]
  pub secrets: Vec<ApiSecret>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub github_id: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub google_id: Option<String>,

  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub created_at: String,
  #[serde(default)]
  pub updated_at: String,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct ApiSecret {
  pub name: String,
  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub hash: String,
  pub created_at: String,
  pub expires: Option<String>,
}

// impl TryFrom<ApiSecret>
//   for monitor_client::entities::user::ApiSecret
// {
//   type Error = anyhow::Error;
//   fn try_from(value: ApiSecret) -> Result<Self, Self::Error> {
//     let secret = Self {
//       name: value.name,
//       hash: value.hash,
//       created_at: unix_from_monitor_ts(&value.created_at)?,
//       expires: value
//         .expires
//         .and_then(|exp| unix_from_monitor_ts(&exp).ok()),
//     };
//     Ok(secret)
//   }
// }

impl TryFrom<User> for monitor_client::entities::user::User {
  type Error = anyhow::Error;
  fn try_from(value: User) -> Result<Self, Self::Error> {
    let config =
      match (value.password, value.github_id, value.google_id) {
        (Some(password), _, _) => UserConfig::Local { password },
        (None, Some(github_id), _) => UserConfig::Github {
          github_id,
          avatar: value.avatar.unwrap_or_default(),
        },
        (None, None, Some(google_id)) => UserConfig::Google {
          google_id,
          avatar: value.avatar.unwrap_or_default(),
        },
        _ => {
          return Err(anyhow!("user is not local, github, or google"))
        }
      };
    let user = Self {
      config,
      id: value.id,
      username: value.username,
      enabled: value.enabled,
      admin: value.admin,
      create_server_permissions: value.create_server_permissions,
      create_build_permissions: value.create_build_permissions,
      last_update_view: Default::default(),
      recently_viewed: Default::default(),
      updated_at: unix_from_monitor_ts(&value.updated_at)?,
    };
    Ok(user)
  }
}
