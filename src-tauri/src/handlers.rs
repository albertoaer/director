use crate::{state, fsop::{*, self}};

#[tauri::command]
pub fn request_directory(
  window: tauri::Window,
  state: tauri::State<state::RouteNotifier>,
  fs_manager: tauri::State<fsop::FSManager>,
  directory: String
) -> Result<Route, String> {
  let route: Route = directory.try_into()?;
  route.ensure_exists().map_err(|err| err.to_string())?;
  
  state.set(window, route.path().clone());
  fs_manager.publish_entry(route.path()).map_err(|err| err.to_string())?;
  
  Ok(route)
}

#[tauri::command]
pub fn request_calculate_directory(
  window: tauri::Window,
  state: tauri::State<state::RouteNotifier>,
  fs_manager: tauri::State<fsop::FSManager>,
  directory: String
) -> Result<Route, String> {
  let route: Route = directory.try_into()?;
  route.ensure_exists().map_err(|err| err.to_string())?;

  state.set(window, route.path().clone());
  fs_manager.publish_entry(route.path()).map_err(|err| err.to_string())?;

  fs_manager.process_order(FSOrder::new(route.path())).map_err(|err| err.to_string())?;

  Ok(route)
}

#[tauri::command]
pub fn request_alerts(
  state: tauri::State<state::AlertNotifier>,
) -> Vec<Alert> {
  state.alerts()
}

#[tauri::command]
pub fn save_alerts(
  state: tauri::State<state::AlertNotifier>,
  alerts: Vec<Alert>
) -> bool {
  state.set_alerts(alerts)
}

#[tauri::command]
pub fn get_detections(
  state: tauri::State<state::AlertNotifier>,
  begin: u32,
  count: u32
) -> Vec<Detection> {
  state.get_detections(begin as usize, count as usize)
}

#[tauri::command]
pub fn refresh_orders(
  state: tauri::State<fsop::FSManager>
) -> Vec<fsop::FSOrder> {
  state.orders()
}

#[tauri::command]
pub fn add_startup(
  state: tauri::State<state::Startup>,
  directory: String
) {
  state.add(directory)
}

#[tauri::command]
pub fn remove_startup(
  state: tauri::State<state::Startup>,
  directory: String
) {
  state.remove(directory)
}

#[tauri::command]
pub fn get_startup(
  state: tauri::State<state::Startup>
) -> Vec<String> {
  state.get()
}