use std::{collections::HashMap, sync::{Arc, RwLock}, thread, fs, io, path::Path};

use crate::ds;

use super::*;

#[derive(Clone)]
pub struct FSManager {
  orders: Arc<RwLock<Vec<FSOrder>>>,
  sizes: Arc<RwLock<HashMap<String, u128>>>,
  listenners: Arc<RwLock<ds::Publisher<FSEvent>>>,
}

impl FSManager {
  pub fn new() -> Self {
    FSManager {
      orders: Arc::new(RwLock::new(Vec::new())),
      sizes: Arc::new(RwLock::new(HashMap::new())),
      listenners: Arc::new(RwLock::new(ds::Publisher::new())),
    }
  }

  pub fn listenners(&self) -> std::sync::RwLockWriteGuard<'_, ds::Publisher<FSEvent>> {
    self.listenners.write().unwrap()
  }

  pub fn orders(&self) -> Vec<FSOrder> {
    self.orders.read().unwrap().clone()
  }

  fn publish(&self, event: &FSEvent) {
    let reader = self.listenners.read().unwrap();
    reader.publish(event);
  }

  fn entries_to_fschilds(&self, entries: fs::ReadDir, calculating: Option<bool>, sizes: &mut HashMap<String, u128>) -> Vec<FSChild> {
    let mut readed: Vec<FSChild> = vec![];
    for entry in entries {
      if let Ok(entry) = entry {
        let mut child = FSChild::new(&entry);

        let calculating = calculating.unwrap_or_else(|| self.orders.read().unwrap().iter().find(|x| x.is_child(&child.path)).is_some());
        
        if let FSSizeStatus::NotCalculated = child.size {
          if let Some(size) = sizes.get(&child.path) {
            child.size = FSSizeStatus::Calculated(*size);
          } else if calculating {
            child.size = FSSizeStatus::Calculating;
          }
        }
        readed.push(child);
      }
    }
    readed
  }

  pub fn publish_entry(&self, path: &String) -> io::Result<()> {
    let manager: FSManager = self.clone();
    let path = path.clone();
    let entries = fs::read_dir(&path)?;
    thread::spawn(move || {
      let mut sizes = manager.sizes.write().unwrap();
      let childs = manager.entries_to_fschilds(entries, None, &mut sizes);
      manager.publish(&FSEvent::Entries { path, childs });
    });
    Ok(())
  }

  fn calculate_entry_rec(&self, path: &String) -> io::Result<u128> {
    let mut childs = {
      let mut sizes = self.sizes.write().unwrap();
      self.entries_to_fschilds(fs::read_dir(&path)?, Some(true), &mut sizes)
    };
    self.publish(&FSEvent::Entries { path: path.clone(), childs: childs.clone() });
    let mut result = 0;
    for child in &mut childs {
      result += match child.size {
        FSSizeStatus::Calculated(size) | FSSizeStatus::Known(size) => size,
        _ if matches!(child.child_type, FSChildType::Directory) => {
          if let Ok(size) = self.calculate_entry_rec(&child.path) {
            child.size = FSSizeStatus::Calculated(size);
            size
          } else {
            0
          }
        }
        _ => 0
      }
    }
    self.publish(&FSEvent::Entries { path: path.clone(), childs });
    let mut sizes = self.sizes.write().unwrap();
    sizes.insert(path.clone(), result);
    Ok(result)
  }

  pub fn process_order(&self, mut order: FSOrder) -> io::Result<()> {
    let mut orders = self.orders.write().unwrap();
    for _order in orders.iter() {
      if *_order >= order {
        return Ok(()); // Already processing an order containing the requesting tree
      }
    }
    let manager = self.clone();
    let path = order.path.clone();
    order.set_status(FSOrderStatus::Running);
    self.publish(&FSEvent::Order { order: order.clone() });
    orders.push(order);
    thread::spawn(move || {
      if let Some(_) = manager.sizes.read().unwrap().get(&path) {
        return
      }

      // Algorithm
      manager.calculate_entry_rec(&path).unwrap();

      // Update parent directory
      if let Some(parent) = Path::new(&path).parent() {
        if let Ok(entries) = fs::read_dir(&parent) {
          let mut sizes = manager.sizes.write().unwrap();
          let childs = manager.entries_to_fschilds(entries, None, &mut sizes);
          manager.publish(&FSEvent::Entries { path: parent.to_str().unwrap().to_string(), childs });
        }
      }

      // Update order
      let mut orders = manager.orders.write().unwrap();
      for _order in orders.iter_mut() {
        let order_path = _order.path.clone();
        if order_path == path {
          _order.set_status(FSOrderStatus::Finished);
          manager.publish(&FSEvent::Order { order: _order.clone() });
        }
      }
    });
    Ok(())
  }
}