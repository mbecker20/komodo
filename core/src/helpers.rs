use types::{PermissionLevel, PermissionsMap};

#[macro_export]
macro_rules! response {
    ($x:expr) => {
        Ok::<_, (axum::http::StatusCode, String)>($x)
    };
}

pub fn get_user_permissions(user_id: &str, permissions: &PermissionsMap) -> PermissionLevel {
    *permissions.get(user_id).unwrap_or(&PermissionLevel::None)
}
