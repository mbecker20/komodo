use anyhow::Context;
use monitor_types::{Deployment, DeploymentWithContainer, SystemStats, Update};
use serde::Serialize;
use serde_json::{json, Value};

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_deployments(
        &self,
        query: impl Into<Option<Value>>,
    ) -> anyhow::Result<Vec<DeploymentWithContainer>> {
        self.get("/api/deployment/list", query.into())
            .await
            .context("failed at list deployments")
    }

    pub async fn get_deployment(&self, deployment_id: &str) -> anyhow::Result<DeploymentWithContainer> {
        self.get(&format!("/api/deployment/{deployment_id}"), Option::<()>::None)
            .await
    }

    pub async fn create_deployment(
        &self,
        name: &str,
        server_id: &str,
    ) -> anyhow::Result<Deployment> {
        self.post(
            "/api/deployment/create",
            json!({ "name": name, "server_id": server_id }),
        )
        .await
        .context(format!(
            "failed at create_deployment with name {name} and server id {server_id}"
        ))
    }

    pub async fn delete_deployment(&self, id: &str) -> anyhow::Result<Deployment> {
        self.delete::<(), _>(&format!("/api/deployment/delete/{id}"), None)
            .await
            .context(format!("failed at deleting deployment {id}"))
    }

    pub async fn update_deployment(&self, deployment: Deployment) -> anyhow::Result<Deployment> {
        self.patch("/api/deployment/update", deployment)
            .await
            .context("failed at updating deployment")
    }

    pub async fn deploy(&self, deployment_id: &str) -> anyhow::Result<Update> {
        self.post::<(), _>(&format!("/api/deployment/deploy/{deployment_id}"), None)
            .await
            .context(format!("failed at deploy deployment {deployment_id}"))
    }

    pub async fn reclone_deployment(&self, id: &str) -> anyhow::Result<Update> {
        self.post::<(), _>(&format!("/api/deployment/reclone/{id}"), None)
            .await
            .context(format!("failed at reclone deployment {id}"))
    }
}
