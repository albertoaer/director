use crate::state;

use std::{path, env, ffi};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PathComponent {
  name: String,
  path: String
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
pub fn request_directory(state: tauri::State<state::AppState>, directory: String) -> Result<Vec<PathComponent>, String> {
  let path = if directory.is_empty() {
    env::current_dir()
  } else {
    Ok(path::PathBuf::from(directory))
  }.map_err(|err| err.to_string())?;
  let components: Vec<PathComponent> = path.ancestors().map(|x| PathComponent {
    name: obtain_name(x).unwrap_or_default().to_str().unwrap().into(),
    path: x.to_str().unwrap().into()
  }).filter(|x| !x.name.is_empty()).collect();
  return Ok(components.into_iter().rev().collect());
}