use std::collections::HashMap;

use monitor_client::{
  api::{
    read::ListTags,
    write::{CreateProcedure, UpdateProcedure},
  },
  entities::{
    procedure::{Procedure, ProcedureConfig, ProcedureListItemInfo},
    resource::{Resource, ResourceListItem},
    update::ResourceTarget,
  },
};

use crate::{
  maps::name_to_procedure,
  monitor_client,
  sync::{ResourceSync, ToCreate, ToUpdate},
};

impl ResourceSync for Procedure {
  type PartialConfig = ProcedureConfig;
  type ListItemInfo = ProcedureListItemInfo;

  fn display() -> &'static str {
    "procedure"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Procedure(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_procedure()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(CreateProcedure {
        name: resource.name,
        config: resource.config,
      })
      .await
      .map(|p| p.id)
  }

  async fn update(
    id: String,
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(UpdateProcedure {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn run_updates(
    mut to_update: ToUpdate<Self::PartialConfig>,
    mut to_create: ToCreate<Self::PartialConfig>,
  ) {
    let mut tag_name_to_id = monitor_client()
      .read(ListTags::default())
      .await
      .expect("failed to ListTags mid run")
      .into_iter()
      .map(|tag| (tag.name, tag.id))
      .collect::<HashMap<_, _>>();

    if to_update.is_empty() && to_create.is_empty() {
      return;
    }

    for i in 0..10 {
      let mut to_pull = Vec::new();
      for (id, resource) in &to_update {
        // Update resource
        let name = resource.name.clone();
        let tags = resource.tags.clone();
        let description = resource.description.clone();
        if let Err(e) =
          Self::update(id.clone(), resource.clone()).await
        {
          if i == 9 {
            warn!(
              "failed to update {} {name} | {e:#}",
              Self::display()
            );
          }
        }
        Self::update_tags(
          id.clone(),
          &name,
          &tags,
          &mut tag_name_to_id,
        )
        .await;
        Self::update_description(id.clone(), description).await;
        info!("{} {name} updated", Self::display());
        // have to clone out so to_update is mutable
        to_pull.push(id.clone());
      }
      to_update.retain(|(id, _)| !to_pull.contains(id));

      let mut to_pull = Vec::new();
      for resource in &to_create {
        let name = resource.name.clone();
        let tags = resource.tags.clone();
        let description = resource.description.clone();
        let id = match Self::create(resource.clone()).await {
          Ok(id) => id,
          Err(e) => {
            if i == 9 {
              warn!(
                "failed to create {} {name} | {e:#}",
                Self::display(),
              );
            }
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
        to_pull.push(name);
      }
      to_create.retain(|resource| !to_pull.contains(&resource.name));

      if to_update.is_empty() && to_create.is_empty() {
        info!(
          "============ {}s synced ✅ ============",
          Self::display()
        );
        return;
      }
    }
    warn!("procedure sync loop exited after max iterations");
  }
}
