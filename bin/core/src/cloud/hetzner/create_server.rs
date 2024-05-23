use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::{HetznerAction, HetznerServer};

#[derive(Debug, Clone, Serialize)]
pub struct CreateServerBody {
  /// Auto-mount Volumes after attach
  pub automount: bool,
  /// ID or name of Datacenter to create Server in (must not be used together with location)
  pub datacenter: Option<String>,
  /// ID or name of Location to create Server in (must not be used together with datacenter)
  pub location: Option<String>,
  /// Firewalls which should be applied on the Server's public network interface at creation time
  pub firewalls: Vec<Firewall>,
  /// ID or name of the Image the Server is created from
  pub image: String,
  /// User-defined labels (key-value pairs) for the Resource
  pub labels: HashMap<String, String>,
  /// Name of the Server to create (must be unique per Project and a valid hostname as per RFC 1123)
  pub name: String,
  /// Network IDs which should be attached to the Server private network interface at the creation time
  pub networks: Vec<i64>,
  /// ID of the Placement Group the server should be in
  pub placement_group: i64,
  /// Public Network options
  pub public_net: PublicNet,
  /// ID or name of the Server type this Server should be created with
  pub server_type: String,
  /// SSH key IDs ( integer ) or names ( string ) which should be injected into the Server at creation time
  pub ssh_keys: Vec<String>,
  /// This automatically triggers a Power on a Server-Server Action after the creation is finished and is returned in the next_actions response object.
  pub start_after_create: bool,
  /// Cloud-Init user data to use during Server creation. This field is limited to 32KiB.
  pub user_data: String,
  /// Volume IDs which should be attached to the Server at the creation time. Volumes must be in the same Location.
  pub volumes: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Firewall {
  /// ID of the Firewall
  pub firewall: i64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PublicNet {
  /// Attach an IPv4 on the public NIC. If false, no IPv4 address will be attached.
  pub enable_ipv4: bool,
  /// Attach an IPv6 on the public NIC. If false, no IPv6 address will be attached.
  pub enable_ipv6: bool,
  /// ID of the ipv4 Primary IP to use. If omitted and enable_ipv4 is true, a new ipv4 Primary IP will automatically be created.
  pub ipv4: Option<i64>,
  /// ID of the ipv6 Primary IP to use. If omitted and enable_ipv6 is true, a new ipv6 Primary IP will automatically be created.
  pub ipv6: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateServerResponse {
  pub action: HetznerAction,
  pub next_actions: Vec<HetznerAction>,
  pub root_password: Option<String>,
  pub server: HetznerServer,
}
