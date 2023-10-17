use std::{collections::HashMap, sync::{Arc, RwLock}, thread, fs, io};

use crate::ds;

mod data;

pub use data::*;

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
    let entries = fs::read_dir(&path)?;
    thread::spawn(move || {
      let mut readed: Vec<FSChild> = vec![];
      for entry in entries {
        if let Ok(entry) = entry {
          readed.push(FSChild::new(&entry))
        }
      }
      let reader = manager.listenners.read().unwrap();
      reader.publish(&FSEvent::Entry { path: path, data: readed });
    });
    Ok(())
  }
}