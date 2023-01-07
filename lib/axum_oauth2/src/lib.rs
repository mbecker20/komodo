use std::time::Duration;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub mod github;
pub mod google;

pub fn random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn random_duration(min_ms: u64, max_ms: u64) -> Duration {
    Duration::from_millis(thread_rng().gen_range(min_ms..max_ms))
}
