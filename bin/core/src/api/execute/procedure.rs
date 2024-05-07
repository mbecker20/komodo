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
    procedure::execute_procedure,
    update::{add_update, make_update, update_update},
  }, resource, state::{action_states, State}
};

#[async_trait]
impl Resolve<RunProcedure, User> for State {
  #[instrument(name = "RunProcedure", skip(self, user))]
  async fn resolve(
    &self,
    RunProcedure { procedure }: RunProcedure,
    user: User,
  ) -> anyhow::Result<Update> {
    let procedure = resource::get_check_permissions::<Procedure>(
      &procedure,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // get the action state for the procedure (or insert default).
    let action_state = action_states()
      .procedure
      .get_or_insert_default(&procedure.id)
      .await;

    // This will set action state back to default when dropped.
    // Will also check to ensure procedure not already busy before updating.
    let _action_guard =
      action_state.update(|state| state.running = true)?;

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
      Err(e) => update.push_error_log(
        "execution error",
        serialize_error_pretty(&e),
      ),
    }

    update.finalize();

    update_update(update.clone()).await?;

    Ok(update)
  }
}
