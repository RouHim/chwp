use std::env;

use crate::cli;

pub struct DisplayInfo {
    pub display_count: u8,
    pub resolutions: Vec<String>,
    pub total_resolution: String,
    pub max_single_resolution: String,
}

pub fn get_info() -> DisplayInfo {
    let resolutions = get_display_resolutions();
    let max_single_resolution = get_max_single_display_resolution();
    let total_resolution = get_total_resolution();

    DisplayInfo {
        display_count: resolutions.len() as u8,
        resolutions,
        total_resolution,
        max_single_resolution,
    }
}

fn is_wayland() -> bool {
    let xdg_session_type = env::var("XDG_SESSION_TYPE");
    if xdg_session_type.is_err() {
        panic!("Can't identify XDG_SESSION_TYPE");
    }

    let xdg_session_type_value = xdg_session_type.unwrap();
    if xdg_session_type_value == "x11" {
        return false;
    } else if xdg_session_type_value == "wayland" {
        return true;
    }

    panic!("Can't identify XDG_SESSION_TYPE");
}

fn get_total_resolution() -> String {
    return
        if is_display_var_set() {
            cli::execute_command(
                &"(xrandr -q|sed -n 's/.*current[ ]\\([0-9]*\\) x \\([0-9]*\\),.*/\\1x\\2/p')".to_string()
            ).trim().to_string()
        } else {
            cli::execute_command(
                &"(DISPLAY=:0 xrandr -q|sed -n 's/.*current[ ]\\([0-9]*\\) x \\([0-9]*\\),.*/\\1x\\2/p')".to_string()
            ).trim().to_string()
        };
}

fn get_max_single_display_resolution() -> String {
    let resolutions = get_display_resolutions();
    let mut max_resolution = 0;
    let mut resolution_string = String::from("");

    for resolution in resolutions {
        let current_resolution = multiply_resolution(&resolution);

        if current_resolution > max_resolution {
            max_resolution = current_resolution;
            resolution_string = resolution;
        }
    }

    resolution_string
}

fn multiply_resolution(resolution: &str) -> i32 {
    let mut multiply = 1;

    resolution
        .split('x')
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|n| multiply *= n);

    multiply
}

fn get_display_resolutions() -> Vec<String> {
    let resolutions_string = execute_display_command("xrandr | grep \\* | cut -d' ' -f4".to_string())
        .trim()
        .to_string();

    return if resolutions_string.contains('\n') {
        resolutions_string.split('\n')
            .map(|s| s.to_string())
            .collect()
    } else {
        vec![resolutions_string]
    };
}

fn execute_display_command(cmd: String) -> String {
    if is_display_var_set() {
        cli::execute_command(&cmd)
    } else if is_wayland() {
        cli::execute_command(&(String::from("WAYLAND_DISPLAY=:wayland-0 ") + &cmd))
    } else {
        cli::execute_command(&(String::from("DISPLAY=:0 ") + &cmd))
    }
}

fn is_display_var_set() -> bool {
    if is_wayland() {
        env::var("WAYLAND_DISPLAY").is_ok()
    } else {
        env::var("DISPLAY").is_ok()
    }
}