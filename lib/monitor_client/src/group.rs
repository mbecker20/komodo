use anyhow::Context;
use monitor_types::Group;
use serde_json::{json, Value};

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_groups(&self, query: impl Into<Option<Value>>) -> anyhow::Result<Vec<Group>> {
        self.get("/api/group/list", query.into())
            .await
            .context("failed at list groups")
    }

    pub async fn get_group(&self, group_id: &str) -> anyhow::Result<Group> {
        self.get(&format!("/api/group/{group_id}"), Option::<()>::None)
            .await
    }

    pub async fn create_group(&self, name: &str) -> anyhow::Result<Group> {
        self.post("/api/group/create", json!({ "name": name }))
            .await
            .context(format!("failed at create group with name {name}"))
    }

    pub async fn create_full_group(&self, group: &Group) -> anyhow::Result<Group> {
        self.post::<&Group, _>("/api/group/create_full", group)
            .await
            .context(format!("failed at creating full group"))
    }

    pub async fn delete_group(&self, id: &str) -> anyhow::Result<Group> {
        self.delete::<(), _>(&format!("/api/group/{id}/delete"), None)
            .await
            .context(format!("failed at deleting group {id}"))
    }

    pub async fn update_group(&self, group: Group) -> anyhow::Result<Group> {
        self.patch("/api/group/update", group)
            .await
            .context("failed at updating group")
    }
}
