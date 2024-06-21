use std::collections::HashMap;

use monitor_client::{
  api::write::{self, DeleteDeployment},
  entities::{
    deployment::{
      Deployment, DeploymentConfig, DeploymentConfigDiff,
      DeploymentImage, PartialDeploymentConfig,
    },
    resource::Resource,
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{
  maps::{id_to_build, id_to_server, name_to_deployment},
  state::monitor_client,
  sync::resource::ResourceSync,
};

impl ResourceSync for Deployment {
  type Config = DeploymentConfig;
  type Info = ();
  type PartialConfig = PartialDeploymentConfig;
  type ConfigDiff = DeploymentConfigDiff;

  fn display() -> &'static str {
    "deployment"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Deployment(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>
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

  fn get_diff(
    mut original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff> {
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
        version: *version,
      };
    }

    Ok(original.partial_diff(update))
  }

  async fn delete(id: String) -> anyhow::Result<()> {
    monitor_client().write(DeleteDeployment { id }).await?;
    Ok(())
  }
}
