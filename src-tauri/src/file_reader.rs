use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file_contents(file_path: &str, scenario_name: Option<&str>) -> Result<String, String> {
    if !Path::new(file_path).is_file() {
        return Err("Invalid file path".to_string());
    }

    // Open the file
    let mut file = File::open(file_path).map_err(|_| "Failed to open file".to_string())?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| "Failed to read file".to_string())?;

    let mut lines_vec = Vec::new();
    let mut line_number = 1;

    if let Some(scenario) = scenario_name {
        // Find the scenarion in the file
        let mut lines = contents.lines();
        let mut in_scenario = false;

        while let Some(line) = lines.next() {
            if line.contains(scenario) {
                in_scenario = true;
            }
            if in_scenario {
                if line.trim().starts_with("Scenario") && !line.contains(scenario) {
                    break;
                }
                lines_vec.push((line_number, line.to_string()));
            }
            line_number += 1;
        }

        if lines_vec.is_empty() {
            return Err("Failed to find scenario".to_string());
        }
    } else {
        for (line_number, line) in contents.lines().enumerate() {
            lines_vec.push((line_number + 1, line.to_string()));
        }
    }

    // Remove trailing empty lines from lines_vec
    while let Some((_, line)) = lines_vec.last() {
        if line.trim().is_empty() {
            lines_vec.pop();
        } else {
            break;
        }
    }

    let mut result = String::new();
    for (line_number, line) in lines_vec {
        result.push_str(&format!("{}: {}\n", line_number, line));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read_file_contents_whole_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        let mut file = File::create(&path).unwrap();
        file.write_all("Line 1\nLine 2\nLine 3\n\n\n".as_bytes())
            .unwrap();

        let file_path = path.to_str().unwrap();
        let scenario_name = None;
        let result = read_file_contents(file_path, scenario_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1: Line 1\n2: Line 2\n3: Line 3\n");
    }

    #[test]
    fn test_read_file_contents_with_scenario() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        let mut file = File::create(&path).unwrap();
        file.write_all("Line 1\nScenario s1\nLine 2\nLine 3\n\n\nScenario s2\nLine 4\n".as_bytes())
            .unwrap();

        let file_path = path.to_str().unwrap();
        let scenario_name = Some("s1");
        let result = read_file_contents(&file_path, scenario_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2: Scenario s1\n3: Line 2\n4: Line 3\n");

        let scenario_name = Some("s2");
        let result = read_file_contents(&file_path, scenario_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "7: Scenario s2\n8: Line 4\n");
    }

    #[test]
    fn test_read_file_contents_scenario_not_found() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        let mut file = File::create(&path).unwrap();
        file.write_all("Line 1\nScenario s1\nLine 2\nLine 3\n".as_bytes())
            .unwrap();

        let file_path = path.to_str().unwrap();
        let scenario_name = Some("s2");
        let result = read_file_contents(file_path, scenario_name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to find scenario");
    }

    #[test]
    fn test_read_file_contents_invalid_path() {
        let file_path = "invalid_path.txt";
        let scenario_name = None;
        let result = read_file_contents(file_path, scenario_name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid file path");
    }
}
