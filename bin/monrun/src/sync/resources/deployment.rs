use std::collections::HashMap;

use monitor_client::{
  api::write,
  entities::{
    deployment::{
      Deployment, DeploymentListItemInfo, PartialDeploymentConfig,
    },
    resource::{Resource, ResourceListItem},
    update::ResourceTarget,
  },
};

use crate::{
  maps::name_to_deployment, monitor_client, sync::ResourceSync,
};

impl ResourceSync for Deployment {
  type PartialConfig = PartialDeploymentConfig;
  type ListItemInfo = DeploymentListItemInfo;

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

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
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
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::UpdateDeployment {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}
