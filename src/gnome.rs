use crate::cli;

/// Writes gnome settings
pub fn write_settings(key: &str, value: &str) {
    if !is_settings_value_equals(key, value) {
        cli::execute_command(format!("gsettings set {key} {value}").as_str());
    }
}

/// Checks if the value of a gnome setting is equal to the given value
pub fn is_settings_value_equals(key: &str, value: &str) -> bool {
    read_settings(key) == *value
}

/// Reads the value of a gnome setting
fn read_settings(key: &str) -> String {
    cli::execute_command(format!("gsettings get {key}").as_str())
}
