use anyhow::Context;
use komodo_client::{
  api::execute::{Deploy, DeployStack},
  entities::{
    build::Build,
    deployment::{Deployment, DeploymentImage, DeploymentState},
    docker::{container::ContainerListItem, image::ImageListItem},
    stack::{Stack, StackService, StackServiceNames},
    user::auto_redeploy_user,
  },
};

use crate::{
  api::execute::{self, ExecuteRequest},
  helpers::query::get_stack_state_from_containers,
  stack::{
    compose_container_match_regex,
    services::extract_services_from_stack,
  },
  state::{deployment_status_cache, stack_status_cache},
};

use super::{CachedDeploymentStatus, CachedStackStatus, History};

pub async fn update_deployment_cache(
  deployments: Vec<Deployment>,
  containers: &[ContainerListItem],
  images: &[ImageListItem],
  builds: &[Build],
) {
  let deployment_status_cache = deployment_status_cache();
  for deployment in deployments {
    let container = containers
      .iter()
      .find(|container| container.name == deployment.name)
      .cloned();
    let prev = deployment_status_cache
      .get(&deployment.id)
      .await
      .map(|s| s.curr.state);
    let state = container
      .as_ref()
      .map(|c| c.state.into())
      .unwrap_or(DeploymentState::NotDeployed);
    let update_available = if let Some(ContainerListItem {
      image_id: Some(curr_image_id),
      ..
    }) = &container
    {
      let source_image = match deployment.config.image {
        DeploymentImage::Build { build_id, version } => {
          let (build_name, build_version) = builds
            .iter()
            .find(|build| build.id == build_id)
            .map(|b| (b.name.as_ref(), b.config.version))
            .unwrap_or(("Unknown", Default::default()));
          let version = if version.is_none() {
            build_version.to_string()
          } else {
            version.to_string()
          };
          format!("{build_name}:{version}")
        }
        DeploymentImage::Image { image } => image,
      };
      images
        .iter()
        .find(|i| i.name == source_image)
        .map(|i| &i.id != curr_image_id)
        .unwrap_or_default()
    } else {
      false
    };
    if deployment.config.auto_update {
      let deployment = deployment.name.clone();
      tokio::spawn(async move {
        if let Err(e) = execute::inner_handler(
          ExecuteRequest::Deploy(Deploy {
            deployment: deployment.clone(),
            stop_time: None,
            stop_signal: None,
          }),
          auto_redeploy_user().to_owned(),
        )
        .await
        {
          warn!("Failed auto update Deployment {deployment} | {e:#}")
        }
      });
    }
    deployment_status_cache
      .insert(
        deployment.id.clone(),
        History {
          curr: CachedDeploymentStatus {
            id: deployment.id,
            state,
            container,
            update_available,
          },
          prev,
        }
        .into(),
      )
      .await;
  }
}

pub async fn update_stack_cache(
  stacks: Vec<Stack>,
  containers: &[ContainerListItem],
  images: &[ImageListItem],
) {
  let stack_status_cache = stack_status_cache();
  for stack in stacks {
    let services = extract_services_from_stack(&stack);
    let mut services_with_containers = services.iter().map(|StackServiceNames { service_name, container_name, image }| {
			let container = containers.iter().find(|container| {
				match compose_container_match_regex(container_name)
					.with_context(|| format!("failed to construct container name matching regex for service {service_name}")) 
				{
					Ok(regex) => regex,
					Err(e) => {
						warn!("{e:#}");
						return false
					}
				}.is_match(&container.name)
			}).cloned();
      let update_available = if let Some(ContainerListItem { image_id: Some(curr_image_id), .. }) = &container {
        images
        .iter()
        .find(|i| &i.name == image)
        .map(|i| &i.id != curr_image_id)
        .unwrap_or_default()
      } else {
        false
      };
      if stack.config.auto_update && update_available {
        let stack = stack.name.clone();
        let service = service_name.clone();
        tokio::spawn(async move {
          if let Err(e) = execute::inner_handler(
            ExecuteRequest::DeployStack(DeployStack { stack: stack.clone(), service: Some(service.clone()), stop_time: None }),
            auto_redeploy_user().to_owned()
          ).await {
            warn!("Failed auto update Stack {stack} | service: {service} | {e:#}")
          }
        });
      }
			StackService {
				service: service_name.clone(),
        image: image.to_string(),
				container,
        update_available,
			}
		}).collect::<Vec<_>>();
    services_with_containers
      .sort_by(|a, b| a.service.cmp(&b.service));
    let prev = stack_status_cache
      .get(&stack.id)
      .await
      .map(|s| s.curr.state);
    let status = CachedStackStatus {
      id: stack.id.clone(),
      state: get_stack_state_from_containers(
        &stack.config.ignore_services,
        &services,
        containers,
      ),
      services: services_with_containers,
    };
    stack_status_cache
      .insert(stack.id, History { curr: status, prev }.into())
      .await;
  }
}
