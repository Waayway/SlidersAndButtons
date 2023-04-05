#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use audio::AudioState;
use background::{BackgroundProcess, BackgroundState};
use commands::DataState;
use serialization::Data;
use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

mod background;
mod commands;
mod serialization;
mod audio;

fn build_tray() -> SystemTrayMenu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
}

fn on_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {}
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {}
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {}

        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main");
                    if window.is_some() {
                        if window.as_ref().unwrap().is_visible().unwrap() {
                            window.unwrap().hide().unwrap();
                        } else {
                            window.unwrap().show().unwrap();
                        }
                        item_handle.set_title("Show").unwrap();
                    } else {
                        eprintln!("ERROR: Application not active somehow...");
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}


fn main() {
    Data::create_if_file_not_exist().unwrap();

    let tray = SystemTray::new().with_menu(build_tray());

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(on_tray_event)
        .manage(DataState(Data::load_data().into()))
        .manage(BackgroundState(BackgroundProcess::new().into()))
        .invoke_handler(tauri::generate_handler![
            commands::get_serial_ports,
            commands::save_serial_port,
            commands::save_config,
            commands::save_grid_config,
            commands::get_grid_config,
            commands::get_possible_keys,
            commands::get_serial_config,
            commands::get_audio_sessions
        ])
        .setup(|app| {
            let bg_state: tauri::State<BackgroundState> = app.state();
            let data_state: tauri::State<DataState> = app.state();
            bg_state.0.lock().unwrap().spawn_thread();
            data_state.0.lock().unwrap().save_data(bg_state).ok();
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
                app.tray_handle()
                    .get_item("hide")
                    .set_title("Show")
                    .unwrap();
            }
            _ => {}
        });
}
