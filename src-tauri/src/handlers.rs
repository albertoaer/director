use crate::{state, fsop::*};

use std::{path, env};

#[tauri::command]
pub fn request_directory(
  window: tauri::Window,
  state: tauri::State<state::AppState>,
  directory: String
) -> Result<Route, String> {
  let route: Route = directory.try_into()?;

  let manager = state.inner().fs_manager();
  state.associate_folder(window, route.path().clone());
  manager.get_entry(route.path()).map_err(|err| err.to_string())?;

  Ok(route)
}

#[tauri::command]
pub fn request_calculate_directory(
  window: tauri::Window,
  state: tauri::State<state::AppState>,
  directory: String
) -> Result<Route, String> {
  let route: Route = directory.try_into()?;

  let manager = state.inner().fs_manager();
  state.associate_folder(window, route.path().clone());
  manager.get_entry(route.path()).map_err(|err| err.to_string())?;
  manager.calculate_entry(route.path()).map_err(|err| err.to_string())?;

  Ok(route)
}

#[tauri::command]
pub fn save_alerts(
  state: tauri::State<state::AppState>,
  alerts: Vec<String>
) {

}