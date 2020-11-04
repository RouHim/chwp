use std::env;
use std::process::Command;

pub struct DisplayInfo {
    pub count: i8,
    pub resolutions: Vec<String>,
    pub total_resolution: String,
    pub max_single_resolution: String,
}

pub fn get_display_info() -> DisplayInfo {
    let resolutions = get_display_resolutions();
    let max_single_resolution = get_max_single_display_resolution();
    let total_resolution = get_total_resolution();

    return DisplayInfo {
        count: resolutions.len() as i8,
        resolutions,
        total_resolution,
        max_single_resolution,
    };
}

fn get_total_resolution() -> String {
    return
        if is_display_var_set() {
            execute_command(
                "(xrandr -q|sed -n 's/.*current[ ]\\([0-9]*\\) x \\([0-9]*\\),.*/\\1x\\2/p')".to_string()
            ).trim().to_string()
        } else {
            execute_command(
                "(DISPLAY=:0 xrandr -q|sed -n 's/.*current[ ]\\([0-9]*\\) x \\([0-9]*\\),.*/\\1x\\2/p')".to_string()
            ).trim().to_string()
        };
}

fn get_max_single_display_resolution() -> String {
    let resolutions = get_display_resolutions();
    let mut max_resolution = 0;
    let mut resolution_string = String::from("");


    for resolution in resolutions.to_owned() {
        let current_resolution = multiply_resolution(resolution.clone());

        if current_resolution > max_resolution {
            max_resolution = current_resolution;
            resolution_string = resolution;
        }
    }

    return resolution_string;
}

fn multiply_resolution(resolution: String) -> i32 {
    let mut multiply = 1;

    resolution
        .split("x")
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|n| multiply *= n);

    return multiply;
}

fn get_display_resolutions() -> Vec<String> {
    let resolutions_string = execute_display_command("xrandr | grep \\* | cut -d' ' -f4".to_string())
        .trim()
        .to_string();

    return if resolutions_string.contains("\n") {
        resolutions_string.split("\n")
            .map(|s| s.to_string())
            .collect()
    } else {
        vec![resolutions_string]
    };
}

fn execute_display_command(cmd: String) -> String {
    return if is_display_var_set() {
        execute_command(cmd)
    } else {
        let s1 = String::from("DISPLAY=:0 ");
        execute_command((s1 + &cmd))
    };
}

fn execute_command(cmd: String) -> String {
    let result = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    let vec = result.stdout.to_owned();

    return String::from_utf8_lossy(&vec).to_string();
}

fn is_display_var_set() -> bool {
    match env::var("ENVIRONMENT_VARIABLE") {
        Ok(s) => s == "yes",
        _ => false
    }
}