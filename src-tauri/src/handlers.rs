use crate::{state, fsop::*, alerts};

#[tauri::command]
pub fn request_directory(
  window: tauri::Window,
  state: tauri::State<state::AppState>,
  directory: String
) -> Result<Route, String> {
  let route: Route = directory.try_into()?;
  route.ensure_exists().map_err(|err| err.to_string())?;
  
  let manager = state.inner().fs_manager();
  state.route_notifier().set(window, route.path().clone());
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
  state.route_notifier().set(window, route.path().clone());
  manager.publish_entry(route.path()).map_err(|err| err.to_string())?;

  manager.process_order(FSOrder::new(route.path())).map_err(|err| err.to_string())?;

  Ok(route)
}

#[tauri::command]
pub fn request_alerts(
  state: tauri::State<state::AppState>,
) -> Vec<alerts::Alert> {
  state.alert_notifier().alerts()
}

#[tauri::command]
pub fn save_alerts(
  state: tauri::State<state::AppState>,
  alerts: Vec<alerts::Alert>
) {
  state.alert_notifier().set_alerts(alerts);
}