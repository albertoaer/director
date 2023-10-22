use crate::{state, fsop::*};

#[tauri::command]
pub fn request_directory(
  window: tauri::Window,
  state: tauri::State<state::AppState>,
  directory: String
) -> Result<Route, String> {
  let route: Route = directory.try_into()?;
  route.ensure_exists().map_err(|err| err.to_string())?;
  
  let manager = state.inner().fs_manager();
  state.associate_folder(window, route.path().clone());
  manager.publish_entry(route.path()).map_err(|err| err.to_string())?;
  
  Ok(route)
}

#[tauri::command]
pub fn request_calculate_directory(
  window: tauri::Window,
  state: tauri::State<state::AppState>,
  directory: String
) -> Result<Route, String> {
  let route: Route = directory.try_into()?;
  route.ensure_exists().map_err(|err| err.to_string())?;

  let manager = state.inner().fs_manager();
  state.associate_folder(window, route.path().clone());
  manager.publish_entry(route.path()).map_err(|err| err.to_string())?;

  manager.process_order(FSOrder::new(route.path())).map_err(|err| err.to_string())?;

  Ok(route)
}

#[tauri::command]
pub fn save_alerts(
  state: tauri::State<state::AppState>,
  alerts: Vec<String>
) {

}