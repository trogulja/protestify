use regex::Regex;
use serde::Serialize;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Clone)]
pub struct StepDefinition {
    pub id: String,
    pub keyword: String,
    pub pattern: String,
    pub file_path: String,
    pub line_number: usize,
    pub category: String,
    pub is_problematic: bool,
    pub problem_reason: Option<String>,
}

/// Parse all step definition files in the given directory
pub fn parse_step_definitions(base_path: &str) -> Result<Vec<StepDefinition>, String> {
    let step_defs_path = Path::new(base_path).join("step-definitions");

    if !step_defs_path.exists() {
        return Err(format!(
            "step_definitions folder not found at: {}",
            step_defs_path.display()
        ));
    }

    let mut steps = Vec::new();
    // Match both regex patterns /.../ and string patterns '...' or "..."
    // Also handle optional generic type like <ScenarioContext>
    let step_regex = Regex::new(r#"(Given|When|Then|And|But)(?:<[^>]*>)?\s*\(\s*(?:/(.+?)/|['"](.+?)['"])"#).unwrap();

    for entry in WalkDir::new(&step_defs_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Only process TypeScript files
        if path.extension().map_or(false, |ext| ext == "ts" || ext == "js") {
            if let Ok(content) = fs::read_to_string(path) {
                let file_path = path.to_string_lossy().to_string();

                for (line_number, line) in content.lines().enumerate() {
                    if let Some(captures) = step_regex.captures(line) {
                        let keyword = captures.get(1).unwrap().as_str().to_string();
                        // Pattern is either in group 2 (regex /.../) or group 3 (string '...' or "...")
                        let pattern = captures
                            .get(2)
                            .or_else(|| captures.get(3))
                            .map(|m| m.as_str().to_string())
                            .unwrap_or_default();

                        if pattern.is_empty() {
                            continue;
                        }

                        let category = categorize_step(&pattern, &keyword);
                        let (is_problematic, problem_reason) = check_problematic(&pattern);

                        let id = format!("{}:{}", file_path, line_number + 1);

                        steps.push(StepDefinition {
                            id,
                            keyword,
                            pattern,
                            file_path: file_path.clone(),
                            line_number: line_number + 1,
                            category,
                            is_problematic,
                            problem_reason,
                        });
                    }
                }
            }
        }
    }

    Ok(steps)
}

fn categorize_step(pattern: &str, keyword: &str) -> String {
    let pattern_lower = pattern.to_lowercase();

    // Navigation
    if pattern_lower.contains("navigate")
        || pattern_lower.contains("visit")
        || pattern_lower.contains("go to")
        || pattern_lower.contains("open")
        || pattern_lower.contains("click")
    {
        return "Navigation".to_string();
    }

    // Waits
    if pattern_lower.contains("wait")
        || pattern_lower.contains("delay")
        || pattern_lower.contains("sleep")
    {
        return "Waits".to_string();
    }

    // Assertions
    if keyword == "Then"
        || pattern_lower.contains("should")
        || pattern_lower.contains("verify")
        || pattern_lower.contains("expect")
        || pattern_lower.contains("assert")
        || pattern_lower.contains("see")
        || pattern_lower.contains("visible")
    {
        return "Assertions".to_string();
    }

    // Data Setup
    if pattern_lower.contains("create")
        || pattern_lower.contains("set")
        || pattern_lower.contains("add")
        || pattern_lower.contains("insert")
        || pattern_lower.contains("generate")
        || pattern_lower.contains("prepare")
    {
        return "Data Setup".to_string();
    }

    // Feature Flags
    if pattern_lower.contains("flag")
        || pattern_lower.contains("feature")
        || pattern_lower.contains("toggle")
    {
        return "Flags".to_string();
    }

    // Actions
    if keyword == "When"
        || pattern_lower.contains("type")
        || pattern_lower.contains("enter")
        || pattern_lower.contains("select")
        || pattern_lower.contains("submit")
        || pattern_lower.contains("fill")
    {
        return "Actions".to_string();
    }

    "Other".to_string()
}

fn check_problematic(pattern: &str) -> (bool, Option<String>) {
    let pattern_lower = pattern.to_lowercase();

    // Check for explicit waits with long durations
    if pattern_lower.contains("wait") {
        // Look for numbers that might indicate long waits
        let number_regex = Regex::new(r"(\d+)").unwrap();
        if let Some(captures) = number_regex.captures(pattern) {
            if let Ok(num) = captures.get(1).unwrap().as_str().parse::<u32>() {
                if num > 5000 {
                    return (
                        true,
                        Some(format!("Long explicit wait: {}ms", num)),
                    );
                }
            }
        }
        if pattern_lower.contains("seconds") || pattern_lower.contains("sec") {
            return (
                true,
                Some("Uses explicit wait with seconds".to_string()),
            );
        }
    }

    // Check for deprecated patterns
    if pattern_lower.contains("deprecated") {
        return (
            true,
            Some("Deprecated step pattern".to_string()),
        );
    }

    // Check for overly complex regex
    let special_chars = pattern.chars().filter(|c| {
        matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | '|' | '?' | '*' | '+')
    }).count();

    if special_chars > 10 {
        return (
            true,
            Some("Overly complex regex pattern".to_string()),
        );
    }

    (false, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_step() {
        assert_eq!(categorize_step("navigate to homepage", "Given"), "Navigation");
        assert_eq!(categorize_step("wait for 5 seconds", "When"), "Waits");
        assert_eq!(categorize_step("should see success message", "Then"), "Assertions");
        assert_eq!(categorize_step("create a new user", "Given"), "Data Setup");
        assert_eq!(categorize_step("enable feature flag", "When"), "Flags");
        assert_eq!(categorize_step("type in the input", "When"), "Actions");
    }

    #[test]
    fn test_check_problematic() {
        let (prob, _) = check_problematic("wait for 10000 milliseconds");
        assert!(prob);

        let (prob, _) = check_problematic("click button");
        assert!(!prob);
    }
}
