use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use derive_variants::EnumVariants;
use mungos::mongodb::bson::{doc, Document};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

use super::resource::{
  AddFilters, Resource, ResourceListItem, ResourceQuery,
};

#[typeshare]
pub type Alerter = Resource<AlerterConfig, AlerterInfo>;

#[typeshare]
pub type AlerterListItem = ResourceListItem<AlerterListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlerterListItemInfo {
  /// Whether alerter is enabled for sending alerts
  pub enabled: bool,
  /// Whether the alerter is the default
  pub is_default: bool,
  /// The type of the alerter, eg. Slack, Custom
  pub alerter_type: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AlerterInfo {
  #[serde(default)]
  pub is_default: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumVariants)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  Display,
  EnumString,
  AsRefStr
)]
#[serde(tag = "type", content = "params")]
pub enum AlerterConfig {
  /// Send alert serialized to JSON to an http endpoint.
  Custom(CustomAlerterConfig),

  /// Send alert to a slack app
  Slack(SlackAlerterConfig),
}

#[typeshare(serialized_as = "Partial<CustomAlerterConfig>")]
pub type _PartialCustomAlerterConfig = PartialCustomAlerterConfig;

#[typeshare(serialized_as = "Partial<SlackAlerterConfig>")]
pub type _PartialSlackAlerterConfig = PartialSlackAlerterConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumVariants)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  Display,
  EnumString,
  AsRefStr
)]
#[serde(tag = "type", content = "params")]
pub enum PartialAlerterConfig {
  Custom(_PartialCustomAlerterConfig),
  Slack(_PartialSlackAlerterConfig),
}

impl From<PartialAlerterConfig> for AlerterConfig {
  fn from(value: PartialAlerterConfig) -> AlerterConfig {
    match value {
      PartialAlerterConfig::Custom(config) => {
        AlerterConfig::Custom(config.into())
      }
      PartialAlerterConfig::Slack(config) => {
        AlerterConfig::Slack(config.into())
      }
    }
  }
}

impl From<AlerterConfig> for PartialAlerterConfig {
  fn from(value: AlerterConfig) -> Self {
    match value {
      AlerterConfig::Custom(config) => {
        PartialAlerterConfig::Custom(config.into())
      }
      AlerterConfig::Slack(config) => {
        PartialAlerterConfig::Slack(config.into())
      }
    }
  }
}

impl AlerterConfig {
  pub fn merge_partial(
    self,
    partial: PartialAlerterConfig,
  ) -> AlerterConfig {
    match partial {
      PartialAlerterConfig::Custom(partial) => match self {
        AlerterConfig::Custom(config) => {
          let config = CustomAlerterConfig {
            url: partial.url.unwrap_or(config.url),
            enabled: partial.enabled.unwrap_or(config.enabled),
          };
          AlerterConfig::Custom(config)
        }
        _ => AlerterConfig::Custom(partial.into()),
      },
      PartialAlerterConfig::Slack(partial) => match self {
        AlerterConfig::Slack(config) => {
          let config = SlackAlerterConfig {
            url: partial.url.unwrap_or(config.url),
            enabled: partial.enabled.unwrap_or(config.enabled),
          };
          AlerterConfig::Slack(config)
        }
        _ => AlerterConfig::Slack(partial.into()),
      },
    }
  }
}

/// Configuration for a custom alerter.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from)]
pub struct CustomAlerterConfig {
  /// The http/s endpoint to send the POST to
  #[partial_default(String::from("http://localhost:7000"))]
  pub url: String,
  /// Whether the alerter is enabled
  #[serde(default)]
  pub enabled: bool,
}

/// Configuration for a slack alerter.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from)]
pub struct SlackAlerterConfig {
  /// The slack app url
  #[partial_default(String::from(
    "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX",
  ))]
  pub url: String,
  /// Whether the alerter is enabled
  #[serde(default)]
  pub enabled: bool,
}

#[typeshare]
pub type AlerterQuery = ResourceQuery<AlerterQuerySpecifics>;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct AlerterQuerySpecifics {
  pub types: Vec<AlerterConfigVariant>,
}

impl AddFilters for AlerterQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    let types =
      self.types.iter().map(|t| t.as_ref()).collect::<Vec<_>>();
    if !self.types.is_empty() {
      filters.insert("config.type", doc! { "$in": types });
    }
  }
}
