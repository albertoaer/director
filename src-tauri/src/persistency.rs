use std::{path::PathBuf, fs};

use serde::{Serialize, de::DeserializeOwned};
use tauri::{api::dialog, Window};
use dirs;

const APP_CONFIG_DIRECTORY: &'static str = "director";

pub trait Persist {
  fn save<T>(&self, data: &T) where T: ?Sized + Serialize;

  fn load<T>(&self) -> Result<T, String> where T: DeserializeOwned;
}

#[derive(Clone, Debug)]
pub struct PersistencyFile {
  path: PathBuf
}

impl PersistencyFile {
  pub fn new(path: impl Into<PathBuf>) -> Self {
    Self { path: path.into() }
  }

  pub fn new_config(name: impl Into<String>) -> Self {
    Self::new(dirs::config_dir().unwrap().join(APP_CONFIG_DIRECTORY).join(name.into() + ".json"))
  }
}

impl Persist for PersistencyFile {
  fn save<T>(&self, data: &T) where T: ?Sized + Serialize {
    match serde_json::to_string_pretty(&data)
      .map_err(|err| err.to_string())
      .and_then(
        |data|
          if let Some(path) = self.path.parent() {
            fs::create_dir_all(path)
          } else {
            Ok(())
          }.and_then(|_| fs::write(&self.path, data)).map_err(|err| err.to_string())
      )
    {
      Err(err) => dialog::message(None::<&Window>, "Error storing alerts", err.to_string()),
      _ => (),
    };
  }

  fn load<T>(&self) -> Result<T, String> where T: DeserializeOwned {
    let data = fs::read_to_string(&self.path).map_err(|err| err.to_string())?;
    serde_json::from_str(data.as_str()).map_err(|err| err.to_string())
  }
}

impl Persist for Option<PersistencyFile> {
  fn save<T>(&self, data: &T) where T: ?Sized + Serialize {
    self.as_ref().and_then(|x| Some(x.save(data)));
  }

  fn load<T>(&self) -> Result<T, String> where T: DeserializeOwned {
    self.as_ref().ok_or(String::from("No persistency")).and_then(|x| x.load())
  }
}