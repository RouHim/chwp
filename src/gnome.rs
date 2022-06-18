use crate::cli;

/// Writes gnome settings
pub fn write_settings(key: &String, value: &String) {
    if !is_settings_value_equals(key, value) {
        cli::execute_command(&["gsettings set", key, value].join(" "));
    }
}

/// Checks if the value of a gnome setting is equal to the given value
pub fn is_settings_value_equals(key: &String, value: &String) -> bool {
    read_settings(key) == *value
}

/// Reads the value of a gnome setting
fn read_settings(key: &String) -> String {
    cli::execute_command(&["gsettings get", key].join(" "))
}
