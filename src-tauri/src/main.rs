// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use persistency::PersistencyFile;
use tauri::{SystemTray, SystemTrayEvent, Manager, SystemTrayMenu, CustomMenuItem};
use window_shadows::set_shadow;
use env_logger;

mod persistency;
mod handlers;
mod state;
mod fsop;
mod ds;

fn main() {
  env_logger::init();

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
    .invoke_handler(tauri::generate_handler![
      handlers::request_directory,
      handlers::request_calculate_directory,
      handlers::request_alerts,
      handlers::save_alerts,
      handlers::get_detections,
      handlers::refresh_orders,
      handlers::add_startup,
      handlers::remove_startup,
      handlers::get_startup,
      handlers::run_startup
    ])
    .setup(|app| {
      #[cfg(any(windows, target_os = "macos"))]
      {
        let window = app.get_window("main").unwrap();
        set_shadow(&window, true).unwrap();
      }
      let alert_notifier = state::AlertNotifier::new(Some(PersistencyFile::new_config("alerts")), app.app_handle());
      app.manage(alert_notifier.clone());

      let route_notifier = state::RouteNotifier::new(app.app_handle());
      app.manage(route_notifier.clone());

      let fs_manager = fsop::FSManager::new();
      fs_manager.listenners().subscribe(alert_notifier);
      fs_manager.listenners().subscribe(route_notifier);

      let startup = state::Startup::new(
        Some(PersistencyFile::new_config("startup")), fs_manager.clone(), app.app_handle()
      );
      app.manage(startup);

      app.manage(fs_manager);
      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|_app, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => { api.prevent_exit(); },
      _ => {}
    })
}
