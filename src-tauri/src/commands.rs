use std::{sync::Mutex};


use crate::{serialization::{Data, GridItem}, background::BackgroundState};

pub struct DataState(pub Mutex<Data>);

#[tauri::command]
pub fn get_serial_ports() -> Vec<String> {
    let ports = serialport::available_ports().expect("No ports found!");
    let mut converted_ports: Vec<String> = vec![];
    for p in ports {
        converted_ports.push(p.port_name);
    }
    return converted_ports;
}

#[tauri::command]
pub fn save_serial_port(serialport: String, state: tauri::State<DataState>) {
    state.0.lock().unwrap().serial_port = serialport;
}

#[tauri::command]
pub fn get_grid_config(state: tauri::State<DataState>) -> Vec<GridItem> {
    return state.0.lock().unwrap().grid_items.clone();
}

#[tauri::command]
pub fn get_possible_keys() -> Vec<String> {
    let mut keys: Vec<String> = vec![
        "Escape".to_string(),
        "MediaNextTrack".to_string(),
        "MediaPreviousTrack".to_string(),
        "MediaStop".to_string(),
        "MediaPlayPause".to_string()
    ];
    for i in 1..25 {
        keys.append(&mut vec![format!("F{}", i)]);
    }
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    return keys;
}

#[tauri::command]
pub fn save_grid_config(grid_items: Vec<GridItem>, state: tauri::State<DataState>) {
    // * Debug code
    // for item in grid_items {
    //     println!(
    //         "x:{} y:{} width:{} height:{} item_type:{}",
    //         item.x, item.y, item.width, item.height, item.item_type
    //     );
    // }
    state.0.lock().unwrap().grid_items = grid_items;
}

#[tauri::command]
pub fn save_config(state: tauri::State<DataState>, background_state: tauri::State<BackgroundState>) {
    state.0.lock().unwrap().save_data(background_state).unwrap();
}

#[tauri::command]
pub fn get_serial_config(state: tauri::State<DataState>) -> String {
    state.0.lock().unwrap().serial_port.clone()
}