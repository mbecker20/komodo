use std::collections::HashMap;

use monitor_client::{
  api::{read::GetDeployment, write},
  entities::{
    deployment::{
      Deployment, DeploymentConfig, DeploymentImage,
      DeploymentListItemInfo, PartialDeploymentConfig,
    },
    resource::{Resource, ResourceListItem},
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{
  maps::{id_to_build, id_to_server, name_to_deployment},
  monitor_client,
};

use super::ResourceSync;

impl ResourceSync for Deployment {
  type PartialConfig = PartialDeploymentConfig;
  type FullConfig = DeploymentConfig;
  type FullInfo = ();
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
    resource: ResourceToml<Self::PartialConfig>,
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
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(write::UpdateDeployment {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn get(
    id: String,
  ) -> anyhow::Result<Resource<Self::FullConfig, Self::FullInfo>> {
    monitor_client()
      .read(GetDeployment { deployment: id })
      .await
  }

  async fn minimize_update(
    mut original: Self::FullConfig,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::PartialConfig> {
    // need to replace the server id with name
    original.server_id = id_to_server()
      .get(&original.server_id)
      .map(|s| s.name.clone())
      .unwrap_or_default();

    // need to replace the build id with name
    if let DeploymentImage::Build { build_id, version } =
      &original.image
    {
      original.image = DeploymentImage::Build {
        build_id: id_to_build()
          .get(build_id)
          .map(|b| b.name.clone())
          .unwrap_or_default(),
        version: version.clone(),
      };
    }

    Ok(original.partial_diff(update).into())
  }
}
