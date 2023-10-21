use std::{collections::HashMap, sync::{Arc, RwLock}, thread, fs, io};

use crate::ds;

mod data;
pub use data::*;

mod routes;
pub use routes::*;

#[derive(Clone)]
pub struct FSManager {
  sizes: Arc<RwLock<HashMap<String, u128>>>,
  listenners: Arc<RwLock<ds::Publisher<FSEvent>>>
}

impl FSManager {
  pub fn new() -> Self {
    FSManager {
      sizes: Arc::new(RwLock::new(HashMap::new())),
      listenners: Arc::new(RwLock::new(ds::Publisher::new()))
    }
  }

  pub fn listenners(&self) -> std::sync::RwLockWriteGuard<'_, ds::Publisher<FSEvent>> {
    return self.listenners.write().unwrap();
  }

  fn publish(&self, event: &FSEvent) {
    let reader = self.listenners.read().unwrap();
    reader.publish(event);
  }

  fn entries_to_fschilds(entries: fs::ReadDir, sizes: &mut HashMap<String, u128>) -> Vec<FSChild> {
    let mut readed: Vec<FSChild> = vec![];
    for entry in entries {
      if let Ok(entry) = entry {
        let mut child = FSChild::new(&entry);
        if let Some(size) = sizes.get(&child.path) {
          child.size = FSSizeStatus::Calculated(*size);
        }
        readed.push(child);
      }
    }
    readed
  }

  pub fn get_entry(&self, path: &String) -> io::Result<()> {
    let manager: FSManager = self.clone();
    let path = path.clone();
    let entries = fs::read_dir(&path)?;
    thread::spawn(move || {
      let mut sizes = manager.sizes.write().unwrap();
      let childs = Self::entries_to_fschilds(entries, &mut sizes);
      manager.publish(&FSEvent::Entry { path, childs });
    });
    Ok(())
  }

  fn calculate_entry_rec(&self, path: &String) -> io::Result<u128> {
    let mut childs = {
      let mut sizes = self.sizes.write().unwrap();
      Self::entries_to_fschilds(fs::read_dir(&path)?, &mut sizes)
    };
    for child in &mut childs {
      if let FSSizeStatus::NotCalculated = child.size {
        child.size = FSSizeStatus::Calculating
      }
    }
    self.publish(&FSEvent::Entry { path: path.clone(), childs: childs.clone() });
    let mut result = 0;
    for child in &mut childs {
      result += match child.size {
        FSSizeStatus::Calculated(size) | FSSizeStatus::Known(size) => size,
        _ if matches!(child.child_type, FSChildType::Directory) => {
          let size = self.calculate_entry_rec(&child.path)?;
          child.size = FSSizeStatus::Calculated(size);
          size
        }
        _ => 0
      }
    }
    self.publish(&FSEvent::Entry { path: path.clone(), childs });
    let mut sizes = self.sizes.write().unwrap();
    sizes.insert(path.clone(), result);
    Ok(result)
  }

  pub fn calculate_entry(&self, path: &String) -> io::Result<()> {
    let manager = self.clone();
    let path = path.clone();
    thread::spawn(move || {
      if let Some(_) = manager.sizes.read().unwrap().get(&path) {
        return
      }
      manager.calculate_entry_rec(&path).unwrap();
    });
    Ok(())
  }
}