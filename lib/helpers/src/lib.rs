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
