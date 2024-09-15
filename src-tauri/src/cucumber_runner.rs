use std::path::Path;
use std::process::{Command, Stdio};

pub fn run_cucumber(
    folder_path: &str,
    feature_file: &str,
    scenario_name: Option<&str>,
) -> Result<String, String> {
    let folder = Path::new(folder_path);
    let mut command = Command::new("npx");
    command.arg("cucumber-js").arg(feature_file);

    if let Some(name) = scenario_name {
        command.arg("--name").arg(name);
    }

    command.arg("-p").arg("local").current_dir(folder);

    let status = command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if status.success() {
        Ok("Command executed successfully".to_string())
    } else {
        Err(format!("Command failed with status: {}", status))
    }
}
