#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serialization::Data;  

mod serialization;
mod commands;

fn main() {
    Data::create_if_file_not_exist();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::get_serial_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
