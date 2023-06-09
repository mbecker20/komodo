use std::sync::Arc;

pub struct State {}

impl State {
    pub async fn load() -> anyhow::Result<Arc<State>> {
        let state = State {};

        Ok(state.into())
    }
}
