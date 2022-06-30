use std::process::Command;

/// Execute a command and return the output
/// # Arguments
/// * `command` - The command to execute
/// # Returns
/// The output of the command
///
/// # Example
/// ```
/// use cli::execute_command;
///
/// let output = execute_command("echo hello");
/// assert_eq!(output, "hello\n");
/// ```
pub fn execute_command(cmd: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    format!("{stdout} \n {stderr}")
}
