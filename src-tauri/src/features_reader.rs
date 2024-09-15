use lazy_static::lazy_static;
use phf::phf_map;
use regex::Regex;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use uuid::Uuid;
use walkdir::WalkDir;

#[derive(Serialize)]
pub struct Feature {
    pub id: String,
    pub name: String,
    pub description: String,
    pub file_path: String,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct Scenario {
    pub id: String,
    pub name: String,
    pub description: String,
    pub targets: BTreeSet<String>,
    pub steps: u32,
    pub examples: u32,
    pub tags: Vec<String>,
    pub feature_id: String,
    pub organization_name: String,
}

#[derive(Debug, PartialEq)]
enum Context {
    None,
    Feature,
    Scenario,
    ScenarioSteps,
    ExamplesHeader,
    ExamplesBody,
}

const STEP_KEYWORDS: [&str; 6] = ["Given ", "When ", "Then ", "And ", "But ", "* "];

static SCREEN_ENDPOINTS: phf::Map<&'static str, &'static str> = phf_map! {
    "budgets" => "/financials/budgets",
    "companies" => "/contacts/companies",
    "company expenses" => "/financials/expenses",
    "company time" => "/time/company",
    "contacts" => "/contacts/people",
    "dashboards" => "/dashboards",
    "deals" => "/sales/deals",
    "docs" => "/docs",
    "expense approvals" => "/approvals/expenses",
    "home" => "",
    "inbound emails" => "/emails",
    "insights" => "/insights",
    "invoices" => "/financials/invoices",
    "jobs" => "/jobs/deals",
    "my expenses" => "/expenses/me",
    "my time" => "/time/me",
    "payments" => "/financials/payments",
    "projects" => "/projects",
    "reports" => "/reports",
    "request time off" => "/time-off-requests",
    "scheduling" => "/scheduling/bookings",
    "settings" => "/settings",
    "tasks" => "/tasks",
    "template center" => "/templates",
    "time approvals" => "/approvals/time-entries",
    "time off approvals" => "/approvals/time-off-requests",
};

lazy_static! {
    // Keyword current organization is {string}
    static ref ORGANIZATION_RE: Regex = Regex::new(r#"current organization is "([^"]+)""#).unwrap();
    // Keyword {user}( is) on a {string} screen
    // Keyword {user}( is) on a {string} screen and flag(s) {string} is enabled
    // Keyword {user}( is) on a {string} screen on date {string}
    // Keyword {user}( is) on a {string} screen on date {string} and flag(s) {string} is enabled
    static ref SCREEN_RE: Regex = Regex::new(r#"on a "([^"]+)" screen"#).unwrap();
    static ref DATE_RE: Regex = Regex::new(r#"on date "([^"]+)""#).unwrap();
    static ref FLAGS_RE: Regex = Regex::new(r#"and flags? "([^"]+)" (?:is|are) enabled"#).unwrap();
}

fn capture_group<'a>(re: &Regex, text: &'a str) -> Option<&'a str> {
    re.captures(text)
        .and_then(|caps| caps.get(1).map(|m| m.as_str()))
}

fn get_target_string(name: &str, flag: Option<&str>, date: Option<&str>) -> String {
    let endpoint = SCREEN_ENDPOINTS.get(name).unwrap_or(&"");

    format!(
        "{};{};{};{}",
        name,
        endpoint,
        flag.unwrap_or_default(),
        date.unwrap_or_default()
    )
}

pub fn get_all_features(base_path: &str) -> Result<(Vec<Feature>, Vec<Scenario>)> {
    let mut features = Vec::new();
    let mut scenarios = Vec::new();

    for entry in WalkDir::new(base_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("feature") {
            let (file_feature, file_scenarios) = read_feature_files(path)?;
            features.push(file_feature);
            scenarios.extend(file_scenarios);
        }
    }

    Ok((features, scenarios))
}

fn read_feature_files<P: AsRef<Path>>(path: P) -> Result<(Feature, Vec<Scenario>)> {
    let file = File::open(path.as_ref())?;
    let reader = BufReader::new(file);
    let file_path = path.as_ref().to_str().unwrap().to_string();

    process_file_content(reader, file_path)
}

fn process_file_content<R: BufRead>(reader: R, path: String) -> Result<(Feature, Vec<Scenario>)> {
    let mut feature: Feature = Feature {
        id: Uuid::new_v4().to_string(),
        name: String::new(),
        description: String::new(),
        file_path: path.clone(),
        tags: Vec::new(),
    };

    let mut scenarios = Vec::new();
    let mut current_scenario: Option<Scenario> = None;

    let mut pending_tags = Vec::new();
    let mut example_lines_count: u32 = 0;

    let mut context = Context::None;

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.starts_with("@") {
            context = Context::None;

            pending_tags.extend(line.split_whitespace().map(String::from));
        } else if line.starts_with("Feature:") {
            context = Context::Feature;

            feature.name = line["Feature:".len()..].trim().to_string();
            feature.tags = pending_tags.clone();

            pending_tags.clear();
        } else if line.starts_with("Scenario:") || line.starts_with("Scenario Outline:") {
            // Stop processing if feature is malformed
            if context == Context::None
                && feature.name.is_empty()
                && feature.description.is_empty()
                && feature.tags.is_empty()
            {
                println!("Feature: {:?}, context: {:?}", feature.name, context);
                println!("Feature is malformed, skipping: {}", path);
                return Ok((feature, scenarios));
            }

            if let Some(mut scenario) = current_scenario.take() {
                match context {
                    Context::ExamplesBody => {
                        scenario.examples = example_lines_count;
                        example_lines_count = 0;
                    }
                    _ => {}
                }
                scenarios.push(scenario);
            }

            context = Context::Scenario;

            let scenario_name = line[line.find(":").unwrap() + 1..].trim().to_string();

            current_scenario = Some(Scenario {
                id: Uuid::new_v4().to_string(),
                name: scenario_name,
                description: String::new(),
                targets: BTreeSet::new(),
                steps: 0,
                examples: 0,
                tags: pending_tags.clone(),
                feature_id: feature.id.clone(),
                organization_name: String::new(),
            });

            pending_tags.clear();
        } else if line.starts_with("Examples:") {
            if pending_tags.len() > 0 {
                if let Some(scenario) = current_scenario.as_mut() {
                    scenario.tags.extend(pending_tags.clone());
                }
                pending_tags.clear();
            }

            context = Context::ExamplesHeader;
        } else if line.starts_with("|") {
            match context {
                Context::ExamplesBody => {
                    example_lines_count += 1;
                }
                Context::ExamplesHeader => {
                    context = Context::ExamplesBody;
                }
                _ => {}
            }
        } else if STEP_KEYWORDS
            .iter()
            .any(|&keyword| line.starts_with(keyword))
        {
            context = Context::ScenarioSteps;

            if let Some(scenario) = current_scenario.as_mut() {
                scenario.steps += 1;

                if let Some(organization_name) = capture_group(&ORGANIZATION_RE, &line) {
                    scenario.organization_name = organization_name.trim().to_string();
                } else if let Some(screen_name) = capture_group(&SCREEN_RE, &line) {
                    let name = screen_name.to_lowercase();
                    let date = capture_group(&DATE_RE, &line);
                    let flag = capture_group(&FLAGS_RE, &line);

                    scenario
                        .targets
                        .insert(get_target_string(&name, flag, date));
                }
            }
        } else {
            match context {
                Context::Feature => {
                    if feature.description.len() > 0 && !line.is_empty() {
                        feature.description.push_str(" ");
                    }
                    feature.description.push_str(&line);
                }
                Context::Scenario => {
                    if let Some(scenario) = current_scenario.as_mut() {
                        if scenario.description.len() > 0 && !line.is_empty() {
                            scenario.description.push_str(" ");
                        }
                        scenario.description.push_str(&line);
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(mut scenario) = current_scenario {
        match context {
            Context::ExamplesBody => {
                scenario.examples = example_lines_count;
            }
            _ => {}
        }
        scenarios.push(scenario);
    }

    Ok((feature, scenarios))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use uuid::Uuid;

    #[test]
    fn test_get_target_string_valid_name() {
        let target = get_target_string("budgets", Some("flag1"), Some("2023-10-01"));
        assert_eq!(target, "budgets;/financials/budgets;flag1;2023-10-01");

        let target = get_target_string("companies", None, Some("2023-10-03"));
        assert_eq!(target, "companies;/contacts/companies;;2023-10-03");

        let target = get_target_string("contacts", Some("flag3"), None);
        assert_eq!(target, "contacts;/contacts/people;flag3;");

        let target = get_target_string("dashboards", None, None);
        assert_eq!(target, "dashboards;/dashboards;;");
    }

    #[test]
    fn test_get_target_string_invalid_name() {
        let target = get_target_string("invalid", Some("flag1"), Some("2023-10-01"));
        assert_eq!(target, "invalid;;flag1;2023-10-01");

        let target = get_target_string("invalid2", None, Some("2023-10-03"));
        assert_eq!(target, "invalid2;;;2023-10-03");

        let target = get_target_string("invalid3", Some("flag3"), None);
        assert_eq!(target, "invalid3;;flag3;");

        let target = get_target_string("invalid4", None, None);
        assert_eq!(target, "invalid4;;;");
    }

    #[test]
    fn test_process_file_content() {
        let input = r#"
            @broken @slow @feature
            Feature: Feature name here
            This is a feature description
            broken into multiple lines
            or something like that

            @billing @bicker @annoy
            Scenario Outline: "<user>" accesses "<query>" with "<shortcut>" on "<route>"
                Given current organization is "Some Org LLC"
                And I do all the steps
                Then I should be in correct state

                @mobile
                Examples:
                | user        | shortcut     | query          | route                             |
                | Admin       | # Tasks      | Test ta        | /tasks/task/116046                |

                @web
                Examples:
                | user        | shortcut     | query          | route                             |
                | Admin       | ## Docs      | Project page   | /docs/doc/22981                   |
                | Admin       | $$ Budgets   | Test project   | /financials/budgets/d/deal/62326  |

            Scenario: Another scenario
                Given current organization is "Some Org LLC"
                And I do all the steps
                Then I should be in correct state

            Scenario: Another with description
                We are trying to do something here that will be awesome

                Given current organization is "Some Org LLC"
                And I do all the steps
                Then I should be in correct state
            "#;
        let reader = Cursor::new(input);
        let path = "some/file.feature".to_string();
        let (feature, scenarios) = process_file_content(reader, path).unwrap();

        assert!(Uuid::parse_str(&feature.id).is_ok());
        assert_eq!(feature.name, "Feature name here");
        assert_eq!(
            feature.description,
            "This is a feature description broken into multiple lines or something like that"
        );
        assert_eq!(feature.file_path, "some/file.feature");
        assert_eq!(feature.tags.len(), 3);
        assert_eq!(feature.tags[0], "@broken");
        assert_eq!(feature.tags[1], "@slow");
        assert_eq!(feature.tags[2], "@feature");

        assert_eq!(scenarios.len(), 3);
        assert!(Uuid::parse_str(&scenarios[0].id).is_ok());
        assert_eq!(
            scenarios[0].name,
            r#""<user>" accesses "<query>" with "<shortcut>" on "<route>""#
        );
        assert_eq!(scenarios[0].description, "");
        assert_eq!(scenarios[0].targets.len(), 0);
        assert_eq!(scenarios[0].steps, 3);
        assert_eq!(scenarios[0].examples, 3);
        assert_eq!(scenarios[0].tags.len(), 5);
        assert_eq!(scenarios[0].tags[0], "@billing");
        assert_eq!(scenarios[0].tags[1], "@bicker");
        assert_eq!(scenarios[0].tags[2], "@annoy");
        assert_eq!(scenarios[0].tags[3], "@mobile");
        assert_eq!(scenarios[0].tags[4], "@web");
        assert_eq!(scenarios[0].feature_id, feature.id);
        assert_eq!(scenarios[0].organization_name, "Some Org LLC");

        assert!(Uuid::parse_str(&scenarios[1].id).is_ok());
        assert_eq!(scenarios[1].name, "Another scenario");
        assert_eq!(scenarios[1].description, "");
        assert_eq!(scenarios[0].targets.len(), 0);
        assert_eq!(scenarios[1].steps, 3);
        assert_eq!(scenarios[1].examples, 0);
        assert_eq!(scenarios[1].tags.len(), 0);
        assert_eq!(scenarios[1].feature_id, feature.id);
        assert_eq!(scenarios[1].organization_name, "Some Org LLC");

        assert!(Uuid::parse_str(&scenarios[2].id).is_ok());
        assert_eq!(scenarios[2].name, "Another with description");
        assert_eq!(
            scenarios[2].description,
            "We are trying to do something here that will be awesome"
        );
        assert_eq!(scenarios[0].targets.len(), 0);
        assert_eq!(scenarios[2].steps, 3);
        assert_eq!(scenarios[2].examples, 0);
        assert_eq!(scenarios[2].tags.len(), 0);
        assert_eq!(scenarios[2].feature_id, feature.id);
        assert_eq!(scenarios[2].organization_name, "Some Org LLC");
    }

    #[test]
    fn test_process_file_content_for_screens() {
        let input = r#"
            Feature: A feature with screens

            Scenario: One good screen
                Given current organization is "Some Org 1 LLC"
                And user is on a "budgets" screen

            Scenario: Two good screens
                Given current organization is "Some Org 2 LLC"
                And user is on a "companies" screen and flags "flag1,flag2" are enabled
                And user is on a "contacts" screen on date "2024-10-01" and flag "flag3" is enabled

            Scenario: Missing screen hashmap
                Given current organization is "Some Org 3 LLC"
                And user is on a "invalid" screen

            Scenario Outline: Malformed screen
                Given current organization is <org>
                And user is on a <name> screen on date <date> and flags <flags> are enabled
            Examples:
            | org   | name        | date          | flags         |
            | org1  | invalid     | 2023-10-01    | flag1         |
            | org2  | companies   | 2023-10-02    | flag2         |
        "#;
        let reader = Cursor::new(input);
        let path = "some/file.feature".to_string();
        let (feature, scenarios) = process_file_content(reader, path).unwrap();

        assert!(Uuid::parse_str(&feature.id).is_ok());
        assert_eq!(feature.name, "A feature with screens");
        assert_eq!(feature.description, "");
        assert_eq!(feature.file_path, "some/file.feature");
        assert_eq!(feature.tags.len(), 0);

        assert_eq!(scenarios.len(), 4);
        assert!(Uuid::parse_str(&scenarios[0].id).is_ok());
        assert_eq!(scenarios[0].name, "One good screen");
        assert_eq!(scenarios[0].description, "");
        assert_eq!(scenarios[0].targets.len(), 1);
        assert_eq!(
            scenarios[0].targets.iter().nth(0).unwrap(),
            "budgets;/financials/budgets;;"
        );
        assert_eq!(scenarios[0].steps, 2);
        assert_eq!(scenarios[0].examples, 0);
        assert_eq!(scenarios[0].tags.len(), 0);
        assert_eq!(scenarios[0].feature_id, feature.id);
        assert_eq!(scenarios[0].organization_name, "Some Org 1 LLC");

        assert!(Uuid::parse_str(&scenarios[1].id).is_ok());
        assert_eq!(scenarios[1].name, "Two good screens");
        assert_eq!(scenarios[1].description, "");
        assert_eq!(scenarios[1].targets.len(), 2);
        assert_eq!(
            scenarios[1].targets.iter().nth(0).unwrap(),
            "companies;/contacts/companies;flag1,flag2;"
        );
        assert_eq!(
            scenarios[1].targets.iter().nth(1).unwrap(),
            "contacts;/contacts/people;flag3;2024-10-01"
        );
        assert_eq!(scenarios[1].steps, 3);
        assert_eq!(scenarios[1].examples, 0);
        assert_eq!(scenarios[1].tags.len(), 0);
        assert_eq!(scenarios[1].feature_id, feature.id);
        assert_eq!(scenarios[1].organization_name, "Some Org 2 LLC");

        assert!(Uuid::parse_str(&scenarios[2].id).is_ok());
        assert_eq!(scenarios[2].name, "Missing screen hashmap");
        assert_eq!(scenarios[2].description, "");
        assert_eq!(scenarios[2].targets.len(), 1);
        assert_eq!(scenarios[2].targets.iter().nth(0).unwrap(), "invalid;;;");
        assert_eq!(scenarios[2].steps, 2);
        assert_eq!(scenarios[2].examples, 0);
        assert_eq!(scenarios[2].tags.len(), 0);
        assert_eq!(scenarios[2].feature_id, feature.id);
        assert_eq!(scenarios[2].organization_name, "Some Org 3 LLC");

        assert!(Uuid::parse_str(&scenarios[3].id).is_ok());
        assert_eq!(scenarios[3].name, "Malformed screen");
        assert_eq!(scenarios[3].description, "");
        assert_eq!(scenarios[3].targets.len(), 0);
        assert_eq!(scenarios[3].steps, 2);
        assert_eq!(scenarios[3].examples, 2);
        assert_eq!(scenarios[3].tags.len(), 0);
        assert_eq!(scenarios[3].feature_id, feature.id);
        assert_eq!(scenarios[3].organization_name, "");
    }

    #[test]
    fn test_missing_feature_name() {
        let content = r#"
            @tag1 @tag2
            Feature:
                This is a sample feature description.

            Scenario: Sample Scenario
                Given current organization is "Sample Org"
                When something happens
                Then something should be true
        "#;
        let reader = Cursor::new(content);
        let path = "missing_feature_name.feature".to_string();
        let (feature, scenarios) = process_file_content(reader, path).unwrap();

        assert!(Uuid::parse_str(&feature.id).is_ok());
        assert_eq!(feature.name, "");
        assert_eq!(feature.description, "This is a sample feature description.");
        assert_eq!(feature.file_path, "missing_feature_name.feature");
        assert_eq!(feature.tags.len(), 2);
        assert_eq!(feature.tags[0], "@tag1");
        assert_eq!(feature.tags[1], "@tag2");
        assert_eq!(scenarios.len(), 1);
        assert_eq!(scenarios[0].name, "Sample Scenario");
        assert_eq!(scenarios[0].description, "");
        assert_eq!(scenarios[0].steps, 3);
        assert_eq!(scenarios[0].examples, 0);
        assert_eq!(scenarios[0].tags.len(), 0);
        assert_eq!(scenarios[0].feature_id, feature.id);
        assert_eq!(scenarios[0].organization_name, "Sample Org");
    }

    #[test]
    fn test_missing_scenario_name() {
        let content = r#"
            @tag1 @tag2
            Feature: Sample Feature
                This is a sample feature description.

            Scenario:
                Given current organization is "Sample Org"
                When something happens
                Then something should be true
        "#;
        let reader = std::io::Cursor::new(content);
        let path = "missing_scenario_name.feature".to_string();
        let (feature, scenarios) = process_file_content(reader, path).unwrap();

        assert!(Uuid::parse_str(&feature.id).is_ok());
        assert_eq!(feature.name, "Sample Feature");
        assert_eq!(feature.description, "This is a sample feature description.");
        assert_eq!(feature.file_path, "missing_scenario_name.feature");
        assert_eq!(feature.tags.len(), 2);
        assert_eq!(feature.tags[0], "@tag1");
        assert_eq!(feature.tags[1], "@tag2");
        assert_eq!(scenarios.len(), 1);
        assert_eq!(scenarios[0].name, "");
        assert_eq!(scenarios[0].description, "");
        assert_eq!(scenarios[0].steps, 3);
        assert_eq!(scenarios[0].examples, 0);
        assert_eq!(scenarios[0].tags.len(), 0);
        assert_eq!(scenarios[0].feature_id, feature.id);
        assert_eq!(scenarios[0].organization_name, "Sample Org");
    }

    #[test]
    fn test_missing_feature_keyword() {
        let content = r#"
            @tag1 @tag2
            Sample Feature
                This is a sample feature description.

            Scenario: Sample Scenario
                Given current organization is "Sample Org"
                When something happens
                Then something should be true
        "#;
        let reader = Cursor::new(content);
        let path = "missing_feature_keyword.feature".to_string();
        let (feature, scenarios) = process_file_content(reader, path).unwrap();

        assert!(Uuid::parse_str(&feature.id).is_ok());
        assert_eq!(feature.name, "");
        assert_eq!(feature.description, "");
        assert_eq!(feature.file_path, "missing_feature_keyword.feature");
        assert_eq!(feature.tags.len(), 0);
        assert_eq!(scenarios.len(), 0);
    }

    #[test]
    fn test_missing_scenario_keyword() {
        let content = r#"
            @tag1 @tag2
            Feature: Sample Feature
                This is a sample feature description.

            Sample Scenario
                Given current organization is "Sample Org"
                When something happens
                Then something should be true
        "#;
        let reader = Cursor::new(content);
        let path = "missing_scenario_keyword.feature".to_string();
        let (feature, scenarios) = process_file_content(reader, path).unwrap();

        assert!(Uuid::parse_str(&feature.id).is_ok());
        assert_eq!(feature.name, "Sample Feature");
        assert_eq!(
            feature.description,
            "This is a sample feature description. Sample Scenario"
        );
        assert_eq!(feature.file_path, "missing_scenario_keyword.feature");
        assert_eq!(feature.tags.len(), 2);
        assert_eq!(feature.tags[0], "@tag1");
        assert_eq!(feature.tags[1], "@tag2");
        assert_eq!(scenarios.len(), 0);
    }

    #[test]
    fn test_incorrect_feature_file() {
        let content = r#"
            Here is some random text that is not a feature file
            Here is some random text that is not a feature file
            Here is some random text that is not a feature file
            Here is some random text that is not a feature file
            Here is some random text that is not a feature file
            Here is some random text that is not a feature file
            Here is some random text that is not a feature file
            Here is some random text that is not a feature file
        "#;
        let reader = Cursor::new(content);
        let path = "incorrect.feature".to_string();
        let (feature, scenarios) = process_file_content(reader, path).unwrap();

        assert!(Uuid::parse_str(&feature.id).is_ok());
        assert_eq!(feature.name, "");
        assert_eq!(feature.description, "");
        assert_eq!(feature.file_path, "incorrect.feature");
        assert_eq!(feature.tags.len(), 0);
        assert_eq!(scenarios.len(), 0);
    }
}
