use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateProcedure, UpdateProcedure},
  entities::{
    procedure::{Procedure, ProcedureConfig, ProcedureListItemInfo},
    resource::{Resource, ResourceListItem},
    update::ResourceTarget,
  },
};

use crate::{
  maps::name_to_procedure, monitor_client, sync::ResourceSync,
};

impl ResourceSync for Procedure {
  type PartialConfig = ProcedureConfig;
  type ListItemInfo = ProcedureListItemInfo;
  type ExtLookup = ();

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

  async fn init_lookup_data() -> Self::ExtLookup {}

  async fn create(
    resource: Resource<Self::PartialConfig>,
    _: &Self::ExtLookup,
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
    _: &Self::ExtLookup,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(UpdateProcedure {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}
