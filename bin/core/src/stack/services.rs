use anyhow::Context;
use komodo_client::entities::{
  stack::{
    ComposeFile, ComposeService, ComposeServiceDeploy, Stack,
    StackServiceNames,
  },
  FileContents,
};

use super::remote::{
  get_remote_compose_contents, RemoteComposeContents,
};

/// Passing fresh will re-extract services from compose file, whether local or remote (repo)
pub async fn extract_services_from_stack(
  stack: &Stack,
  fresh: bool,
) -> anyhow::Result<Vec<StackServiceNames>> {
  if !fresh {
    if let Some(services) = &stack.info.deployed_services {
      return Ok(services.clone());
    } else {
      return Ok(stack.info.latest_services.clone());
    }
  }

  let compose_contents = if stack.config.file_contents.is_empty() {
    let RemoteComposeContents {
      successful,
      errored,
      ..
    } = get_remote_compose_contents(stack, None).await.context(
      "failed to get remote compose files to extract services",
    )?;
    if !errored.is_empty() {
      let mut e = anyhow::Error::msg("Trace root");
      for err in errored {
        e = e.context(format!("{}: {}", err.path, err.contents));
      }
      return Err(
        e.context("Failed to read one or more remote compose files"),
      );
    }
    successful
  } else {
    vec![FileContents {
      path: String::from("compose.yaml"),
      contents: stack.config.file_contents.clone(),
    }]
  };

  let mut res = Vec::new();
  for FileContents { path, contents } in &compose_contents {
    extract_services_into_res(
      &stack.project_name(true),
      contents,
      &mut res,
    )
    .with_context(|| {
      format!("failed to extract services from file at path: {path}")
    })?;
  }

  Ok(res)
}

pub fn extract_services_into_res(
  project_name: &str,
  compose_contents: &str,
  res: &mut Vec<StackServiceNames>,
) -> anyhow::Result<()> {
  let compose = serde_yaml::from_str::<ComposeFile>(compose_contents)
    .context("failed to parse service names from compose contents")?;

  let mut services = Vec::with_capacity(compose.services.capacity());

  for (
    service_name,
    ComposeService {
      container_name,
      deploy,
      image,
    },
  ) in compose.services
  {
    let image = image.unwrap_or_default();
    match deploy {
      Some(ComposeServiceDeploy {
        replicas: Some(replicas),
      }) if replicas > 1 => {
        for i in 1..1 + replicas {
          services.push(StackServiceNames {
            container_name: format!(
              "{project_name}-{service_name}-{i}"
            ),
            service_name: format!("{service_name}-{i}"),
            image: image.clone(),
          });
        }
      }
      _ => {
        services.push(StackServiceNames {
          container_name: container_name.unwrap_or_else(|| {
            format!("{project_name}-{service_name}")
          }),
          service_name,
          image,
        });
      }
    }
  }

  res.extend(services);

  Ok(())
}
