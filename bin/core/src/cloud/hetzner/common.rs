use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerServer {
  pub id: i64,
  pub image: HetznerImage,
  pub name: String,
  pub primary_disk_size: f64,
  pub private_net: Vec<HetznerPrivateNet>,
  pub public_net: HetznerPublicNet,
  pub server_type: HetznerServerType,
  pub status: String,
  pub volumes: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerServerType {
  pub architecture: String,
  pub cores: i64,
  pub cpu_type: String,
  pub description: String,
  pub disk: f64,
  pub id: i64,
  pub memory: f64,
  pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerPrivateNet {
  pub alias_ips: Vec<String>,
  pub ip: String,
  pub mac_address: String,
  pub network: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerPublicNet {
  pub firewalls: Vec<HetznerFirewall>,
  pub ipv4: HetznerIpv4,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerFirewall {
  pub id: i64,
  pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerIpv4 {
  pub blocked: bool,
  pub dns_prt: String,
  pub ip: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerImage {
  pub description: String,
  pub name: String,
  pub os_flavor: String,
  pub os_version: String,
  pub rapid_deploy: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerAction {
  pub command: String,
  pub error: HetznerError,
  pub finished: Option<String>,
  pub id: i64,
  pub progress: i32,
  pub resources: Vec<HetznerResource>,
  pub started: String,
  pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerError {
  pub code: String,
  pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerResource {
  pub id: i64,
  #[serde(rename = "type")]
  pub ty: String,
}
