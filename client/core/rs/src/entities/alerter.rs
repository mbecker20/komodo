use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use derive_variants::EnumVariants;
use mungos::mongodb::bson::{doc, Document};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};
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
  Custom(CustomAlerterConfig),
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
