use derive_builder::Builder;
use derive_variants::EnumVariants;
use mungos::{mongodb::bson::serde_helpers::hex_string_as_object_id, MungosIndexed};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use crate::{i64_is_zero, MongoId, I64};

use super::PermissionsMap;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, MungosIndexed)]
pub struct Alerter {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[builder(setter(skip))]
    pub id: MongoId,

    #[unique_index]
    pub name: String,

    #[serde(default)]
    #[builder(default)]
    pub description: String,

    #[serde(default)]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[serde(default, skip_serializing_if = "i64_is_zero")]
    #[builder(setter(skip))]
    pub created_at: I64,

    #[serde(default)]
    #[builder(setter(skip))]
    pub updated_at: I64,

    #[serde(default)]
    #[builder(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    #[builder(default)]
    pub default_alerter: bool,

    pub config: AlerterConfig,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, MungosIndexed, EnumVariants)]
#[variant_derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString)]
#[serde(tag = "type", content = "params")]
pub enum AlerterConfig {
    Custom(CustomAlerterConfig),
    Slack(SlackAlerterConfig),
}

#[typeshare(serialized_as = "Partial<CustomAlerterConfig>")]
pub type _PartialCustomAlerterConfig = PartialCustomAlerterConfig;

#[typeshare(serialized_as = "Partial<SlackAlerterConfig>")]
pub type _PartialSlackAlerterConfig = PartialSlackAlerterConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, MungosIndexed, EnumVariants)]
#[variant_derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString)]
#[serde(tag = "type", content = "params")]
pub enum PartialAlerterConfig {
    Custom(_PartialCustomAlerterConfig),
    Slack(_PartialSlackAlerterConfig),
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[skip_serializing_none]
pub struct CustomAlerterConfig {
    pub url: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[skip_serializing_none]
pub struct SlackAlerterConfig {
    pub url: String,
}
