mod clone_detector;
mod cucumber_runner;
mod e2e_locator;
mod features_reader;
mod file_reader;
mod organizations_reader;
mod steps_reader;

use serde_json::json;

#[tauri::command(async)]
async fn get_features(base_path: String) -> serde_json::Value {
    let result = features_reader::get_all_features(base_path).await;
    match result {
        Ok((features, scenarios)) => json!({ "features": features, "scenarios": scenarios }),
        Err(e) => json!({ "err": e.to_string() }),
    }
}

#[tauri::command]
fn get_organizations(file_path: &str) -> serde_json::Value {
    let orgs = organizations_reader::parse(file_path);
    match orgs {
        Ok((people, teams, organizations)) => {
            json!({ "people": people, "teams": teams, "organizations": organizations })
        }
        Err(e) => json!({ "err": e }),
    }
}

#[tauri::command(async)]
async fn find_e2e_repo() -> serde_json::Value {
    let entry_dir: Option<&str> = None;

    match e2e_locator::locate_e2e_repo(entry_dir).await {
        Ok(results) => json!({ "ok": results }),
        Err(e) => json!({ "err": e }),
    }
}

#[tauri::command(async)]
async fn validate_e2e_repo(path: String) -> bool {
    e2e_locator::is_target_valid(&path).await
}

#[tauri::command]
fn get_file_contents(file_path: &str, scenario_name: Option<&str>) -> serde_json::Value {
    match file_reader::read_file_contents(file_path, scenario_name) {
        Ok(contents) => json!({ "ok": contents }),
        Err(e) => json!({ "err": e }),
    }
}

#[tauri::command]
fn run_e2e(
    folder_path: &str,
    feature_file: &str,
    scenario_name: Option<&str>,
) -> serde_json::Value {
    match cucumber_runner::run_cucumber(folder_path, feature_file, scenario_name) {
        Ok(output) => json!({ "ok": output }),
        Err(e) => json!({ "err": e }),
    }
}

#[tauri::command]
fn detect_organization_clones(organizations: Vec<(String, String)>) -> serde_json::Value {
    let clone_groups = clone_detector::detect_clones(&organizations);
    json!({ "ok": clone_groups })
}

#[tauri::command]
fn get_steps(base_path: String) -> serde_json::Value {
    match steps_reader::parse_step_definitions(&base_path) {
        Ok(steps) => json!({ "ok": steps }),
        Err(e) => json!({ "err": e }),
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            detect_organization_clones,
            find_e2e_repo,
            get_features,
            get_file_contents,
            get_organizations,
            get_steps,
            run_e2e,
            validate_e2e_repo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
