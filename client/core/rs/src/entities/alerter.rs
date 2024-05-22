use bson::{doc, Document};
use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use derive_variants::EnumVariants;
use partial_derive2::{Diff, MaybeNone, Partial, PartialDiff};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

use super::{
  resource::{Resource, ResourceListItem, ResourceQuery},
  MergePartial,
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

impl MaybeNone for PartialAlerterConfig {
  fn is_none(&self) -> bool {
    match self {
      PartialAlerterConfig::Custom(config) => config.is_none(),
      PartialAlerterConfig::Slack(config) => config.is_none(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlerterConfigDiff {
  Custom(CustomAlerterConfigDiff),
  Slack(SlackAlerterConfigDiff),
}

impl From<AlerterConfigDiff> for PartialAlerterConfig {
  fn from(value: AlerterConfigDiff) -> Self {
    match value {
      AlerterConfigDiff::Custom(diff) => {
        PartialAlerterConfig::Custom(diff.into())
      }
      AlerterConfigDiff::Slack(diff) => {
        PartialAlerterConfig::Slack(diff.into())
      }
    }
  }
}

impl Diff for AlerterConfigDiff {
  fn iter_field_diffs(
    &self,
  ) -> impl Iterator<Item = partial_derive2::FieldDiff> {
    match self {
      AlerterConfigDiff::Custom(diff) => {
        diff.iter_field_diffs().collect::<Vec<_>>().into_iter()
      }
      AlerterConfigDiff::Slack(diff) => {
        diff.iter_field_diffs().collect::<Vec<_>>().into_iter()
      }
    }
  }
}

impl PartialDiff<PartialAlerterConfig, AlerterConfigDiff>
  for AlerterConfig
{
  fn partial_diff(
    &self,
    partial: PartialAlerterConfig,
  ) -> AlerterConfigDiff {
    match self {
      AlerterConfig::Custom(original) => match partial {
        PartialAlerterConfig::Custom(partial) => {
          AlerterConfigDiff::Custom(original.partial_diff(partial))
        }
        PartialAlerterConfig::Slack(partial) => {
          let full: SlackAlerterConfig = partial.into();
          AlerterConfigDiff::Slack(SlackAlerterConfigDiff {
            url: Some((original.url.clone(), full.url)),
            enabled: Some((original.enabled, full.enabled)),
          })
        }
      },
      AlerterConfig::Slack(original) => match partial {
        PartialAlerterConfig::Slack(partial) => {
          AlerterConfigDiff::Slack(original.partial_diff(partial))
        }
        PartialAlerterConfig::Custom(partial) => {
          let full: CustomAlerterConfig = partial.into();
          AlerterConfigDiff::Custom(CustomAlerterConfigDiff {
            url: Some((original.url.clone(), full.url)),
            enabled: Some((original.enabled, full.enabled)),
          })
        }
      },
    }
  }
}

impl MaybeNone for AlerterConfigDiff {
  fn is_none(&self) -> bool {
    match self {
      AlerterConfigDiff::Custom(config) => config.is_none(),
      AlerterConfigDiff::Slack(config) => config.is_none(),
    }
  }
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

impl MergePartial for AlerterConfig {
  type Partial = PartialAlerterConfig;
  fn merge_partial(
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

#[typeshare(serialized_as = "Partial<CustomAlerterConfig>")]
pub type _PartialCustomAlerterConfig = PartialCustomAlerterConfig;

/// Configuration for a custom alerter.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from, diff)]
pub struct CustomAlerterConfig {
  /// The http/s endpoint to send the POST to
  #[serde(default = "default_custom_url")]
  #[builder(default = "default_custom_url()")]
  #[partial_default(default_custom_url())]
  pub url: String,

  /// Whether the alerter is enabled
  #[serde(default)]
  #[builder(default)]
  pub enabled: bool,
}

fn default_custom_url() -> String {
  String::from("http://localhost:7000")
}

#[typeshare(serialized_as = "Partial<SlackAlerterConfig>")]
pub type _PartialSlackAlerterConfig = PartialSlackAlerterConfig;

/// Configuration for a slack alerter.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from, diff)]
pub struct SlackAlerterConfig {
  /// The slack app url
  #[serde(default = "default_slack_url")]
  #[builder(default = "default_slack_url()")]
  #[partial_default(default_slack_url())]
  pub url: String,

  /// Whether the alerter is enabled
  #[serde(default)]
  #[builder(default)]
  pub enabled: bool,
}

impl Default for SlackAlerterConfig {
  fn default() -> Self {
    Self {
      url: default_slack_url(),
      enabled: false,
    }
  }
}

fn default_slack_url() -> String {
  String::from(
    "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX",
  )
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

impl super::resource::AddFilters for AlerterQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    let types =
      self.types.iter().map(|t| t.as_ref()).collect::<Vec<_>>();
    if !self.types.is_empty() {
      filters.insert("config.type", doc! { "$in": types });
    }
  }
}
