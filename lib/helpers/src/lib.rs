use std::{fs::File, io::Read};

use serde::de::DeserializeOwned;

pub mod docker;
pub mod git;

pub fn parse_config_file<T: DeserializeOwned>(path: &str) -> T {
    let mut file = File::open(&path).expect(&format!("failed to find config at {path}"));
    if path.ends_with("toml") {
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(&format!("failed to read toml at {path}"));
        toml::from_str(&contents).expect(&format!("failed to parse toml at {path}"))
    } else if path.ends_with("json") {
        serde_json::from_reader(file).expect(&format!("failed to parse json at {path}"))
    } else {
        panic!("unsupported config file type: {}", path)
    }
}