use std::{string, sync::Mutex};

use serde::Deserialize;

use crate::serialization::{Data, GridItem};

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
pub fn save_config(state: tauri::State<DataState>) {
    state.0.lock().unwrap().save_data().unwrap();
}
