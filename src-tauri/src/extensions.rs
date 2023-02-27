use std::fs::{read_dir, read_to_string};

use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Extension {
    pub title: String,
    pub main: String,
    pub icon: Option<String>,
}

pub fn get_extensions() -> Vec<Extension> {
    let mut result: Vec<Extension> = Vec::new();

    if let Some(proj_dirs) = ProjectDirs::from("com", "parth jadhav", "verve") {
        let data_dir = proj_dirs.data_local_dir();
        let extenions_dir = data_dir.join("extensions");

        if !extenions_dir.is_dir() {
            return result;
        }

        for entry in read_dir(extenions_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            for entry in read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if !path.is_file() || !path.ends_with("info.json") {
                    continue;
                }

                if let Ok(package_json) = read_to_string(&path) {
                    if let Ok(mut ext) = serde_json::from_str::<Extension>(&package_json) {
                        let mut file_path = path.parent().unwrap().to_path_buf();
                        file_path.push(&ext.main);
                        ext.main = file_path.as_path().to_str().unwrap().to_string();
                        result.push(ext);
                    }
                }
            }
        }
    }

    result
}
