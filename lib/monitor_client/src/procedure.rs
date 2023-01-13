use anyhow::Context;
use monitor_types::{Procedure, Update};
use serde_json::{json, Value};

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_procedures(
        &self,
        query: impl Into<Option<Value>>,
    ) -> anyhow::Result<Vec<Procedure>> {
        self.get("/api/procedure/list", query.into())
            .await
            .context("failed at list procedures")
    }

    pub async fn get_procedure(&self, procedure_id: &str) -> anyhow::Result<Procedure> {
        self.get(
            &format!("/api/procedure/{procedure_id}"),
            Option::<()>::None,
        )
        .await
    }

    pub async fn create_procedure(&self, name: &str) -> anyhow::Result<Procedure> {
        self.post("/api/procedure/create", json!({ "name": name }))
            .await
            .context(format!("failed at create procedure with name {name}"))
    }

    pub async fn create_full_procedure(&self, procedure: &Procedure) -> anyhow::Result<Procedure> {
        self.post::<&Procedure, _>("/api/procedure/create_full", procedure)
            .await
            .context(format!("failed at creating full procedure"))
    }

    pub async fn delete_procedure(&self, id: &str) -> anyhow::Result<Procedure> {
        self.delete::<(), _>(&format!("/api/procedure/{id}/delete"), None)
            .await
            .context(format!("failed at deleting procedure {id}"))
    }

    pub async fn update_procedure(&self, procedure: Procedure) -> anyhow::Result<Procedure> {
        self.patch("/api/procedure/update", procedure)
            .await
            .context("failed at updating procedure")
    }

    pub async fn run_procedure(&self, procedure_id: &str) -> anyhow::Result<Update> {
        self.post::<(), _>(&format!("/api/procedure/{procedure_id}/run"), None)
            .await
            .context(format!("failed at deploy procedure {procedure_id}"))
    }
}
