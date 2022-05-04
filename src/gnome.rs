use crate::cli;

pub fn write_settings(key: &String, value: &String) {
    if !is_settings_value_equals(key, value) {
        cli::execute_command(&["gsettings set", key, value].join(" "));
    }
}

pub fn is_settings_value_equals(key: &String, value: &String) -> bool {
    read_settings(key) == *value
}

fn read_settings(key: &String) -> String {
    cli::execute_command(&["gsettings get", key].join(" "))
}
