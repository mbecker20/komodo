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
