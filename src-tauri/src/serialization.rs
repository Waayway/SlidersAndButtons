use std::{
    fs,
    path::{Path, PathBuf},
    vec,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

fn get_config_path() -> Result<PathBuf, ()> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "waayway", "sliders-and-buttons") {
        return Ok(proj_dirs.config_dir().join("config.json").to_path_buf());
    }
    return Err(());
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GridItemData {
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GridItem {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    #[serde(rename = "type")]
    pub item_type: String,
    pub data: GridItemData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub serial_port: String,
    pub grid_items: Vec<GridItem>,
}

impl Data {
    pub fn load_data() -> Self {
        let DEFAULT_PATH = match get_config_path() {
            Ok(path) => path,
            Err(_) => "".into(),
        };

        let file_contents = fs::read_to_string(DEFAULT_PATH);
        let content = match file_contents {
            Ok(content) => content,
            Err(_e) => String::new(),
        };
        let deserialized: Self = serde_json::from_str(content.as_str()).unwrap();
        deserialized
    }

    pub fn create_if_file_not_exist() -> Result<(), ()> {
        let DEFAULT_PATH = match get_config_path() {
            Ok(path) => path,
            Err(_) => "".into(),
        };
        let file = DEFAULT_PATH.exists();
        if file {
            return Ok(());
        };
        let prefix = DEFAULT_PATH.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        let data = Self {
            serial_port: "".to_string(),
            grid_items: vec![],
        };
        let serialized = serde_json::to_string(&data).unwrap();
        let tmp = fs::write(DEFAULT_PATH, serialized).unwrap();
        Ok(())
    }

    pub fn save_data(&self) -> Result<(), ()> {
        let DEFAULT_PATH = match get_config_path() {
            Ok(path) => path,
            Err(_) => "".into(),
        };

        let serialized = serde_json::to_string(&self).unwrap();
        println!("Saving config: {}", serialized);

        fs::write(DEFAULT_PATH, serialized).unwrap();
        Ok(())
    }
}
