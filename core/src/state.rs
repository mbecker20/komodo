use std::{os::linux::raw::stat, sync::Arc};

pub struct AppState {}

impl AppState {
    pub async fn load() -> anyhow::Result<Arc<AppState>> {
        let state = AppState {};

        Ok(state.into())
    }
}
