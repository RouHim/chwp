use crate::cli;

pub fn write_settings(key: &str, value: &str) {
    if !is_settings_value_equals(key, value) {
        cli::execute_command(&["gsettings set", key, value].join(" "));
    }
}

pub fn is_settings_value_equals(key: &str, value: &str) -> bool {
    read_settings(key) == *value
}

fn read_settings(key: &str) -> String {
    cli::execute_command(&["gsettings get", key].join(" "))
}