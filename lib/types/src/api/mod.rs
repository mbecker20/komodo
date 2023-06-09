use anyhow::Context;
use axum::{headers::ContentType, http::StatusCode, TypedHeader};
use serde::{de::DeserializeOwned, Serialize};

pub mod core;
pub mod periphery;

pub trait HasResponse: Serialize + DeserializeOwned + std::fmt::Debug + Send + 'static {
    type Response: Serialize + DeserializeOwned + std::fmt::Debug;
    fn req_type() -> &'static str;
}

#[async_trait::async_trait]
pub trait Resolve<Req: HasResponse> {
    async fn resolve(&self, req: Req) -> anyhow::Result<Req::Response>;
    async fn resolve_to_response(
        &self,
        req: Req,
    ) -> Result<(TypedHeader<ContentType>, String), (StatusCode, String)> {
        let res = self
            .resolve(req)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")))?;
        let res = serde_json::to_string(&res)
            .context("failed at serializing response")
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")))?;
        Ok((TypedHeader(ContentType::json()), res))
    }
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
