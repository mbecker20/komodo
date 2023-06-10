use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "params")]
pub enum CoreRequest {}

pub trait HasResponse: Serialize + DeserializeOwned + std::fmt::Debug + Send + 'static {
    type Response: Serialize + DeserializeOwned + std::fmt::Debug;
    fn req_type() -> &'static str;
}

#[async_trait::async_trait]
pub trait Resolve<Req: HasResponse> {
    async fn resolve(&self, req: Req) -> anyhow::Result<Req::Response>;
    async fn resolve_to_json(&self, req: Req) -> anyhow::Result<String> {
        let res = self.resolve(req).await?;
        let res = serde_json::to_string(&res).context("failed at serializing response")?;
        Ok(res)
    }
}

#[async_trait::async_trait]
pub trait ResolveToString<Req: HasResponse> {
    async fn resolve_to_string(&self, req: Req) -> anyhow::Result<String>;
}

#[macro_export]
macro_rules! impl_has_response {
    ($req:ty, $res:ty) => {
        impl $crate::api::HasResponse for $req {
            type Response = $res;
            fn req_type() -> &'static str {
                stringify!($req)
            }
        }
    };
}
