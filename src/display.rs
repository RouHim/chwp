use std::env;
use std::process::Command;

pub struct DisplayInfo {
    pub count: i8,
    pub resolutions: Vec<String>,
    pub total_resolution: String,
    pub max_single_resolution: String,
}

pub fn get_display_info() -> DisplayInfo {
    let display_resolutions = get_display_resolutions();
    // maxSingleResolution = get_Max_single_display_resolution();
    // totalResolution = get_total_resolution();

    return DisplayInfo {

        // Resolutions: displayResolutions,
        // Count: len(displayResolutions),
        // TotalResolution: totalResolution,
        // MaxSingleResolution: maxSingleResolution,
        count: 0,
        resolutions: vec![],
        total_resolution: "".to_string(),
        max_single_resolution: "".to_string(),
    };
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