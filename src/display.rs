use std::env;
use winit::window::Window;

use crate::cli;

/// Container that holds information about the display current configuration
pub struct DisplayInfo {
    #[allow(dead_code)]
    pub count: i8,
    #[allow(dead_code)]
    pub resolutions: Vec<String>,
    pub total_resolution: String,
    pub max_single_resolution: String,
}

/// Gets the current display information
pub fn get_info(window: Window) -> DisplayInfo {
    let resolutions = get_display_resolutions(&window);
    let max_single_resolution = get_max_single_display_resolution(&window);
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
    env::var("XDG_SESSION_TYPE")
        .expect("Can't identify XDG_SESSION_TYPE")
        .eq("wayland")
}

/// Gets the maximum resolution of a single display
/// # Returns the maximum resolution of a single display
fn get_max_single_display_resolution(window: &Window) -> String {
    let resolutions = get_display_resolutions(window);
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
pub fn get_total_resolution() -> String {
    execute_display_command(
        r#"xprop -notype -len 16 -root _NET_DESKTOP_GEOMETRY | cut -c 25-"#,
    )
    .replace(", ", "x")
    .trim()
    .to_string()
}

/// Gets all available display resolutions
/// # Example: ["1920x1080", "2560x1440"]
pub fn get_display_resolutions(window: &Window) -> Vec<String> {
    window
        .available_monitors()
        .map(|monitor| format!("{}x{}", monitor.size().width, monitor.size().height))
        .collect()
}

/// Ensures that the DISPLAY variable is set and executes a command
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

/// Gets the width of a resolution string
/// # Arguments
/// * `resolution` - The resolution string
/// # Returns
/// The width of the resolution string
/// # Example
/// ```
/// use image_edit::get_width;
/// assert_eq!(get_width("1920x1080"), 1920);
/// ```
pub fn get_width(resolution_string: &str) -> String {
    resolution_string
        .split('x')
        .next()
        .expect("wrong display resolution format")
        .to_string()
}

/// Gets the height of a resolution string
/// # Arguments
/// * `resolution` - The resolution string
/// # Returns
/// The height of the resolution string
/// # Example
/// ```
/// use image_edit::get_height;
/// assert_eq!(get_height("1920x1080"), 1080);
/// ```
pub fn get_height(resolution_string: &str) -> String {
    resolution_string
        .split('x')
        .nth(1)
        .expect("wrong display resolution format")
        .to_string()
}
