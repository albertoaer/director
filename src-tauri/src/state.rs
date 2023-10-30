use std::{collections::HashMap, sync::{Arc, RwLock}};

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
  cache: Arc<RwLock<fsop::FilterCache>>
}

impl AlertNotifier {
  pub fn new(persistency_file: Option<PersistencyFile>, app_handle: AppHandle) -> Self {
    Self {
      app_handle,
      alerts: Arc::new(RwLock::new(persistency_file.load().unwrap_or_default())),
      persistency_file,
      cache: Arc::new(RwLock::new(fsop::FilterCache::new()))
    }
  }

  pub fn set_alerts(&self, alerts: Vec<fsop::Alert>) -> bool {
    self.persistency_file.save(&alerts);
    self.app_handle.emit_all("load-alerts", &alerts).ok();
    let refresh = self.cache.write().unwrap().set_filter(fsop::Filter::new(alerts.clone()));
    *self.alerts.write().unwrap() = alerts;
    refresh
  }

  pub fn alerts(&self) -> Vec<fsop::Alert> {
    self.alerts.read().unwrap().clone()
  }

  pub fn get_detections(&self, begin: usize, count: usize) -> Vec<fsop::Detection> {
    self.cache.read().unwrap().get_range(begin, count)
  }
}

impl Subscriber<fsop::FSEvent> for AlertNotifier {
  fn notify(&self, event: &fsop::FSEvent) {
    match event {
      fsop::FSEvent::Entries { childs, .. } => {
        let mut writer = self.cache.write().unwrap();
        for child in childs {
          writer.include(child.clone());
        }
      }
      fsop::FSEvent::Order { order, .. } => {
        self.app_handle.emit_all("order", order).unwrap();
      }
    }
  }
}