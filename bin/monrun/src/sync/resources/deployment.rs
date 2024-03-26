use std::collections::HashMap;

use async_trait::async_trait;
use monitor_client::{
  api::{
    read::{ListBuilds, ListServers},
    write,
  },
  entities::{
    deployment::{
      Deployment, DeploymentImage, DeploymentListItemInfo,
      PartialDeploymentConfig,
    },
    resource::{Resource, ResourceListItem},
    update::ResourceTarget,
  },
};

use crate::{
  maps::name_to_deployment, monitor_client, sync::ResourceSync,
};

pub struct DeploymentExtLookup {
  pub servers: HashMap<String, String>,
  pub builds: HashMap<String, String>,
}

#[async_trait]
impl ResourceSync for Deployment {
  type PartialConfig = PartialDeploymentConfig;
  type ListItemInfo = DeploymentListItemInfo;
  type ExtLookup = DeploymentExtLookup;

  fn display() -> &'static str {
    "deployment"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Deployment(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_deployment()
  }

  async fn init_lookup_data() -> Self::ExtLookup {
    let servers = monitor_client()
      .read(ListServers::default())
      .await
      .expect("failed to get servers")
      .into_iter()
      .map(|b| (b.name, b.id))
      .collect::<HashMap<_, _>>();

    let builds = monitor_client()
      .read(ListBuilds::default())
      .await
      .expect("failed to get builds")
      .into_iter()
      .map(|b| (b.name, b.id))
      .collect::<HashMap<_, _>>();

    DeploymentExtLookup { servers, builds }
  }

  async fn create(
    mut resource: Resource<Self::PartialConfig>,
    lookup: &Self::ExtLookup,
  ) -> anyhow::Result<String> {
    handle_name_to_id_switch(&mut resource.config, lookup);
    monitor_client()
      .write(write::CreateDeployment {
        name: resource.name,
        config: resource.config,
      })
      .await
      .map(|res| res.id)
  }

  async fn update(
    id: String,
    mut resource: Resource<Self::PartialConfig>,
    lookup: &Self::ExtLookup,
  ) -> anyhow::Result<()> {
    handle_name_to_id_switch(&mut resource.config, lookup);
    monitor_client()
      .write(write::UpdateDeployment {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}

fn handle_name_to_id_switch(
  config: &mut PartialDeploymentConfig,
  lookup: &DeploymentExtLookup,
) {
  config.server_id = config
    .server_id
    .as_ref()
    .and_then(|name| lookup.servers.get(name).cloned());
  if let Some(DeploymentImage::Build {
    build_id: name,
    version,
  }) = &config.image
  {
    match lookup.builds.get(name).cloned() {
      Some(build_id) => {
        config.image = DeploymentImage::Build {
          build_id,
          version: version.clone(),
        }
        .into();
      }
      None => {
        config.image = DeploymentImage::Image {
          image: String::new(),
        }
        .into();
      }
    }
  }
}
