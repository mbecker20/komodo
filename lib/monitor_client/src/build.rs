use anyhow::Context;
use monitor_types::{AwsBuilderConfig, Build, BuildActionState, BuildVersionsReponse, Update};
use serde_json::{json, Value};

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_builds(&self, query: impl Into<Option<Value>>) -> anyhow::Result<Vec<Build>> {
        self.get("/api/build/list", query.into())
            .await
            .context("failed at list builds")
    }

    pub async fn get_build(&self, build_id: &str) -> anyhow::Result<Build> {
        self.get(&format!("/api/build/{build_id}"), Option::<()>::None)
            .await
            .context(format!("failed at getting build {build_id}"))
    }

    pub async fn get_build_action_state(&self, build_id: &str) -> anyhow::Result<BuildActionState> {
        self.get(
            &format!("/api/build/{build_id}/action_state"),
            Option::<()>::None,
        )
        .await
        .context(format!(
            "failed at getting action state for build {build_id}"
        ))
    }

    pub async fn get_build_versions(
        &self,
        build_id: &str,
        page: u32,
        major: impl Into<Option<u32>>,
        minor: impl Into<Option<u32>>,
        patch: impl Into<Option<u32>>,
    ) -> anyhow::Result<BuildVersionsReponse> {
        self.get(
            &format!("/api/build/{build_id}/versions"),
            json!({ "page": page, "major": major.into(), "minor": minor.into(), "patch": patch.into() }),
        )
        .await
        .context("failed at getting build versions")
    }

    pub async fn create_build(&self, name: &str, server_id: &str) -> anyhow::Result<Build> {
        self.post(
            "/api/build/create",
            json!({ "name": name, "server_id": server_id }),
        )
        .await
        .context(format!(
            "failed at creating build with name {name} on server id {server_id}"
        ))
    }

    pub async fn create_full_build(&self, build: &Build) -> anyhow::Result<Build> {
        self.post::<&Build, _>("/api/build/create_full", build)
            .await
            .context(format!(
                "failed at creating full build with name {}",
                build.name
            ))
    }

    pub async fn copy_build(&self, id: &str, new_name: &str) -> anyhow::Result<Build> {
        self.post(
            &format!("/api/build/{id}/copy"),
            json!({ "name": new_name }),
        )
        .await
        .context(format!("failed at copying build {id}"))
    }

    pub async fn delete_build(&self, id: &str) -> anyhow::Result<Build> {
        self.delete::<(), _>(&format!("/api/build/{id}/delete"), None)
            .await
            .context(format!("failed at deleting build {id}"))
    }

    pub async fn update_build(&self, build: Build) -> anyhow::Result<Build> {
        self.patch("/api/build/update", build)
            .await
            .context("failed at updating build")
    }

    pub async fn build(&self, build_id: &str) -> anyhow::Result<Update> {
        self.post::<(), _>(&format!("/api/build/{build_id}/build"), None)
            .await
            .context(format!("failed at building build {build_id}"))
    }

    pub async fn get_aws_builder_defaults(&self) -> anyhow::Result<AwsBuilderConfig> {
        self.get("/api/build/aws_builder_defaults", Option::<()>::None)
            .await
            .context("failed at getting aws builder defaults")
    }

    // pub async fn reclone_build(&self, id: &str) -> anyhow::Result<Update> {
    //     self.post::<(), _>(&format!("/api/build/{id}/reclone"), None)
    //         .await
    //         .context(format!("failed at recloning build {id}"))
    // }
}
