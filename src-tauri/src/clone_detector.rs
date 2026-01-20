use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone)]
pub struct CloneGroup {
    pub template_name: String,
    pub template_id: Option<String>,
    pub clones: Vec<CloneInfo>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CloneInfo {
    pub org_id: String,
    pub org_name: String,
    pub clone_id: String,
}

/// Detects organization clones based on naming patterns.
/// Clone pattern: `{Template Name} - TEMP-{clone_id}`
/// Example: "Budgeting Service LLC - TEMP-1206400"
pub fn detect_clones(
    organizations: &[(String, String)], // Vec of (id, name)
) -> Vec<CloneGroup> {
    let clone_pattern = Regex::new(r"^(.+?) - TEMP-(\d+)$").unwrap();

    let mut template_to_clones: HashMap<String, Vec<CloneInfo>> = HashMap::new();
    let mut template_ids: HashMap<String, String> = HashMap::new();

    // Build a map of org names to their IDs for quick lookup
    let name_to_id: HashMap<&str, &str> = organizations
        .iter()
        .map(|(id, name)| (name.as_str(), id.as_str()))
        .collect();

    for (org_id, org_name) in organizations {
        if let Some(captures) = clone_pattern.captures(org_name) {
            let template_name = captures.get(1).unwrap().as_str().to_string();
            let clone_id = captures.get(2).unwrap().as_str().to_string();

            template_to_clones
                .entry(template_name.clone())
                .or_default()
                .push(CloneInfo {
                    org_id: org_id.clone(),
                    org_name: org_name.clone(),
                    clone_id,
                });

            // Check if the template org exists and store its ID
            if let Some(&template_id) = name_to_id.get(template_name.as_str()) {
                template_ids.insert(template_name, template_id.to_string());
            }
        }
    }

    // Convert to CloneGroup vec
    template_to_clones
        .into_iter()
        .map(|(template_name, clones)| CloneGroup {
            template_id: template_ids.get(&template_name).cloned(),
            template_name,
            clones,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_clones_basic() {
        let orgs = vec![
            ("1".to_string(), "Template Org".to_string()),
            ("2".to_string(), "Template Org - TEMP-100".to_string()),
            ("3".to_string(), "Template Org - TEMP-200".to_string()),
            ("4".to_string(), "Another Org".to_string()),
        ];

        let groups = detect_clones(&orgs);

        assert_eq!(groups.len(), 1);

        let group = &groups[0];
        assert_eq!(group.template_name, "Template Org");
        assert_eq!(group.template_id, Some("1".to_string()));
        assert_eq!(group.clones.len(), 2);

        let clone_ids: Vec<&str> = group.clones.iter().map(|c| c.clone_id.as_str()).collect();
        assert!(clone_ids.contains(&"100"));
        assert!(clone_ids.contains(&"200"));
    }

    #[test]
    fn test_detect_clones_no_template() {
        let orgs = vec![
            ("1".to_string(), "Orphan Clone - TEMP-100".to_string()),
            ("2".to_string(), "Orphan Clone - TEMP-200".to_string()),
        ];

        let groups = detect_clones(&orgs);

        assert_eq!(groups.len(), 1);

        let group = &groups[0];
        assert_eq!(group.template_name, "Orphan Clone");
        assert_eq!(group.template_id, None); // No template exists
        assert_eq!(group.clones.len(), 2);
    }

    #[test]
    fn test_detect_clones_no_clones() {
        let orgs = vec![
            ("1".to_string(), "Regular Org".to_string()),
            ("2".to_string(), "Another Org".to_string()),
        ];

        let groups = detect_clones(&orgs);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_detect_clones_complex_names() {
        let orgs = vec![
            (
                "1".to_string(),
                "Budgeting Service Time Approval LLC".to_string(),
            ),
            (
                "2".to_string(),
                "Budgeting Service Time Approval LLC - TEMP-1206400".to_string(),
            ),
        ];

        let groups = detect_clones(&orgs);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].template_name, "Budgeting Service Time Approval LLC");
        assert_eq!(groups[0].clones[0].clone_id, "1206400");
    }
}
