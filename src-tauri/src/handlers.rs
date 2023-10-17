use crate::state;

use std::{path, env, ffi};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouteItem {
  name: String,
  path: String
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Route {
  items: Vec<RouteItem>,
  path: String,
  prefixed: bool
}

fn obtain_name(path: &path::Path) -> Option<&ffi::OsStr> {
  let file_name = path.file_name();
  if let Some(file_name) = file_name {
    return Some(file_name);
  }
  if let Some(path::Component::Prefix(prefix)) = path.components().next() {
    return Some(prefix.as_os_str())
  }
  return None
}

#[tauri::command]
pub fn request_directory(
  window: tauri::Window,
  state: tauri::State<state::AppState>,
  directory: String
) -> Result<Route, String> {
  let path = if directory.is_empty() {
    env::current_dir()
  } else {
    Ok(path::PathBuf::from(directory))
  }.map_err(|err| err.to_string())?;

  let components: Vec<RouteItem> = path.ancestors().map(|x| RouteItem {
    name: obtain_name(x).unwrap_or_default().to_str().unwrap().into(),
    path: x.to_str().unwrap().into()
  }).filter(|x| !x.name.is_empty()).collect();
  
  let path_name: String = path.to_str().unwrap().into();
  let manager = state.inner().fs_manager();
  state.associate_folder(window, path_name.clone());
  manager.get_entry(&path_name).map_err(|err| err.to_string())?;

  let prefixed = path.components().next().and_then(|x| Some(matches!(x, path::Component::Prefix(_)))).unwrap_or(false);
  return Ok(Route { items: components.into_iter().rev().collect(), path: path_name, prefixed: prefixed });
}