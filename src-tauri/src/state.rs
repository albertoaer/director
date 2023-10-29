use std::{collections::{HashMap, HashSet}, sync::{Arc, RwLock}};

use tauri::{Window, AppHandle, Manager};

use crate::{fsop, ds::Subscriber, persistency::{Persist, PersistencyFile}};

#[derive(Clone)]
pub struct RouteNotifier {
  window_watcher: Arc<RwLock<HashMap<Window, String>>>,
}

impl RouteNotifier {
  pub fn new() -> Self {
    Self { window_watcher: Arc::new(RwLock::new(HashMap::new())) }
  }

  pub fn set(&self, window: Window, path: String) {
    self.window_watcher.write().unwrap().insert(window, path);
  }
}

impl Subscriber<fsop::FSEvent> for RouteNotifier {
  fn notify(&self, event: &fsop::FSEvent) {
    match event {
      fsop::FSEvent::Entries { path, .. } => {
        for (window, _path) in self.window_watcher.read().unwrap().iter() {
          if _path != path {
            continue;
          }
          window.emit("updated-entries", event).unwrap();
        }
      },
      _ => ()
    }
  }
}

#[derive(Clone)]
pub struct AlertNotifier {
  persistency_file: Option<PersistencyFile>,
  app_handle: AppHandle,
  alerts: Arc<RwLock<Vec<fsop::Alert>>>,
  detections: Arc<RwLock<(HashSet<String>, Vec<fsop::Detection>)>>
}

impl AlertNotifier {
  pub fn new(persistency_file: Option<PersistencyFile>, app_handle: AppHandle) -> Self {
    Self {
      app_handle,
      alerts: Arc::new(RwLock::new(persistency_file.load().unwrap_or_default())),
      persistency_file,
      detections: Arc::new(RwLock::new((HashSet::new(), Vec::new())))
    }
  }

  pub fn set_alerts(&self, alerts: Vec<fsop::Alert>) {
    self.persistency_file.save(&alerts);
    self.app_handle.emit_all("load-alerts", &alerts).ok();
    *self.alerts.write().unwrap() = alerts;
  }

  pub fn alerts(&self) -> Vec<fsop::Alert> {
    self.alerts.read().unwrap().clone()
  }

  pub fn get_detections(&self, begin: usize, count: usize) -> Vec<fsop::Detection> {
    self.detections.read().unwrap().1.iter()
      .take(begin + count).skip(begin).map(|x| x.clone()).collect()
  }
}

impl Subscriber<fsop::FSEvent> for AlertNotifier {
  fn notify(&self, event: &fsop::FSEvent) {
    let alerts = self.alerts.read().unwrap().clone();
    match event {
      fsop::FSEvent::Entries { childs, .. } => {
        'childs: for child in childs {
          for alert in &alerts {
            if alert.matches(child) {
              let mut detections = self.detections.write().unwrap();
              if detections.0.insert(child.path().clone()) {
                detections.1.push(
                  fsop::Detection::new(alert.clone(), child.clone())
                );
              }
              continue 'childs;
            }
          }
        }
      }
      fsop::FSEvent::Order { order, .. } => {
        self.app_handle.emit_all("order", order).unwrap();
      }
    }
  }
}