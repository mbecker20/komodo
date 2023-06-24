use async_timing_util::unix_timestamp_ms;

pub fn to_monitor_name(name: &str) -> String {
    name.to_lowercase().replace(' ', "_")
}

pub fn monitor_timestamp() -> i64 {
    unix_timestamp_ms() as i64
}
