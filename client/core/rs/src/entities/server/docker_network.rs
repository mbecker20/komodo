use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use typeshare::typeshare;

#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct DockerNetwork {
  #[serde(rename = "Name")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  #[serde(rename = "Id")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,

  #[serde(rename = "Created")]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(
    default,
    deserialize_with = "deserialize_timestamp",
    serialize_with = "serialize_timestamp"
  )]
  pub created: Option<String>,

  #[serde(rename = "Scope")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub scope: Option<String>,

  #[serde(rename = "Driver")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub driver: Option<String>,

  #[serde(rename = "EnableIPv6")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enable_ipv6: Option<bool>,

  #[serde(rename = "IPAM")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ipam: Option<Ipam>,

  #[serde(rename = "Internal")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub internal: Option<bool>,

  #[serde(rename = "Attachable")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub attachable: Option<bool>,

  #[serde(rename = "Ingress")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ingress: Option<bool>,

  #[serde(rename = "Containers")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub containers: Option<HashMap<String, NetworkContainer>>,

  #[serde(rename = "Options")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub options: Option<HashMap<String, String>>,

  #[serde(rename = "Labels")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub labels: Option<HashMap<String, String>>,
}

fn deserialize_timestamp<'de, D: Deserializer<'de>>(
  d: D,
) -> Result<Option<String>, D::Error> {
  serde::Deserialize::deserialize(d)
}

fn serialize_timestamp<S: Serializer>(
  date: &Option<String>,
  s: S,
) -> Result<S::Ok, S::Error> {
  match date {
    Some(inner) => s.serialize_some(inner),
    None => s.serialize_none(),
  }
}

impl From<bollard::service::Network> for DockerNetwork {
  fn from(value: bollard::service::Network) -> Self {
    Self {
      name: value.name,
      id: value.id,
      created: value.created,
      scope: value.scope,
      driver: value.driver,
      enable_ipv6: value.enable_ipv6,
      ipam: value.ipam.map(|ipam| ipam.into()),
      internal: value.internal,
      attachable: value.attachable,
      ingress: value.ingress,
      containers: value.containers.map(|containers| {
        containers.into_iter().map(|(k, v)| (k, v.into())).collect()
      }),
      options: value.options,
      labels: value.labels,
    }
  }
}

#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct Ipam {
  /// Name of the IPAM driver to use.
  #[serde(rename = "Driver")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub driver: Option<String>,

  /// List of IPAM configuration options, specified as a map:  ``` {\"Subnet\": <CIDR>, \"IPRange\": <CIDR>, \"Gateway\": <IP address>, \"AuxAddress\": <device_name:IP address>} ```
  #[serde(rename = "Config")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub config: Option<Vec<IpamConfig>>,

  /// Driver-specific options, specified as a map.
  #[serde(rename = "Options")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub options: Option<HashMap<String, String>>,
}

impl From<bollard::service::Ipam> for Ipam {
  fn from(value: bollard::service::Ipam) -> Self {
    Self {
      driver: value.driver,
      config: value
        .config
        .map(|config| config.into_iter().map(|c| c.into()).collect()),
      options: value.options,
    }
  }
}

#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct IpamConfig {
  #[serde(rename = "Subnet")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subnet: Option<String>,

  #[serde(rename = "IPRange")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ip_range: Option<String>,

  #[serde(rename = "Gateway")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gateway: Option<String>,

  #[serde(rename = "AuxiliaryAddresses")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub auxiliary_addresses: Option<HashMap<String, String>>,
}

impl From<bollard::service::IpamConfig> for IpamConfig {
  fn from(value: bollard::service::IpamConfig) -> Self {
    Self {
      subnet: value.subnet,
      ip_range: value.ip_range,
      gateway: value.gateway,
      auxiliary_addresses: value.auxiliary_addresses,
    }
  }
}

#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct NetworkContainer {
  #[serde(rename = "Name")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,

  #[serde(rename = "EndpointID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub endpoint_id: Option<String>,

  #[serde(rename = "MacAddress")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mac_address: Option<String>,

  #[serde(rename = "IPv4Address")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ipv4_address: Option<String>,

  #[serde(rename = "IPv6Address")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ipv6_address: Option<String>,
}

impl From<bollard::service::NetworkContainer> for NetworkContainer {
  fn from(value: bollard::service::NetworkContainer) -> Self {
    Self {
      name: value.name,
      endpoint_id: value.endpoint_id,
      mac_address: value.mac_address,
      ipv4_address: value.ipv4_address,
      ipv6_address: value.ipv6_address,
    }
  }
}
