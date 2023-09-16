use derive_builder::Builder;
use derive_variants::EnumVariants;
use mungos::derive::MungosIndexed;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use super::resource::{Resource, ResourceListItem};

#[typeshare]
pub type Builder = Resource<BuilderConfig, ()>;

#[typeshare]
pub type BuilderListItem = ResourceListItem<BuilderListItemInfo>;

#[typeshare(serialized_as = "Partial<BuilderConfig>")]
pub type _PartialBuilderConfig = PartialBuilderConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuilderListItemInfo {
    pub provider: String,
    pub instance_type: Option<String>,
}

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, MungosIndexed, EnumVariants,
)]
#[variant_derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    Display,
    EnumString
)]
#[serde(tag = "type", content = "params")]
pub enum BuilderConfig {
    Server(ServerBuilderConfig),
    Aws(AwsBuilderConfig),
}

#[typeshare(serialized_as = "Partial<ServerBuilderConfig>")]
pub type _PartialServerBuilderConfig = PartialServerBuilderConfig;

#[typeshare(serialized_as = "Partial<AwsBuilderConfig>")]
pub type _PartialAwsBuilderConfig = PartialAwsBuilderConfig;

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, MungosIndexed, EnumVariants,
)]
#[variant_derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    Display,
    EnumString
)]
#[serde(tag = "type", content = "params")]
pub enum PartialBuilderConfig {
    Server(_PartialServerBuilderConfig),
    Aws(_PartialAwsBuilderConfig),
}

impl From<PartialBuilderConfig> for BuilderConfig {
    fn from(value: PartialBuilderConfig) -> BuilderConfig {
        match value {
            PartialBuilderConfig::Server(server) => {
                BuilderConfig::Server(server.into())
            }
            PartialBuilderConfig::Aws(builder) => {
                BuilderConfig::Aws(builder.into())
            }
        }
    }
}

impl BuilderConfig {
    pub fn merge_partial(
        self,
        partial: PartialBuilderConfig,
    ) -> BuilderConfig {
        match partial {
            PartialBuilderConfig::Server(partial) => match self {
                BuilderConfig::Server(config) => {
                    let config = ServerBuilderConfig {
                        id: partial.id.unwrap_or(config.id),
                    };
                    BuilderConfig::Server(config)
                }
                _ => BuilderConfig::Server(partial.into()),
            },
            PartialBuilderConfig::Aws(partial) => match self {
                BuilderConfig::Aws(config) => {
                    let config = AwsBuilderConfig {
                        region: partial
                            .region
                            .unwrap_or(config.region),
                        instance_type: partial
                            .instance_type
                            .unwrap_or(config.instance_type),
                        volume_gb: partial
                            .volume_gb
                            .unwrap_or(config.volume_gb),
                        ami_id: partial
                            .ami_id
                            .unwrap_or(config.ami_id),
                        subnet_id: partial
                            .subnet_id
                            .unwrap_or(config.subnet_id),
                        security_group_ids: partial
                            .security_group_ids
                            .unwrap_or(config.security_group_ids),
                        key_pair_name: partial
                            .key_pair_name
                            .unwrap_or(config.key_pair_name),
                        assign_public_ip: partial
                            .assign_public_ip
                            .unwrap_or(config.assign_public_ip),
                        github_accounts: partial
                            .github_accounts
                            .unwrap_or(config.github_accounts),
                        docker_accounts: partial
                            .docker_accounts
                            .unwrap_or(config.docker_accounts),
                    };
                    BuilderConfig::Aws(config)
                }
                _ => BuilderConfig::Aws(partial.into()),
            },
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
pub struct ServerBuilderConfig {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
pub struct AwsBuilderConfig {
    #[serde(default = "aws_default_region")]
    #[builder(default = "aws_default_region()")]
    #[partial_default(aws_default_region())]
    pub region: String,

    #[serde(default = "aws_default_instance_type")]
    #[builder(default = "aws_default_instance_type()")]
    #[partial_default(aws_default_instance_type())]
    pub instance_type: String,

    #[serde(default = "aws_default_volume_gb")]
    #[builder(default = "aws_default_volume_gb()")]
    #[partial_default(aws_default_volume_gb())]
    pub volume_gb: i32,

    pub ami_id: String,
    pub subnet_id: String,
    pub security_group_ids: Vec<String>,
    pub key_pair_name: String,
    pub assign_public_ip: bool,

    #[serde(default)]
    pub github_accounts: Vec<String>,
    #[serde(default)]
    pub docker_accounts: Vec<String>,
}

fn aws_default_region() -> String {
    String::from("us-east-1")
}

fn aws_default_instance_type() -> String {
    String::from("c5.2xlarge")
}

fn aws_default_volume_gb() -> i32 {
    20
}
