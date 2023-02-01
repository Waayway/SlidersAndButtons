#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::DataState;
use serialization::Data;  

mod serialization;
mod commands;

fn main() {
    Data::create_if_file_not_exist().unwrap();
    tauri::Builder::default()
        .manage(DataState(Data::load_data().into()))
        .invoke_handler(tauri::generate_handler![commands::get_serial_ports, commands::save_serial_port ,commands::save_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
