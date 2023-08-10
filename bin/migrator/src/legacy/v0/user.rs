use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ApiSecret {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub hash: String,
    pub created_at: String,
    pub expires: Option<String>,
}
