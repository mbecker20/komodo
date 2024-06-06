use monitor_client::entities::{
  alerter::Alerter, build::Build, builder::Builder,
  deployment::Deployment, procedure::Procedure, repo::Repo,
  server::Server, server_template::ServerTemplate, sync,
  update::ResourceTarget,
};
use partial_derive2::MaybeNone;

use crate::resource::{
  ResourceSync, ResourceSyncOuter, SyncLogger, ToUpdateItem,
};

impl<Implementer: ResourceSync, Logger: SyncLogger<Implementer>>
  ResourceSyncOuter<Implementer, Logger> for Server
{
  fn display() -> &'static str {
    "server"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Server(id)
  }
}

impl<Implementer: ResourceSync, Logger: SyncLogger<Implementer>>
  ResourceSyncOuter<Implementer, Logger> for Deployment
{
  fn display() -> &'static str {
    "deployment"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Deployment(id)
  }
}

impl<Implementer: ResourceSync, Logger: SyncLogger<Implementer>>
  ResourceSyncOuter<Implementer, Logger> for Build
{
  fn display() -> &'static str {
    "build"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Build(id)
  }
}

impl<Implementer: ResourceSync, Logger: SyncLogger<Implementer>>
  ResourceSyncOuter<Implementer, Logger> for Repo
{
  fn display() -> &'static str {
    "repo"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Repo(id)
  }
}

impl<Implementer: ResourceSync, Logger: SyncLogger<Implementer>>
  ResourceSyncOuter<Implementer, Logger> for Alerter
{
  fn display() -> &'static str {
    "alerter"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Alerter(id)
  }
}

impl<Implementer: ResourceSync, Logger: SyncLogger<Implementer>>
  ResourceSyncOuter<Implementer, Logger> for Builder
{
  fn display() -> &'static str {
    "builder"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Builder(id)
  }
}

impl<Implementer: ResourceSync, Logger: SyncLogger<Implementer>>
  ResourceSyncOuter<Implementer, Logger> for ServerTemplate
{
  fn display() -> &'static str {
    "server_template"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::ServerTemplate(id)
  }
}

impl<Implementer: ResourceSync, Logger: SyncLogger<Implementer>>
  ResourceSyncOuter<Implementer, Logger> for sync::ResourceSync
{
  fn display() -> &'static str {
    "resource_sync"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::ResourceSync(id)
  }
}

impl<Implementer: ResourceSync, Logger: SyncLogger<Implementer>>
  ResourceSyncOuter<Implementer, Logger> for Procedure
{
  fn display() -> &'static str {
    "procedure"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Procedure(id)
  }

  async fn run_updates(
    mut to_create: crate::resource::ToCreate<
      <Implementer as ResourceSync>::PartialConfig,
    >,
    mut to_update: crate::resource::ToUpdate<
      <Implementer as ResourceSync>::PartialConfig,
    >,
    to_delete: crate::resource::ToDelete,
  ) {
    for name in to_delete {
      if let Err(e) = Implementer::delete(name.clone()).await {
        Logger::log_failed_delete(&name, e);
        // warn!("failed to delete procedure {name} | {e:#}",);
      } else {
        Logger::log_deleted(&name);
        // info!(
        //   "{} procedure '{}'",
        //   "deleted".red().bold(),
        //   name.bold(),
        // );
      }
    }

    if to_update.is_empty() && to_create.is_empty() {
      return;
    }

    for i in 0..10 {
      let mut to_pull = Vec::new();
      for ToUpdateItem {
        id,
        resource,
        update_description,
        update_tags,
      } in &to_update
      {
        // Update resource
        let name = resource.name.clone();
        let tags = resource.tags.clone();
        let description = resource.description.clone();
        if *update_description {
          crate::resource::run_update_description::<
            Implementer,
            Self,
            Logger,
          >(id.clone(), &name, description)
          .await;
        }
        if *update_tags {
          crate::resource::run_update_tags::<Implementer, Self, Logger>(
            id.clone(),
            &name,
            tags,
          )
          .await;
        }
        if !resource.config.is_none() {
          if let Err(e) =
            Implementer::update(id.clone(), resource.clone()).await
          {
            if i == 9 {
              Logger::log_failed_update(&name, e);
              // warn!(
              //   "failed to update {} {name} | {e:#}",
              //   Self::display()
              // );
            }
          }
        }

        // info!("{} {name} updated", Self::display());
        Logger::log_updated(&name);

        // have to clone out so to_update is mutable
        to_pull.push(id.clone());
      }
      //
      to_update.retain(|resource| !to_pull.contains(&resource.id));

      let mut to_pull = Vec::new();
      for resource in &to_create {
        let name = resource.name.clone();
        let tags = resource.tags.clone();
        let description = resource.description.clone();
        let id = match Implementer::create(resource.clone()).await {
          Ok(id) => id,
          Err(e) => {
            if i == 9 {
              // warn!(
              //   "failed to create {} {name} | {e:#}",
              //   Self::display(),
              // );
              Logger::log_failed_create(&name, e);
            }
            continue;
          }
        };
        crate::resource::run_update_tags::<Implementer, Self, Logger>(
          id.clone(),
          &name,
          tags,
        )
        .await;
        crate::resource::run_update_description::<
          Implementer,
          Self,
          Logger,
        >(id, &name, description)
        .await;
        Logger::log_created(&name);
        // info!("{} {name} created", Self::display());
        to_pull.push(name);
      }
      to_create.retain(|resource| !to_pull.contains(&resource.name));

      if to_update.is_empty() && to_create.is_empty() {
        // info!("all procedures synced");
        return;
      }
    }
    Logger::log_procedure_sync_failed_max_iter();
    // warn!("procedure sync loop exited after max iterations");
  }
}
