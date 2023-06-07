use serde::{de::DeserializeOwned, Serialize};

pub trait HasResponse: Serialize + std::fmt::Debug {
    type Response: DeserializeOwned + std::fmt::Debug;
    fn req_type() -> &'static str;
}

#[macro_export]
macro_rules! impl_has_response {
    ($req:ty, $res:ty) => {
        impl $crate::HasResponse for $req {
            type Response = $res;
            fn req_type() -> &'static str {
                stringify!($t)
            }
        }
    };
}
