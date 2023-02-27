mod calculator;
mod icons;
mod preferences;
mod search;

extern crate directories;
extern crate plist;

use auto_launch::AutoLaunchBuilder;
use calculator::calculate;
use directories::ProjectDirs;
use tauri::State;
use std::{process::Command, time::Instant};

pub use icons::convert_all_app_icons_to_png;
pub use preferences::create_preferences_if_missing;
pub use search::{search, similarity_sort};

use crate::app::AppState;

#[derive(Debug, serde::Serialize)]
pub enum ResultType {
    Applications = 1,
    Files = 2,
    Calculation = 3,
    Extensions = 4,
}

#[derive(Debug, serde::Serialize)]
pub struct InputResult {
    pub r#type: ResultType,
    pub title: Option<String>,
    pub value: String,
}

#[tauri::command]
pub fn handle_input(input: &str, state: State<'_, AppState>) -> (Vec<InputResult>, f32) {
    let mut result: Vec<InputResult>;
    let start_time = Instant::now();
    if !input.starts_with('/') {
        let mut apps_result = search(
            input,
            vec![
                "/Applications",
                "/System/Applications",
                "/System/Applications/Utilities",
            ],
            Some(".app"),
            Some(1),
        );
        similarity_sort(&mut apps_result, input);
        let mut apps_result: Vec<InputResult> = apps_result
            .iter()
            .map(|it| InputResult {
                r#type: ResultType::Applications,
                title: None,
                value: it.to_string(),
            })
            .collect();
        let state = state.0.lock().unwrap();
        let mut exts_result: Vec<InputResult> = state
            .extensions
            .iter()
            .filter(|it| {
                it.title.contains(input)
            })
            .map(|it| InputResult {
                r#type: ResultType::Extensions,
                title: Some(it.title.clone()),
                value: it.main.clone(),
            })
            .collect();
        result = Vec::new();
        result.append(&mut apps_result);
        result.append(&mut exts_result);
    } else {
        result = search(
            input.trim_start_matches('/'),
            vec!["/Users/"],
            None,
            Some(10000),
        )
            .iter()
            .map(|it| {
                InputResult {
                    r#type: ResultType::Files,
                title: None,
                    value: it.to_string(),
                }
            })
            .collect();
    }
    if result.is_empty() {
        let calculation_result = calculate(input);
        if !calculation_result.is_empty() {
            result.push(InputResult {
                r#type: ResultType::Calculation,
                title: None,
                value: calculation_result,
            });
        }
    }
    let time_taken = start_time.elapsed().as_secs_f32();
    (result, time_taken)
}

#[tauri::command]
pub fn get_icon(app_name: &str) -> String {
    if let Some(proj_dirs) = ProjectDirs::from("com", "parth jadhav", "verve") {
        let icon_dir = proj_dirs.config_dir().join("appIcons");
        let icon_path = icon_dir.join(app_name.to_owned() + &".png");
        if icon_path.exists() {
            return icon_path.to_str().unwrap().to_owned();
        }
        return String::from("");
    }
    return String::from("");
}

#[tauri::command]
pub fn open_command(path: &str) {
    Command::new("open")
        .arg(path.trim())
        .spawn()
        .expect("failed to execute process");
}

#[tauri::command]
pub fn launch_on_login(enable: bool) -> bool {
    let auto = AutoLaunchBuilder::new()
        .set_app_name("verve")
        .set_app_path("/Applications/verve.app")
        .build()
        .unwrap();

    if enable {
        match auto.enable() {
            Ok(_) => return true,
            Err(_) => {
                println!("Failed");
                false
            }
        }
    } else {
        match auto.disable() {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
