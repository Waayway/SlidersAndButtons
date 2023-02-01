use std::sync::Mutex;

use crate::serialization::Data;

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
pub fn save_config(state: tauri::State<DataState>) {
    state.0.lock().unwrap().save_data().unwrap();
}
