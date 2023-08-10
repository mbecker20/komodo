use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

use super::{Command, PermissionsMap};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Action {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    pub id: String,

    pub name: String,

    #[serde(default)]
    pub description: String,

    pub path: String,

    pub command: String,

    // run action on all servers in this array
    #[serde(default)]
    pub server_ids: Vec<String>,

    // run action on all servers in these groups
    #[serde(default)]
    pub group_ids: Vec<String>,

    // run action on all servers
    #[serde(default)]
    pub run_on_all: bool,

    #[serde(default)]
    pub permissions: PermissionsMap,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

impl From<Action> for Command {
    fn from(value: Action) -> Command {
        Command {
            path: value.path,
            command: value.command,
        }
    }
}
