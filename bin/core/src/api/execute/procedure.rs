use std::pin::Pin;

use formatting::{bold, colored, format_serror, muted, Color};
use monitor_client::{
  api::execute::RunProcedure,
  entities::{
    permission::PermissionLevel, procedure::Procedure,
    update::Update, user::User,
  },
};
use mungos::{by_id::update_one_by_id, mongodb::bson::to_document};
use resolver_api::Resolve;
use tokio::sync::Mutex;

use crate::{
  helpers::{procedure::execute_procedure, update::update_update},
  resource::{self, refresh_procedure_state_cache},
  state::{action_states, db_client, State},
};

impl Resolve<RunProcedure, (User, Update)> for State {
  #[instrument(name = "RunProcedure", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RunProcedure { procedure }: RunProcedure,
    (user, update): (User, Update),
  ) -> anyhow::Result<Update> {
    resolve_inner(procedure, user, update).await
  }
}

fn resolve_inner(
  procedure: String,
  user: User,
  mut update: Update,
) -> Pin<
  Box<
    dyn std::future::Future<Output = anyhow::Result<Update>> + Send,
  >,
> {
  Box::pin(async move {
    let procedure = resource::get_check_permissions::<Procedure>(
      &procedure,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // Need to push the initial log, as execute_procedure
    // assumes first log is already created
    // and will panic otherwise.
    update.push_simple_log(
      "execute_procedure",
      format!(
        "{}: executing procedure '{}'",
        muted("INFO"),
        bold(&procedure.name)
      ),
    );

    // get the action state for the procedure (or insert default).
    let action_state = action_states()
      .procedure
      .get_or_insert_default(&procedure.id)
      .await;

    // This will set action state back to default when dropped.
    // Will also check to ensure procedure not already busy before updating.
    let _action_guard =
      action_state.update(|state| state.running = true)?;

    let update = Mutex::new(update);

    let res = execute_procedure(&procedure, &update).await;

    let mut update = update.into_inner();

    match res {
      Ok(_) => {
        update.push_simple_log(
          "execution ok",
          format!(
            "{}: the procedure has {} with no errors",
            muted("INFO"),
            colored("completed", Color::Green)
          ),
        );
      }
      Err(e) => update
        .push_error_log("execution error", format_serror(&e.into())),
    }

    update.finalize();

    // Need to manually update the update before cache refresh,
    // and before broadcast with add_update.
    // The Err case of to_document should be unreachable,
    // but will fail to update cache in that case.
    if let Ok(update_doc) = to_document(&update) {
      let _ = update_one_by_id(
        &db_client().await.updates,
        &update.id,
        mungos::update::Update::Set(update_doc),
        None,
      )
      .await;
      refresh_procedure_state_cache().await;
    }

    update_update(update.clone()).await?;

    Ok(update)
  })
}
