use std::sync::Arc;

use axum::{http::HeaderMap, Router};
use komodo_client::entities::resource::Resource;
use tokio::sync::Mutex;

use crate::{helpers::cache::Cache, resource::KomodoResource};

mod github;
mod gitlab;
mod resources;
mod router;

pub fn router() -> Router {
  Router::new()
    .nest("/github", router::router::<github::Github>())
    .nest("/gitlab", router::router::<gitlab::Gitlab>())
}

type ListenerLockCache = Cache<String, Arc<Mutex<()>>>;

trait ExtractBranch {
  fn extract_branch(body: &str) -> anyhow::Result<String>;
}

trait VerifySecret {
  fn verify_secret(
    headers: HeaderMap,
    body: &str,
    custom_secret: &str,
  ) -> anyhow::Result<()>;
}

trait CustomSecret: KomodoResource {
  fn custom_secret(
    resource: &Resource<Self::Config, Self::Info>,
  ) -> &str;
}
