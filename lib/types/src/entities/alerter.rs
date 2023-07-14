use derive_builder::Builder;
use derive_variants::EnumVariants;
use mungos::{mongodb::bson::serde_helpers::hex_string_as_object_id, derive::{MungosIndexed, StringObjectId}};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use crate::{MongoId, I64};

use super::PermissionsMap;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, MungosIndexed, StringObjectId)]
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

    #[serde(default)]
    #[builder(setter(skip))]
    pub updated_at: I64,

    #[serde(default)]
    #[builder(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    #[builder(default)]
    pub is_default: bool,

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

impl From<PartialAlerterConfig> for AlerterConfig {
    fn from(value: PartialAlerterConfig) -> AlerterConfig {
        match value {
            PartialAlerterConfig::Custom(config) => AlerterConfig::Custom(config.into()),
            PartialAlerterConfig::Slack(config) => AlerterConfig::Slack(config.into()),
        }
    }
}

impl AlerterConfig {
    pub fn merge_partial(self, partial: PartialAlerterConfig) -> AlerterConfig {
        match partial {
            PartialAlerterConfig::Custom(partial) => match self {
                AlerterConfig::Custom(config) => {
                    let config = CustomAlerterConfig {
                        url: partial.url.unwrap_or(config.url),
                    };
                    AlerterConfig::Custom(config)
                }
                _ => AlerterConfig::Custom(partial.into()),
            },
            PartialAlerterConfig::Slack(partial) => match self {
                AlerterConfig::Slack(config) => {
                    let config = SlackAlerterConfig {
                        url: partial.url.unwrap_or(config.url),
                    };
                    AlerterConfig::Slack(config)
                }
                _ => AlerterConfig::Slack(partial.into()),
            },
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
pub struct CustomAlerterConfig {
    #[partial_default(String::from("http://localhost:7000"))]
    pub url: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
pub struct SlackAlerterConfig {
    #[partial_default(String::from(
        "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX",
    ))]
    pub url: String,
}
