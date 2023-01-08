use std::fs;

use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
static DEFAULT_PATH: &str = "$XDG_CONFIG_HOME/sliders_and_buttons/settings.json";
#[cfg(target_os = "linux")]
static DEFAULT_PATH: &str = "$XDG_CONFIG_HOME/sliders_and_buttons/settings.json";
#[cfg(target_os = "windows")]
static DEFAULT_PATH: &str = "%appdata%/sliders_and_buttons/settings.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub serial_port: String,
}

impl Data {
    pub fn load_data() -> Self {
        let file_contents = fs::read_to_string(DEFAULT_PATH);
        let content = match file_contents {
            Ok(content) => content,
            Err(_e) => String::new(),
        };
        let deserialized: Self = serde_json::from_str(content.as_str()).unwrap();
        deserialized
    }

    pub fn create_if_file_not_exist() -> Result<(), ()> {
        let file = std::path::Path::new(DEFAULT_PATH).exists();
        if file {return Ok(())};
        let data = Self {serial_port: "".to_string()};
        let serialized = serde_json::to_string(&data).unwrap();
        let tmp = fs::write(DEFAULT_PATH, serialized).unwrap_err();
        println!("{} {}",tmp, DEFAULT_PATH);
        Ok(())
    }

    pub fn save_data(&self) -> Result<(), ()> {
        let serialized = serde_json::to_string(&self).unwrap();
        fs::write(DEFAULT_PATH, serialized).unwrap_err();
        Ok(())
    }
}
