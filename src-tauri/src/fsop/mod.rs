use std::{collections::HashMap, sync::{Arc, RwLock}, thread, fs, io};

pub type EntryId = String; // identify filesystem entries by its path

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FSChild {
  name: String,
  path: String,
  size: Option<u64>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FSEntry {
  name: String,
  childs: Vec<EntryId>,
  size: u64,
}

pub trait FSListenner: Send + Sync {
  fn on_entry_update(&self, id: &EntryId, data: &Vec<FSChild>);
}

#[derive(Clone)]
pub struct FSManager {
  data: Arc<RwLock<HashMap<EntryId, FSEntry>>>,
  listenners: Arc<RwLock<HashMap<EntryId, Vec<Box<dyn FSListenner>>>>>
}

impl FSManager {
  pub fn new() -> Self {
    FSManager {
      data: Arc::new(RwLock::new(HashMap::new())),
      listenners: Arc::new(RwLock::new(HashMap::new()))
    }
  }

  pub fn listen(&self, id: &EntryId, item: impl FSListenner + 'static) {
    let mut writer = self.listenners.write().unwrap();
    if let Some(list) = writer.get_mut(id) {
      list.push(Box::new(item));
    } else {
      writer.insert(id.clone(), vec![Box::new(item)]);
    }
  }

  pub fn process_entry(&self, id: &EntryId) -> io::Result<()> {
    let manager: FSManager = self.clone();
    let id = id.clone();
    let dir_data = fs::read_dir(&id)?;
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
      if let Some(list) = reader.get(&id) {
        for listenner in list {
          listenner.on_entry_update(&id, &readed);
        }
      }
    });
    Ok(())
  }
}