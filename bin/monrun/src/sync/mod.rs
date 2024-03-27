use std::{collections::HashMap, path::Path};

use monitor_client::{
  api::{
    read::ListTags,
    write::{CreateTag, UpdateDescription, UpdateTagsOnResource},
  },
  entities::{
    alerter::Alerter,
    build::Build,
    builder::Builder,
    deployment::Deployment,
    procedure::Procedure,
    repo::Repo,
    resource::{Resource, ResourceListItem},
    server::Server,
    update::ResourceTarget,
  },
};

use crate::{monitor_client, wait_for_enter};

mod resource_file;
mod resources;

pub async fn run_sync(path: &Path) -> anyhow::Result<()> {
  info!("path: {path:?}");

  let resources = resource_file::read_resources(path)?;

  println!("{resources:#?}");

  let (server_updates, server_creates) =
    Server::get_updates(resources.servers)?;
  let (deployment_updates, deployment_creates) =
    Deployment::get_updates(resources.deployments)?;
  let (build_updates, build_creates) =
    Build::get_updates(resources.builds)?;
  let (builder_updates, builder_creates) =
    Builder::get_updates(resources.builders)?;
  let (alerter_updates, alerter_creates) =
    Alerter::get_updates(resources.alerters)?;
  let (repo_updates, repo_creates) =
    Repo::get_updates(resources.repos)?;
  let (procedure_updates, procedure_creates) =
    Procedure::get_updates(resources.procedures)?;

  wait_for_enter("CONTINUE")?;

  // Run these first, which require no name -> id replacement
  Alerter::run_updates(alerter_updates, alerter_creates).await;
  Builder::run_updates(builder_updates, builder_creates).await;
  Server::run_updates(server_updates, server_creates).await;

  Build::run_updates(build_updates, build_creates).await;
  Deployment::run_updates(deployment_updates, deployment_creates)
    .await;
  Repo::run_updates(repo_updates, repo_creates).await;
  Procedure::run_updates(procedure_updates, procedure_creates).await;

  Ok(())
}

type ToUpdate<T> = Vec<(String, Resource<T>)>;
type ToCreate<T> = Vec<Resource<T>>;
type UpdatesResult<T> = (ToUpdate<T>, ToCreate<T>);

pub trait ResourceSync {
  type PartialConfig: Clone + Send + 'static;
  type ListItemInfo: 'static;
  type ExtLookup: Send + Sync;

  fn display() -> &'static str;

  fn resource_target(id: String) -> ResourceTarget;

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>;

  async fn init_lookup_data() -> Self::ExtLookup;

  /// Returns created id
  async fn create(
    resource: Resource<Self::PartialConfig>,
    ext_lookup: &Self::ExtLookup,
  ) -> anyhow::Result<String>;

  async fn update(
    id: String,
    resource: Resource<Self::PartialConfig>,
    ext_lookup: &Self::ExtLookup,
  ) -> anyhow::Result<()>;

  fn get_updates(
    resources: Vec<Resource<Self::PartialConfig>>,
  ) -> anyhow::Result<UpdatesResult<Self::PartialConfig>> {
    let map = Self::name_to_resource();

    // (name, partial config)
    let mut to_update =
      Vec::<(String, Resource<Self::PartialConfig>)>::new();
    let mut to_create = Vec::<Resource<Self::PartialConfig>>::new();

    for resource in resources {
      match map.get(&resource.name).map(|s| s.id.clone()) {
        Some(id) => {
          to_update.push((id, resource));
        }
        None => {
          to_create.push(resource);
        }
      }
    }

    if !to_create.is_empty() {
      println!(
        "\n{} TO CREATE: {}",
        Self::display(),
        to_create
          .iter()
          .map(|item| item.name.as_str())
          .collect::<Vec<_>>()
          .join(", ")
      );
    }

    if !to_update.is_empty() {
      println!(
        "\n{} TO UPDATE: {}",
        Self::display(),
        to_update
          .iter()
          .map(|(_, item)| item.name.as_str())
          .collect::<Vec<_>>()
          .join(", ")
      );
    }

    Ok((to_update, to_create))
  }

  async fn run_updates(
    to_update: ToUpdate<Self::PartialConfig>,
    to_create: ToCreate<Self::PartialConfig>,
  ) {
    let mut tag_name_to_id = monitor_client()
      .read(ListTags::default())
      .await
      .expect("failed to ListTags mid run")
      .into_iter()
      .map(|tag| (tag.name, tag.id))
      .collect::<HashMap<_, _>>();

    let ext_lookup = Self::init_lookup_data().await;

    let log_after = !to_update.is_empty() || !to_create.is_empty();

    for (id, resource) in to_update {
      // Update resource
      let name = resource.name.clone();
      let tags = resource.tags.clone();
      let description = resource.description.clone();
      if let Err(e) =
        Self::update(id.clone(), resource, &ext_lookup).await
      {
        warn!("failed to update {} {name} | {e:#}", Self::display());
      }
      Self::update_tags(
        id.clone(),
        &name,
        &tags,
        &mut tag_name_to_id,
      )
      .await;
      Self::update_description(id, description).await;
      info!("{} {name} updated", Self::display());
    }

    for resource in to_create {
      let name = resource.name.clone();
      let tags = resource.tags.clone();
      let description = resource.description.clone();
      let id = match Self::create(resource, &ext_lookup).await {
        Ok(id) => id,
        Err(e) => {
          warn!(
            "failed to create {} {name} | {e:#}",
            Self::display(),
          );
          continue;
        }
      };
      Self::update_tags(
        id.clone(),
        &name,
        &tags,
        &mut tag_name_to_id,
      )
      .await;
      Self::update_description(id, description).await;
      info!("{} {name} created", Self::display());
    }
    if log_after {
      info!(
        "============ {}s synced ✅ ============",
        Self::display()
      );
    }
  }

  async fn update_tags(
    resource_id: String,
    resource_name: &str,
    tags: &[String],
    tag_name_to_id: &mut HashMap<String, String>,
  ) {
    // make sure all tags are created
    for tag_name in tags {
      if !tag_name_to_id.contains_key(tag_name) {
        let tag_id = monitor_client()
          .write(CreateTag {
            name: tag_name.to_string(),
          })
          .await
          .expect("failed to CreateTag mid run")
          .id;
        tag_name_to_id.insert(tag_name.to_string(), tag_id);
      }
    }

    // get Vec<tag_id>
    let tags = tags
      .iter()
      .map(|tag_name| {
        tag_name_to_id
          .get(tag_name)
          .expect("somehow didn't find tag at this point")
          .to_string()
      })
      .collect();

    // Update tags
    if let Err(e) = monitor_client()
      .write(UpdateTagsOnResource {
        target: Self::resource_target(resource_id),
        tags,
      })
      .await
    {
      warn!(
        "failed to update tags on {} {resource_name} | {e:#}",
        Self::display(),
      );
    }
  }

  async fn update_description(id: String, description: String) {
    if let Err(e) = monitor_client()
      .write(UpdateDescription {
        target: Self::resource_target(id.clone()),
        description,
      })
      .await
    {
      warn!("failed to update resource {id} description | {e:#}");
    }
  }
}
