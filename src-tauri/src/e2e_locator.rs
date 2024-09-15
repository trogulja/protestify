use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};

fn is_valid_package_json(package_json_path: &PathBuf) -> bool {
    if let Ok(contents) = fs::read_to_string(package_json_path) {
        if let Ok(json) = from_str::<Value>(&contents) {
            if let Some(url) = json.pointer("/repository/url").and_then(Value::as_str) {
                return url == "https://github.com/productiveio/e2e-tests.git";
            }
        }
    }
    false
}

pub fn is_target_valid(target: &str) -> bool {
    let path = Path::new(target);
    path.is_dir() && is_valid_package_json(&path.join("package.json"))
}

fn find_package_json_with_repo_url(entry_point: Option<&str>) -> Result<Option<String>, String> {
    let start_path = entry_point
        .map(PathBuf::from)
        .or_else(home_dir)
        .ok_or_else(|| "Could not find home directory".to_string())?;

    fn traverse_dir(dir: &Path) -> Result<Option<String>, String> {
        // println!("Traversing directory: {}", dir.display()); // Log the current directory

        // Skip directories that are known to not contain package.json
        if dir.ends_with("node_modules")
            || dir.ends_with("debug")
            || dir.ends_with("tmp")
            || dir.ends_with("test")
            || dir.ends_with("spec")
            || dir.ends_with("cache")
            || dir.ends_with("release")
            || dir.ends_with("Music")
            || dir.ends_with("Pictures")
            || dir.ends_with("Library")
            || dir.ends_with("Movies")
            || dir
                .file_name()
                .map_or(false, |name| name.to_string_lossy().starts_with('.'))
        {
            return Ok(None);
        }

        // Check if the current directory contains a package.json
        let package_json_path = dir.join("package.json");
        if package_json_path.exists() {
            if is_valid_package_json(&package_json_path) {
                return Ok(Some(dir.to_string_lossy().to_string()));
            }
            // Skip the rest of the folder if package.json is found but doesn't contain the required URL
            return Ok(None);
        }

        // Check if the current directory contains a .git folder and skip if it does
        if dir.join(".git").exists() {
            return Ok(None);
        }

        let entries = fs::read_dir(dir).ok();
        match entries {
            None => return Ok(None),
            Some(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(Some(result)) = traverse_dir(&path) {
                            return Ok(Some(result));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    traverse_dir(&start_path)
}

#[derive(Debug, Serialize, Deserialize)]
struct E2eEnv {
    password: String,
    api_url: String,
    app_url: String,
}

fn read_from_env_file(file_path: &str) -> Result<E2eEnv, String> {
    let path = Path::new(file_path);
    let file = fs::File::open(&path).map_err(|_| "File not found".to_string())?;
    let reader = std::io::BufReader::new(file);

    let mut password = String::new();
    let mut api_url = String::new();
    let mut app_url = String::new();

    for line in reader.lines().flatten() {
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().to_string();
            match key {
                "PASSWORD" => password = value,
                "DEFAULT_API_HOST" => api_url = value,
                "DEFAULT_APP_HOST" => app_url = value,
                _ => {}
            }
        }
    }

    Ok(E2eEnv {
        password,
        api_url,
        app_url,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoInformation {
    location: String,
    password: String,
    api_url: String,
    app_url: String,
}

pub fn locate_e2e_repo(entry_point: Option<&str>) -> Result<RepoInformation, String> {
    let location = match find_package_json_with_repo_url(entry_point) {
        Ok(Some(location)) => location,
        Ok(None) => return Err("No valid repo found".to_string()),
        Err(e) => return Err(e),
    };

    let env_file_path = Path::new(&location).join(".env");
    let env = match read_from_env_file(env_file_path.to_str().unwrap()) {
        Ok(env) => env,
        Err(e) => return Err(e),
    };

    let repo_info = RepoInformation {
        location,
        password: env.password,
        api_url: env.api_url,
        app_url: env.app_url,
    };

    Ok(repo_info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_validator_for_valid_json() {
        let dir = tempdir().unwrap();
        let valid_file_path = dir.path().join("package.json");
        let valid_dir_str = dir.path().to_str().unwrap();
        let mut valid_file = File::create(&valid_file_path).unwrap();
        writeln!(
            valid_file,
            r#"{{
                "repository": {{
                    "url": "https://github.com/productiveio/e2e-tests.git"
                }}
            }}"#
        )
        .unwrap();

        assert!(is_valid_package_json(&valid_file_path));
        assert!(is_target_valid(valid_dir_str));
    }

    #[test]
    fn test_validator_for_invalid_json() {
        let dir = tempdir().unwrap();
        let invalid_file_path = dir.path().join("invalid_package.json");
        let invalid_dir_str = dir.path().to_str().unwrap();
        let mut invalid_file = File::create(&invalid_file_path).unwrap();
        writeln!(
            invalid_file,
            r#"{{
                "repository": {{
                    "url": "https://github.com/other/repo.git"
                }}
            }}"#
        )
        .unwrap();

        assert!(!is_valid_package_json(&invalid_file_path));
        assert!(!is_target_valid(invalid_dir_str));
    }

    #[test]
    fn test_validator_for_invalid_file() {
        let dir = tempdir().unwrap();
        let invalid_file_path = dir.path().join("invalid_package.txt");
        let invalid_dir_str = dir.path().to_str().unwrap();
        let mut invalid_file = File::create(&invalid_file_path).unwrap();
        writeln!(
            invalid_file,
            r#" Here is some file
            That is not valid json
            "#
        )
        .unwrap();

        assert!(!is_valid_package_json(&invalid_file_path));
        assert!(!is_target_valid(invalid_dir_str));
    }

    #[test]
    fn test_validator_for_missing_file() {
        let dir = tempdir().unwrap();
        let missing_file_path = dir.path().join("non_existent_package.json");
        let missing_dir_str = dir.path().join("missing_dir");
        assert!(!is_valid_package_json(&missing_file_path));
        assert!(!is_target_valid(missing_dir_str.to_str().unwrap()));
    }

    #[test]
    fn test_locator_when_entry_exists() {
        let dir = tempdir().unwrap();
        let entry_dir = dir.path().join("entry_dir");
        fs::create_dir(&entry_dir).unwrap();

        let invalid_subdir = entry_dir.as_path().join("invalid_subdir");
        fs::create_dir(&invalid_subdir).unwrap();
        let invalid_file_path = invalid_subdir.join("package.json");

        let mut invalid_file = File::create(&invalid_file_path).unwrap();
        writeln!(
            invalid_file,
            r#"{{
                "repository": {{
                    "url": "https://github.com/other/repo.git"
                }}
            }}"#
        )
        .unwrap();

        assert_eq!(
            find_package_json_with_repo_url(Some(entry_dir.as_path().to_str().unwrap())).unwrap(),
            None
        );

        let subdir = entry_dir.as_path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let file_path = subdir.join("package.json");

        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            r#"{{
                "repository": {{
                    "url": "https://github.com/productiveio/e2e-tests.git"
                }}
            }}"#
        )
        .unwrap();

        assert_eq!(
            find_package_json_with_repo_url(Some(entry_dir.as_path().to_str().unwrap())).unwrap(),
            Some(subdir.to_string_lossy().to_string())
        );
    }

    #[test]
    fn test_locator_when_entry_does_not_exist() {
        let dir = tempdir().unwrap();
        let entry_dir = dir.path().join("something_that_doesnt_exist");

        assert_eq!(
            find_package_json_with_repo_url(Some(entry_dir.as_path().to_str().unwrap())).unwrap(),
            None
        );
    }

    #[test]
    fn test_read_from_env_file_valid() {
        let dir = tempdir().unwrap();
        let env_file_path = dir.path().join(".env");
        let mut env_file = File::create(&env_file_path).unwrap();
        writeln!(
            env_file,
            "PASSWORD=my_password\nDEFAULT_API_HOST=https://api.example.com\nDEFAULT_APP_HOST=https://app.example.com"
        )
        .unwrap();

        let result = read_from_env_file(env_file_path.to_str().unwrap());
        assert!(result.is_ok());
        let env = result.unwrap();
        assert_eq!(env.password, "my_password");
        assert_eq!(env.api_url, "https://api.example.com");
        assert_eq!(env.app_url, "https://app.example.com");
    }

    #[test]
    fn test_read_from_env_file_missing_keys() {
        let dir = tempdir().unwrap();
        let env_file_path = dir.path().join(".env");
        let mut env_file = File::create(&env_file_path).unwrap();
        writeln!(
            env_file,
            "PASSWORD=my_password\nDEFAULT_API_HOST=https://api.example.com"
        )
        .unwrap();

        let result = read_from_env_file(env_file_path.to_str().unwrap());
        assert!(result.is_ok());
        let env = result.unwrap();
        assert_eq!(env.password, "my_password");
        assert_eq!(env.api_url, "https://api.example.com");
        assert_eq!(env.app_url, ""); // DEFAULT_APP_HOST is missing
    }

    #[test]
    fn test_read_from_env_file_invalid_format() {
        let dir = tempdir().unwrap();
        let env_file_path = dir.path().join(".env");
        let mut env_file = File::create(&env_file_path).unwrap();
        writeln!(env_file, "This is not a valid env file format").unwrap();

        let result = read_from_env_file(env_file_path.to_str().unwrap());
        assert!(result.is_ok());
        let env = result.unwrap();
        assert_eq!(env.password, ""); // No valid keys found
        assert_eq!(env.api_url, ""); // No valid keys found
        assert_eq!(env.app_url, ""); // No valid keys found
    }

    #[test]
    fn test_read_from_env_file_not_found() {
        let dir = tempdir().unwrap();
        let env_file_path = dir.path().join("non_existent.env");

        let result = read_from_env_file(env_file_path.to_str().unwrap());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "File not found");
    }

    #[test]
    fn test_locate_e2e_repo_valid() {
        let dir = tempdir().unwrap();
        let entry_dir = dir.path().join("entry_dir");
        fs::create_dir(&entry_dir).unwrap();

        let subdir = entry_dir.as_path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let package_json_path = subdir.join("package.json");
        let env_file_path = subdir.join(".env");

        let mut package_json_file = File::create(&package_json_path).unwrap();
        writeln!(
            package_json_file,
            r#"{{
                "repository": {{
                    "url": "https://github.com/productiveio/e2e-tests.git"
                }}
            }}"#
        )
        .unwrap();

        let mut env_file = File::create(&env_file_path).unwrap();
        writeln!(
            env_file,
            "PASSWORD=my_password\nDEFAULT_API_HOST=https://api.example.com\nDEFAULT_APP_HOST=https://app.example.com"
        )
        .unwrap();

        let result = locate_e2e_repo(Some(entry_dir.as_path().to_str().unwrap()));
        assert!(result.is_ok());
        let repo_info = result.unwrap();
        assert_eq!(repo_info.location, subdir.to_string_lossy().to_string());
        assert_eq!(repo_info.password, "my_password");
        assert_eq!(repo_info.api_url, "https://api.example.com");
        assert_eq!(repo_info.app_url, "https://app.example.com");
    }

    #[test]
    fn test_locate_e2e_repo_missing_env_file() {
        let dir = tempdir().unwrap();
        let entry_dir = dir.path().join("entry_dir");
        fs::create_dir(&entry_dir).unwrap();

        let subdir = entry_dir.as_path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let package_json_path = subdir.join("package.json");

        let mut package_json_file = File::create(&package_json_path).unwrap();
        writeln!(
            package_json_file,
            r#"{{
                "repository": {{
                    "url": "https://github.com/productiveio/e2e-tests.git"
                }}
            }}"#
        )
        .unwrap();

        let result = locate_e2e_repo(Some(entry_dir.as_path().to_str().unwrap()));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "File not found");
    }

    #[test]
    fn test_locate_e2e_repo_invalid_repo() {
        let dir = tempdir().unwrap();
        let entry_dir = dir.path().join("entry_dir");
        fs::create_dir(&entry_dir).unwrap();

        let subdir = entry_dir.as_path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let package_json_path = subdir.join("package.json");

        let mut package_json_file = File::create(&package_json_path).unwrap();
        writeln!(
            package_json_file,
            r#"{{
                "repository": {{
                    "url": "https://github.com/other/repo.git"
                }}
            }}"#
        )
        .unwrap();

        let result = locate_e2e_repo(Some(entry_dir.as_path().to_str().unwrap()));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "No valid repo found");
    }

    #[test]
    fn test_locate_e2e_repo_entry_does_not_exist() {
        let dir = tempdir().unwrap();
        let entry_dir = dir.path().join("non_existent_dir");

        let result = locate_e2e_repo(Some(entry_dir.as_path().to_str().unwrap()));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "No valid repo found");
    }
}
