use derive_builder::Builder;
use derive_variants::EnumVariants;
use mungos::derive::MungosIndexed;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use super::resource::{Resource, ResourceListItem};

#[typeshare]
pub type Alerter = Resource<AlerterConfig, AlerterInfo>;

#[typeshare]
pub type AlerterListItem = ResourceListItem<AlerterListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlerterListItemInfo {
    pub is_default: bool,
    pub alerter_type: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AlerterInfo {
    #[serde(default)]
    pub is_default: bool,
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
