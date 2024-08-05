use monitor_client::{
  api::write::{
    CreateBuild, CreateBuilder, CreateDeployment, CreateServer,
    UpdateTagsOnResource,
  },
  entities::{
    build::BuildConfig,
    builder::{PartialBuilderConfig, ServerBuilderConfig},
    deployment::DeploymentConfig,
    server::ServerConfig,
  },
  MonitorClient,
};
use rand::Rng;

use crate::random_string;

#[allow(unused)]
pub async fn tests() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();

  let monitor = MonitorClient::new_from_env().await?;

  let tags = (0..6).map(|_| random_string(5)).collect::<Vec<_>>();

  let mut rng = rand::thread_rng();
  let mut get_tags = || vec![tags[rng.gen_range(0..6)].to_string()];

  let server_names = (0..20)
    .map(|i| format!("server-{}-{}", random_string(8), i))
    .collect::<Vec<_>>();

  for name in &server_names {
    let resource = monitor
      .write(CreateServer {
        name: name.to_string(),
        config: ServerConfig::builder()
          .address(String::new())
          .build()?
          .into(),
      })
      .await?;
    info!("created server {}", resource.name);
    monitor
      .write(UpdateTagsOnResource {
        target: (&resource).into(),
        tags: get_tags(),
      })
      .await?;
    info!("updated tags on server {}", resource.name);
  }

  for (i, server_name) in server_names.iter().enumerate() {
    let resource = monitor
      .write(CreateDeployment {
        name: format!("dep-{}-{}", random_string(8), i),
        config: DeploymentConfig::builder()
          .server_id(server_name.to_string())
          .build()?
          .into(),
      })
      .await?;
    info!("created deployment {}", resource.name);
    monitor
      .write(UpdateTagsOnResource {
        target: (&resource).into(),
        tags: get_tags(),
      })
      .await?;
    info!("updated tags on deployment {}", resource.name);
  }

  let builder_names = (0..20)
    .map(|i| format!("builder-{}-{}", random_string(8), i))
    .collect::<Vec<_>>();

  for (i, server_name) in server_names.iter().enumerate() {
    let resource = monitor
      .write(CreateBuilder {
        name: builder_names[i].clone(),
        config: PartialBuilderConfig::Server(
          ServerBuilderConfig {
            server_id: server_name.to_string(),
          }
          .into(),
        ),
      })
      .await?;
    info!("created builder {}", resource.name);
    monitor
      .write(UpdateTagsOnResource {
        target: (&resource).into(),
        tags: get_tags(),
      })
      .await?;
    info!("updated tags on builder {}", resource.name);
  }

  for (i, builder_name) in builder_names.iter().enumerate() {
    let resource = monitor
      .write(CreateBuild {
        name: format!("build-{}-{}", random_string(8), i),
        config: BuildConfig::builder()
          .builder_id(builder_name.to_string())
          .build()?
          .into(),
      })
      .await?;
    info!("created build {}", resource.name);
    monitor
      .write(UpdateTagsOnResource {
        target: (&resource).into(),
        tags: get_tags(),
      })
      .await?;
    info!("updated tags on build {}", resource.name);
  }

  Ok(())
}
