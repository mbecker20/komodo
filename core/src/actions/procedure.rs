use anyhow::anyhow;
use types::{traits::Permissioned, PermissionLevel, Procedure, Update};

use crate::{auth::RequestUser, state::State};

impl State {
    pub async fn get_procedure_check_permissions(
        &self,
        procedure_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Procedure> {
        let procedure = self.db.get_procedure(procedure_id).await?;
        let permissions = procedure.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(procedure)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this procedure"
            ))
        }
    }

    pub async fn create_procedure(
        &self,
        name: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Procedure> {
        todo!()
    }

    pub async fn create_full_procedure(
        &self,
        mut full_procedure: Procedure,
        user: &RequestUser,
    ) -> anyhow::Result<Procedure> {
        let procedure = self.create_procedure(&full_procedure.name, user).await?;
        full_procedure.id = procedure.id;
        let procedure = self.update_procedure(full_procedure, user).await?;
        Ok(procedure)
    }

    pub async fn delete_procedure(
        &self,
        id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Procedure> {
        todo!()
    }

    pub async fn update_procedure(
        &self,
        new_procedure: Procedure,
        user: &RequestUser,
    ) -> anyhow::Result<Procedure> {
        let current_procedure = self
            .get_procedure_check_permissions(&new_procedure.id, user, PermissionLevel::Write)
            .await?;
        todo!()
    }

    pub async fn run_procedure(&self, id: &str, user: &RequestUser) -> anyhow::Result<Vec<Update>> {
        let procedure = self
            .get_procedure_check_permissions(id, user, PermissionLevel::Write)
            .await?;

        todo!()
    }
}
