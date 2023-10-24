// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{SystemTray, SystemTrayEvent, Manager, SystemTrayMenu, CustomMenuItem};
use window_shadows::set_shadow;

mod handlers;
mod alerts;
mod state;
mod fsop;
mod ds;

fn main() {
  let menu = SystemTrayMenu::new().add_item(CustomMenuItem::new("exit", "Exit"));

  tauri::Builder::default()
    .system_tray(SystemTray::new().with_menu(menu))
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { .. } => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      },
      SystemTrayEvent::MenuItemClick { id, .. } if id == "exit" => {
        std::process::exit(0);
      }
      _ => {}
    })
    .on_window_event(|event| match event.event() {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        event.window().hide().unwrap();
        api.prevent_close();
      }
      _ => {}
    })
    .manage(state::AppState::new())
    .invoke_handler(tauri::generate_handler![
      handlers::request_directory,
      handlers::request_calculate_directory,
      handlers::save_alerts
    ])
    .setup(|app| {
      #[cfg(any(windows, target_os = "macos"))]
      {
        let window = app.get_window("main").unwrap();
        set_shadow(&window, true).unwrap();
      }
      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|_app, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => { api.prevent_exit(); },
      _ => {}
    })
}
