use std::{fs, path::PathBuf, vec};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::background::{BackgroundProcess, BackgroundState};

fn get_config_path() -> Result<PathBuf, ()> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "waayway", "sliders-and-buttons") {
        return Ok(proj_dirs.config_dir().join("config.json").to_path_buf());
    }
    return Err(());
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyComboMod {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    #[serde(rename = "super")]
    pub meta: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyCombo {
    pub key: Option<String>,
    pub modifiers: KeyComboMod,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GridItemData {
    pub key: Option<String>,
    pub keyCombo: Option<KeyCombo>
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
        fs::write(DEFAULT_PATH, serialized).unwrap();
        Ok(())
    }

    pub fn save_data(&self, state: tauri::State<BackgroundState>) -> Result<(), ()> {
        let DEFAULT_PATH = match get_config_path() {
            Ok(path) => path,
            Err(_) => "".into(),
        };

        state
            .0
            .lock()
            .unwrap()
            .update_config(self.to_owned());

        let serialized = serde_json::to_string(&self).unwrap();
        println!("Saving config: {}", serialized);

        fs::write(DEFAULT_PATH, serialized).unwrap();

        Ok(())
    }
}
