use async_timing_util::unix_timestamp_ms;
use monitor_types::entities::update::Log;

pub fn to_monitor_name(name: &str) -> String {
    name.to_lowercase().replace(' ', "_")
}

pub fn optional_string(string: &str) -> Option<String> {
    if string.is_empty() {
        None
    } else {
        Some(string.to_string())
    }
}

pub fn monitor_timestamp() -> i64 {
    unix_timestamp_ms() as i64
}

pub fn all_logs_success(logs: &Vec<Log>) -> bool {
    for log in logs {
        if !log.success {
            return false;
        }
    }
    true
}