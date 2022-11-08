use std::sync::Arc;

use axum::{Router, Extension};

pub fn router() -> Router {
	Router::new()
		
		.layer(StatsClient::extension())
}

type StatsExtension = Extension<Arc<StatsClient>>;

struct StatsClient {
	client: sysinfo::System,
}

impl StatsClient {
	pub fn extension() -> StatsExtension {
		let client = StatsClient {
			client: sysinfo::System::default(),
		};
		Extension(Arc::new(client))
	}
}