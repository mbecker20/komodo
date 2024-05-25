use std::{sync::OnceLock, time::Duration};

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  alert::{Alert, AlertData, AlertDataVariant},
  monitor_timestamp,
  server::stats::SeverityLevel,
  server_template::hetzner::{
    HetznerDatacenter, HetznerServerTemplateConfig,
    HetznerServerType, HetznerVolumeFormat,
  },
  update::ResourceTarget,
};

use crate::{
  cloud::hetzner::{
    common::HetznerServerStatus, create_server::CreateServerBody,
    create_volume::CreateVolumeBody,
  },
  config::core_config,
  helpers::alert::send_alerts,
};

use self::{
  client::HetznerClient,
  common::{HetznerAction, HetznerActionResponse},
};

mod client;
mod common;
mod create_server;
mod create_volume;

fn hetzner() -> Option<&'static HetznerClient> {
  static HETZNER_CLIENT: OnceLock<Option<HetznerClient>> =
    OnceLock::new();
  HETZNER_CLIENT
    .get_or_init(|| {
      let token = &core_config().hetzner.token;
      (!token.is_empty()).then(|| HetznerClient::new(token))
    })
    .as_ref()
}

pub struct HetznerServerMinimal {
  pub id: i64,
  pub ip: String,
}

const POLL_RATE_SECS: u64 = 2;
const MAX_POLL_TRIES: usize = 30;

#[instrument]
pub async fn launch_hetzner_server(
  name: &str,
  config: HetznerServerTemplateConfig,
) -> anyhow::Result<HetznerServerMinimal> {
  let hetzner =
    *hetzner().as_ref().context("Hetzner token not configured")?;
  let HetznerServerTemplateConfig {
    image,
    automount,
    datacenter,
    private_network_ids,
    placement_group,
    enable_public_ipv4,
    enable_public_ipv6,
    firewall_ids,
    server_type,
    ssh_keys,
    user_data,
    use_public_ip,
    labels,
    volumes,
    port: _,
  } = config;
  let datacenter = hetzner_datacenter(datacenter);

  // Create volumes and get their ids
  let mut volume_ids = Vec::new();
  for volume in volumes {
    let body = CreateVolumeBody {
      name: volume.name,
      format: Some(hetzner_format(volume.format)),
      location: Some(datacenter.into()),
      labels: volume.labels,
      size: volume.size_gb,
      automount: None,
      server: None,
    };
    let id = hetzner
      .create_volume(&body)
      .await
      .context("failed to create hetzner volume")?
      .volume
      .id;
    volume_ids.push(id);
  }

  let body = CreateServerBody {
    name: name.to_string(),
    automount: Some(automount),
    datacenter: Some(datacenter),
    location: None,
    firewalls: firewall_ids
      .into_iter()
      .map(|firewall| create_server::Firewall { firewall })
      .collect(),
    image,
    labels,
    networks: private_network_ids,
    placement_group: (placement_group > 0).then_some(placement_group),
    public_net: (enable_public_ipv4 || enable_public_ipv6).then_some(
      create_server::PublicNet {
        enable_ipv4: enable_public_ipv4,
        enable_ipv6: enable_public_ipv6,
        ipv4: None,
        ipv6: None,
      },
    ),
    server_type: hetzner_server_type(server_type),
    ssh_keys,
    start_after_create: true,
    user_data: (!user_data.is_empty()).then_some(user_data),
    volumes: volume_ids,
  };

  let server = hetzner
    .create_server(&body)
    .await
    .context("failed to create hetnzer server")?
    .server;

  let ip = if use_public_ip {
    server.public_net.ipv4.context("instance ")?.ip
  } else {
    server
      .private_net
      .first()
      .context("no private networks attached")?
      .ip
      .to_string()
  };
  let server = HetznerServerMinimal { id: server.id, ip };

  for _ in 0..MAX_POLL_TRIES {
    tokio::time::sleep(Duration::from_secs(POLL_RATE_SECS)).await;
    let Ok(res) = hetzner.get_server(server.id).await else {
      continue;
    };
    if matches!(res.server.status, HetznerServerStatus::Running) {
      return Ok(server);
    }
  }

  Err(anyhow!(
    "failed to verify server running after polling status"
  ))
}

#[allow(unused)]
const MAX_TERMINATION_TRIES: usize = 5;
#[allow(unused)]
const TERMINATION_WAIT_SECS: u64 = 15;

#[allow(unused)]
pub async fn terminate_hetzner_server_with_retry(
  id: i64,
) -> anyhow::Result<()> {
  let hetzner =
    *hetzner().as_ref().context("Hetzner token not configured")?;

  for i in 0..MAX_TERMINATION_TRIES {
    let message = match hetzner.delete_server(id).await {
      Ok(HetznerActionResponse {
        action: HetznerAction { error: None, .. },
      }) => return Ok(()),
      Ok(HetznerActionResponse {
        action: HetznerAction { error: Some(e), .. },
      }) => (i == MAX_TERMINATION_TRIES - 1).then(|| {
        format!(
          "failed to terminate instance | code: {} | {}",
          e.code, e.message
        )
      }),
      Err(e) => {
        (i == MAX_TERMINATION_TRIES - 1).then(|| format!("{e:#}"))
      }
    };
    if let Some(message) = message {
      error!("failed to terminate hetzner server {id} | {message}");
      let alert = Alert {
        id: Default::default(),
        ts: monitor_timestamp(),
        resolved: false,
        level: SeverityLevel::Critical,
        target: ResourceTarget::system(),
        variant: AlertDataVariant::HetznerBuilderTerminationFailed,
        data: AlertData::HetznerBuilderTerminationFailed {
          server_id: id,
          message: message.clone(),
        },
        resolved_ts: None,
      };
      send_alerts(&[alert]).await;
      return Err(anyhow::Error::msg(message));
    }
    tokio::time::sleep(Duration::from_secs(TERMINATION_WAIT_SECS))
      .await;
  }

  Ok(())
}

fn hetzner_format(
  format: HetznerVolumeFormat,
) -> common::HetznerVolumeFormat {
  match format {
    HetznerVolumeFormat::Xfs => common::HetznerVolumeFormat::Xfs,
    HetznerVolumeFormat::Ext4 => common::HetznerVolumeFormat::Ext4,
  }
}

fn hetzner_datacenter(
  datacenter: HetznerDatacenter,
) -> common::HetznerDatacenter {
  match datacenter {
    HetznerDatacenter::Nuremberg1Dc3 => {
      common::HetznerDatacenter::Nuremberg1Dc3
    }
    HetznerDatacenter::Helsinki1Dc2 => {
      common::HetznerDatacenter::Helsinki1Dc2
    }
    HetznerDatacenter::Falkenstein1Dc14 => {
      common::HetznerDatacenter::Falkenstein1Dc14
    }
    HetznerDatacenter::AshburnDc1 => {
      common::HetznerDatacenter::AshburnDc1
    }
    HetznerDatacenter::HillsboroDc1 => {
      common::HetznerDatacenter::HillsboroDc1
    }
  }
}

fn hetzner_server_type(
  server_type: HetznerServerType,
) -> common::HetznerServerType {
  match server_type {
    HetznerServerType::SharedIntel1Core2Ram20Disk => {
      common::HetznerServerType::SharedIntel1Core2Ram20Disk
    }
    HetznerServerType::SharedAmd2Core2Ram40Disk => {
      common::HetznerServerType::SharedAmd2Core2Ram40Disk
    }
    HetznerServerType::SharedArm2Core4Ram40Disk => {
      common::HetznerServerType::SharedArm2Core4Ram40Disk
    }
    HetznerServerType::SharedIntel2Core4Ram40Disk => {
      common::HetznerServerType::SharedIntel2Core4Ram40Disk
    }
    HetznerServerType::SharedAmd3Core4Ram80Disk => {
      common::HetznerServerType::SharedAmd3Core4Ram80Disk
    }
    HetznerServerType::SharedArm4Core8Ram80Disk => {
      common::HetznerServerType::SharedArm4Core8Ram80Disk
    }
    HetznerServerType::SharedIntel2Core8Ram80Disk => {
      common::HetznerServerType::SharedIntel2Core8Ram80Disk
    }
    HetznerServerType::SharedAmd4Core8Ram160Disk => {
      common::HetznerServerType::SharedAmd4Core8Ram160Disk
    }
    HetznerServerType::SharedArm8Core16Ram160Disk => {
      common::HetznerServerType::SharedArm8Core16Ram160Disk
    }
    HetznerServerType::SharedIntel4Core16Ram160Disk => {
      common::HetznerServerType::SharedIntel4Core16Ram160Disk
    }
    HetznerServerType::SharedAmd8Core16Ram240Disk => {
      common::HetznerServerType::SharedAmd8Core16Ram240Disk
    }
    HetznerServerType::SharedArm16Core32Ram320Disk => {
      common::HetznerServerType::SharedArm16Core32Ram320Disk
    }
    HetznerServerType::SharedIntel8Core32Ram240Disk => {
      common::HetznerServerType::SharedIntel8Core32Ram240Disk
    }
    HetznerServerType::SharedAmd16Core32Ram360Disk => {
      common::HetznerServerType::SharedAmd16Core32Ram360Disk
    }
    HetznerServerType::DedicatedAmd2Core8Ram80Disk => {
      common::HetznerServerType::DedicatedAmd2Core8Ram80Disk
    }
    HetznerServerType::DedicatedAmd4Core16Ram160Disk => {
      common::HetznerServerType::DedicatedAmd4Core16Ram160Disk
    }
    HetznerServerType::DedicatedAmd8Core32Ram240Disk => {
      common::HetznerServerType::DedicatedAmd8Core32Ram240Disk
    }
    HetznerServerType::DedicatedAmd16Core64Ram360Disk => {
      common::HetznerServerType::DedicatedAmd16Core64Ram360Disk
    }
    HetznerServerType::DedicatedAmd32Core128Ram600Disk => {
      common::HetznerServerType::DedicatedAmd32Core128Ram600Disk
    }
    HetznerServerType::DedicatedAmd48Core192Ram960Disk => {
      common::HetznerServerType::DedicatedAmd48Core192Ram960Disk
    }
  }
}
