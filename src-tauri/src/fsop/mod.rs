use std::{collections::HashMap, sync::{Arc, RwLock}, thread, fs, io};

use crate::ds;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FSChild {
  name: String,
  path: String,
  size: Option<u64>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FSEntry {
  name: String,
  childs: Vec<String>,
  size: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FSEvent {
  #[serde(rename = "entry")]
  Entry {
    path: String,
    data: Vec<FSChild>
  }
}

#[derive(Clone)]
pub struct FSManager {
  data: Arc<RwLock<HashMap<String, FSEntry>>>,
  listenners: Arc<RwLock<ds::Publisher<FSEvent>>>
}

impl FSManager {
  pub fn new() -> Self {
    FSManager {
      data: Arc::new(RwLock::new(HashMap::new())),
      listenners: Arc::new(RwLock::new(ds::Publisher::new()))
    }
  }

  pub fn listenners(&self) -> std::sync::RwLockWriteGuard<'_, ds::Publisher<FSEvent>> {
    return self.listenners.write().unwrap();
  }

  pub fn process_entry(&self, path: &String) -> io::Result<()> {
    let manager: FSManager = self.clone();
    let path = path.clone();
    let dir_data = fs::read_dir(&path)?;
    thread::spawn(move || {
      let mut readed: Vec<FSChild> = vec![];
      for item in dir_data {
        if let Ok(item) = item {
          readed.push(FSChild {
            name: item.file_name().to_str().unwrap().into(),
            path: item.path().to_str().unwrap().into(),
            size: None
          })
        }
      }
      let reader = manager.listenners.read().unwrap();
      reader.publish(&FSEvent::Entry { path: path, data: readed });
    });
    Ok(())
  }
}