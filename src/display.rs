use std::fs::DirEntry;
use std::{env, fs};

use crate::cli;

/// Container that holds information about the display
pub struct DisplayInfo {
    pub count: i8,
    pub resolutions: Vec<String>,
    pub total_resolution: String,
    pub max_single_resolution: String,
}

/// Gets the current display information
pub fn get_info() -> DisplayInfo {
    let resolutions = get_display_resolutions();
    let max_single_resolution = get_max_single_display_resolution();
    let total_resolution = get_total_resolution();

    DisplayInfo {
        count: resolutions.len() as i8,
        resolutions,
        total_resolution,
        max_single_resolution,
    }
}

/// Checks if the session is running on wayland
/// # Returns true if the session is running on wayland
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

/// Gets the maximum resolution of a single display
/// # Returns the maximum resolution of a single display
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

/// Multiplies the resolution of all displays
/// # Arguments
/// * `resolution_string` - The resolution to multiply
/// # Returns the multiplied resolution
/// # Example
/// ```
/// let display_ratio = multiply_resolution("1920x1080");
/// assert_eq!(display_ratio, 1920 * 1080);
/// ```
fn multiply_resolution(resolution: &str) -> i32 {
    let mut multiply = 1;

    resolution
        .split('x')
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|n| multiply *= n);

    multiply
}

/// Gets the total desktop resolution.
/// # Example Two desktops (1) 1920x1080 (2) 1920x1080 | get_total_resolution() -> "3840x1080"
pub(crate) fn get_total_resolution() -> String {
    return execute_display_command(
        r#"xprop -notype -len 16 -root _NET_DESKTOP_GEOMETRY | cut -c 25-"#,
    )
    .replace(", ", "x")
    .trim()
    .to_string();
}

/// Gets all resolutions
/// # Example: ["1920x1080", "2560x1440"]
pub(crate) fn get_display_resolutions() -> Vec<String> {
    let paths = std::fs::read_dir("/sys/class/drm/").unwrap();

    paths
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .filter(is_connected)
        .filter_map(to_primary_mode)
        .collect()

    //FIXME: does not work: this way misses the orientation :(
}

fn to_primary_mode(drm_dir: DirEntry) -> Option<String> {
    let drm_path = drm_dir.path().read_dir().unwrap();
    let modes_value = drm_path
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().is_file() && entry.file_name().to_str().unwrap_or("").eq("modes")
        })
        .map(|status_file| fs::read_to_string(status_file.path()))
        .next();

    if let Some(Ok(modes_value)) = modes_value {
        return modes_value.trim().split('\n').map(|str| str.to_string()).next();
    }

    return None;
}

fn is_connected(drm_dir: &DirEntry) -> bool {
    let drm_path = drm_dir.path().read_dir().unwrap();
    let status_value = drm_path
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().is_file() && entry.file_name().to_str().unwrap_or("").eq("status")
        })
        .map(|status_file| fs::read_to_string(status_file.path()))
        .next();

    if let Some(Ok(status_value)) = status_value {
        return status_value.trim().eq("connected");
    }

    false
}

/// Checks if the DISPLAY variable is set and executes a command
fn execute_display_command(cmd: &str) -> String {
    if is_display_var_set() {
        cli::execute_command(cmd)
    } else if is_wayland() {
        cli::execute_command(format!("WAYLAND_DISPLAY=:wayland-0 {cmd}").as_str())
    } else {
        cli::execute_command(format!("DISPLAY=:0  {cmd}").as_str())
    }
}

/// Checks if the DISPLAY or WAYLAND_DISPLAY variable is set
fn is_display_var_set() -> bool {
    if is_wayland() {
        env::var("WAYLAND_DISPLAY").is_ok()
    } else {
        env::var("DISPLAY").is_ok()
    }
}
