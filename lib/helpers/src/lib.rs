pub fn to_monitor_name(name: &str) -> String {
    name.to_lowercase().replace(' ', "_")
}
