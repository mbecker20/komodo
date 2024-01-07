use async_trait::async_trait;
use monitor_client::{
  api::execute::RunProcedure,
  entities::{
    procedure::Procedure, update::Update, Operation, PermissionLevel,
  },
};
use resolver_api::Resolve;
use serror::serialize_error_pretty;
use tokio::sync::Mutex;

use crate::{
  auth::RequestUser,
  helpers::{make_update, resource::StateResource},
  state::State,
};

#[async_trait]
impl Resolve<RunProcedure, RequestUser> for State {
  async fn resolve(
    &self,
    RunProcedure { procedure_id }: RunProcedure,
    user: RequestUser,
  ) -> anyhow::Result<Update> {
    let procedure: Procedure = self
      .get_resource_check_permissions(
        &procedure_id,
        &user,
        PermissionLevel::Execute,
      )
      .await?;

    let map = self.make_procedure_map(&procedure).await?;

    let mut update =
      make_update(&procedure, Operation::StopContainer, &user);
    update.in_progress();
    update.push_simple_log(
      "execute procedure",
      format!("Executing procedure: {}", procedure.name),
    );

    update.id = self.add_update(update.clone()).await?;

    let update = Mutex::new(update);

    let res = self.execute_procedure(&procedure, &map, &update).await;

    let mut update = update.into_inner();

    match res {
      Ok(_) => {
        update.push_simple_log(
          "execution ok",
          "the procedure has completed with no errors",
        );
      }
      Err(e) => update
        .push_error_log("execution error", serialize_error_pretty(e)),
    }

    update.finalize();

    self.update_update(update.clone()).await?;

    Ok(update)
  }
}
