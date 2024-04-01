use async_trait::async_trait;
use monitor_client::{
  api::execute::RunProcedure,
  entities::{
    permission::PermissionLevel, procedure::Procedure,
    update::Update, user::User, Operation,
  },
};
use resolver_api::Resolve;
use serror::serialize_error_pretty;
use tokio::sync::Mutex;

use crate::{
  helpers::{
    add_update, make_update, procedure::execute_procedure,
    resource::StateResource, update_update,
  },
  state::State,
};

#[async_trait]
impl Resolve<RunProcedure, User> for State {
  async fn resolve(
    &self,
    RunProcedure { procedure }: RunProcedure,
    user: User,
  ) -> anyhow::Result<Update> {
    let procedure = Procedure::get_resource_check_permissions(
      &procedure,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    let mut update =
      make_update(&procedure, Operation::RunProcedure, &user);
    update.in_progress();
    update.push_simple_log(
      "execute procedure",
      format!("Executing procedure: {}", procedure.name),
    );

    update.id = add_update(update.clone()).await?;

    let update = Mutex::new(update);

    let res = execute_procedure(&procedure, &update).await;

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

    update_update(update.clone()).await?;

    Ok(update)
  }
}
