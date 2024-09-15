use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;

pub type People = Vec<Person>;
pub type Teams = Vec<Team>;
pub type Organizations = Vec<Organization>;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Person {
    name: String,
    avatar: String,
    teams: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct Team {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Organization {
    id: String,
    name: String,
    blame: String,
    team: String,
    users: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Org {
    id: String,
    blame: String,
    team: String,
    users: HashMap<String, String>,
}
type Avatars = HashMap<String, String>;
type Orgs = HashMap<String, Org>;

pub fn parse(folder_path: &str) -> Result<(People, Teams, Organizations), String> {
    let org_file_path = Path::new(folder_path).join("organizations.yml");
    let avatars_file_path = Path::new(folder_path).join("avatars.yml");

    let org_file =
        File::open(&org_file_path).map_err(|_| "organization file not found".to_string())?;
    let orgs: Orgs =
        from_reader(org_file).map_err(|_| "organization file invalid YAML".to_string())?;

    let avatars: Avatars = if let Ok(avatars_file) = File::open(&avatars_file_path) {
        from_reader(avatars_file).map_err(|_| "avatars file invalid YAML".to_string())?
    } else {
        HashMap::new()
    };

    let mut people_map = HashMap::new();
    let mut teams_set = HashSet::new();
    let mut organizations = Organizations::new();

    for (org_name, org) in orgs {
        people_map
            .entry(org.blame.clone())
            .or_insert_with(HashSet::new)
            .insert(org.team.clone());

        teams_set.insert(Team {
            name: org.team.clone(),
        });

        organizations.push(Organization {
            id: org.id.clone(),
            name: org_name.clone(),
            blame: org.blame.clone(),
            team: org.team.clone(),
            users: org.users.clone(),
        });
    }

    let people = people_map
        .into_iter()
        .map(|(name, teams)| Person {
            name: name.clone(),
            avatar: avatars.get(&name).cloned().unwrap_or_default(),
            teams,
        })
        .collect();

    let teams = teams_set.into_iter().collect();

    Ok((people, teams, organizations))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_success() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let org_file_path = dir.path().join("organizations.yml");
        let avatars_file_path = dir.path().join("avatars.yml");

        // Write mock YAML data to the organizations file
        let mut org_file = File::create(&org_file_path).unwrap();
        writeln!(
            org_file,
            r#"
            org1:
              id: "1"
              blame: "blame1"
              team: "team1"
              users:
                user1: "email1"
                user2: "email2"
            org2:
              id: "2"
              blame: "blame2"
              team: "team2"
              users:
                user3: "email3"
                user4: "email4"
            org3:
              id: "3"
              blame: "blame1"
              team: "team3"
              users:
                user5: "email5"
                user6: "email6"
            "#
        )
        .unwrap();

        // Write mock YAML data to the avatars file
        let mut avatars_file = File::create(&avatars_file_path).unwrap();
        writeln!(
            avatars_file,
            r#"
            blame1: "blame_link1"
            blame2: "blame_link2"
            "#
        )
        .unwrap();

        // Call the function and check the result
        let result = parse(dir.path().to_str().unwrap());
        assert!(result.is_ok());

        let (people, teams, organizations) = result.unwrap();
        assert_eq!(people.len(), 2);
        assert!(people.iter().any(|p| p.name == "blame1"
            && p.avatar == "blame_link1"
            && p.teams.len() == 2
            && p.teams.contains("team1")
            && p.teams.contains("team3")));
        assert!(people.iter().any(|p| p.name == "blame2"
            && p.avatar == "blame_link2"
            && p.teams.len() == 1
            && p.teams.contains("team2")));

        assert_eq!(teams.len(), 3);
        assert!(teams.iter().any(|t| t.name == "team1"));
        assert!(teams.iter().any(|t| t.name == "team2"));
        assert!(teams.iter().any(|t| t.name == "team3"));

        assert_eq!(organizations.len(), 3);
        assert!(organizations.iter().any(|o| o.id == "1"
            && o.name == "org1"
            && o.blame == "blame1"
            && o.team == "team1"
            && o.users["user1"] == "email1"
            && o.users["user2"] == "email2"));
        assert!(organizations.iter().any(|o| o.id == "2"
            && o.name == "org2"
            && o.blame == "blame2"
            && o.team == "team2"
            && o.users["user3"] == "email3"
            && o.users["user4"] == "email4"));
        assert!(organizations.iter().any(|o| o.id == "3"
            && o.name == "org3"
            && o.blame == "blame1"
            && o.team == "team3"
            && o.users["user5"] == "email5"
            && o.users["user6"] == "email6"));
    }

    #[test]
    fn test_parse_org_file_not_found() {
        let dir = tempdir().unwrap();
        let avatars_file_path = dir.path().join("avatars.yml");

        // Write mock YAML data to the avatars file
        let mut avatars_file = File::create(&avatars_file_path).unwrap();
        writeln!(
            avatars_file,
            r#"
            blame1: "blame_link1"
            blame2: "blame_link2"
            "#
        )
        .unwrap();

        let result = parse(dir.path().to_str().unwrap());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "organization file not found");
    }

    #[test]
    fn test_parse_avatars_file_not_found() {
        let dir = tempdir().unwrap();
        let org_file_path = dir.path().join("organizations.yml");

        // Write mock YAML data to the organizations file
        let mut org_file = File::create(&org_file_path).unwrap();
        writeln!(
            org_file,
            r#"
            org1:
              id: "1"
              blame: "blame1"
              team: "team1"
              users:
                user1: "email1"
                user2: "email2"
            "#
        )
        .unwrap();

        // Call the function and check the result
        let result = parse(dir.path().to_str().unwrap());
        assert!(result.is_ok());

        let (people, teams, organizations) = result.unwrap();
        assert_eq!(people.len(), 1);
        assert!(people.iter().any(|p| p.name == "blame1" && p.avatar == ""));

        assert_eq!(teams.len(), 1);
        assert!(teams.iter().any(|t| t.name == "team1"));

        assert_eq!(organizations.len(), 1);
        assert!(organizations.iter().any(|o| o.id == "1"
            && o.name == "org1"
            && o.blame == "blame1"
            && o.team == "team1"
            && o.users["user1"] == "email1"
            && o.users["user2"] == "email2"));
    }
}
