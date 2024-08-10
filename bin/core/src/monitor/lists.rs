use anyhow::Context;
use monitor_client::entities::{
  deployment::ContainerSummary,
  server::{
    docker_image::ImageSummary, docker_network::DockerNetwork,
  },
  stack::ComposeProject,
};
use periphery_client::{
  api::{
    container::GetContainerList, GetDockerLists,
    GetDockerListsResponse,
  },
  PeripheryClient,
};

pub async fn get_docker_lists(
  periphery: &PeripheryClient,
) -> anyhow::Result<(
  Vec<ContainerSummary>,
  Vec<DockerNetwork>,
  Vec<ImageSummary>,
  Vec<ComposeProject>,
)> {
  if let Ok(GetDockerListsResponse {
    containers,
    networks,
    images,
    projects,
  }) = periphery.request(GetDockerLists {}).await
  {
    // TODO: handle the errors
    let (mut containers, mut networks, images, mut projects) = (
      containers.unwrap_or_default(),
      networks.unwrap_or_default(),
      images.unwrap_or_default(),
      projects.unwrap_or_default(),
    );
    containers.sort_by(|a, b| a.name.cmp(&b.name));
    networks.sort_by(|a, b| a.name.cmp(&b.name));
    projects.sort_by(|a, b| a.name.cmp(&b.name));
    return Ok((containers, networks, images, projects));
  }
  // Fallback to ListContainers for backward compat w/ v1.12
  let mut containers =
    periphery
      .request(GetContainerList {})
      .await
      .context("failed to get docker container list")?;
  containers.sort_by(|a, b| a.name.cmp(&b.name));
  Ok((containers, Vec::new(), Vec::new(), Vec::new()))
}
