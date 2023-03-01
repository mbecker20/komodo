use std::{borrow::Borrow, fs::File, io::Read, net::SocketAddr, str::FromStr};

use anyhow::{anyhow, Context};
use axum::http::StatusCode;
use rand::{distributions::Alphanumeric, Rng};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use types::Log;

pub fn parse_config_files<'a, T: DeserializeOwned>(
    paths: impl IntoIterator<Item = impl Borrow<String>>,
    merge_nested: bool,
    extend_array: bool,
) -> anyhow::Result<T> {
    let mut target = Map::new();
    for path in paths {
        target = merge_objects(
            target,
            parse_config_file(path.borrow())?,
            merge_nested,
            extend_array,
        )?;
    }
    serde_json::from_str(&serde_json::to_string(&target)?)
        .context("failed to parse final config into expected type")
}

pub fn parse_config_file<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
    let mut file = File::open(&path).expect(&format!("failed to find config at {path}"));
    let config = if path.ends_with("toml") {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context(format!("failed to read toml at {path}"))?;
        toml::from_str(&contents).context(format!("failed to parse toml at {path}"))?
    } else if path.ends_with("json") {
        serde_json::from_reader(file).context(format!("failed to parse json at {path}"))?
    } else {
        panic!("unsupported config file type: {}", path)
    };
    Ok(config)
}

/// object is serde_json::Map<String, serde_json::Value>
/// source will overide target
/// will recurse when field is object if merge_object = true, otherwise object will be replaced
/// will extend when field is array if extend_array = true, otherwise array will be replaced
/// will return error when types on source and target fields do not match
fn merge_objects(
    mut target: Map<String, Value>,
    source: Map<String, Value>,
    merge_nested: bool,
    extend_array: bool,
) -> anyhow::Result<Map<String, Value>> {
    for (key, value) in source {
        let curr = target.remove(&key);
        if curr.is_none() {
            target.insert(key, value);
            continue;
        }
        let curr = curr.unwrap();
        match curr {
            Value::Object(target_obj) => {
                if !merge_nested {
                    target.insert(key, value);
                    continue;
                }
                match value {
                    Value::Object(source_obj) => {
                        target.insert(
                            key,
                            Value::Object(merge_objects(
                                target_obj,
                                source_obj,
                                merge_nested,
                                extend_array,
                            )?),
                        );
                    }
                    _ => {
                        return Err(anyhow!(
                            "types on field {key} do not match. got {value:?}, expected object"
                        ))
                    }
                }
            }
            Value::Array(mut target_arr) => {
                if !extend_array {
                    target.insert(key, value);
                    continue;
                }
                match value {
                    Value::Array(source_arr) => {
                        target_arr.extend(source_arr);
                        target.insert(key, Value::Array(target_arr));
                    }
                    _ => {
                        return Err(anyhow!(
                            "types on field {key} do not match. got {value:?}, expected array"
                        ))
                    }
                }
            }
            _ => {
                target.insert(key, value);
            }
        }
    }
    Ok(target)
}

pub fn parse_comma_seperated_list<T: FromStr>(
    comma_sep_list: impl Borrow<str>,
) -> anyhow::Result<Vec<T>> {
    comma_sep_list
        .borrow()
        .split(",")
        .filter(|item| item.len() > 0)
        .map(|item| {
            let item = item
                .parse()
                .map_err(|_| anyhow!("error parsing string {item} into type T"))?;
            Ok::<T, anyhow::Error>(item)
        })
        .collect()
}

pub fn get_socket_addr(port: u16) -> SocketAddr {
    SocketAddr::from_str(&format!("0.0.0.0:{}", port)).expect("failed to parse socket addr")
}

pub fn to_monitor_name(name: &str) -> String {
    name.to_lowercase().replace(" ", "_")
}

pub fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal Error: {err:#?}"),
    )
}

pub fn generate_secret(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn all_logs_success(logs: &Vec<Log>) -> bool {
    for log in logs {
        if !log.success {
            return false;
        }
    }
    true
}
