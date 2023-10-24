use std::{collections::HashMap, sync::{Arc, RwLock}};

use tauri::{Window, AppHandle, Manager};

use crate::{fsop, ds::Subscriber, alerts};

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
      fsop::FSEvent::Entry { path, .. } => {
        for (window, _path) in self.window_watcher.read().unwrap().iter() {
          if _path != path {
            continue;
          }
          window.emit("updated-entry", event).unwrap();
        }
      },
      _ => ()
    }
  }
}

#[derive(Clone)]
pub struct AlertNotifier {
  app_handle: Arc<RwLock<Option<AppHandle>>>,
  alerts: Arc<RwLock<Vec<alerts::Alert>>>,
}

impl AlertNotifier {
  pub fn new() -> Self {
    Self { app_handle: Arc::new(RwLock::new(None)), alerts: Arc::new(RwLock::new(Vec::new())) }
  }

  pub fn set_alerts(&self, alerts: Vec<alerts::Alert>) {
    *self.alerts.write().unwrap() = alerts;
  }

  pub fn set_handler(&self, app_handle: AppHandle) {
    *self.app_handle.write().unwrap() = Some(app_handle);
  }
}

impl Subscriber<fsop::FSEvent> for AlertNotifier {
  fn notify(&self, event: &fsop::FSEvent) {
    let app_handle = self.app_handle.read().unwrap().clone();
    if let Some(app_handle) = app_handle { 
      let alerts = self.alerts.read().unwrap().clone();
      match event {
        fsop::FSEvent::Entry { childs, .. } => {
          'childs: for child in childs {
            for alert in &alerts {
              if alert.matches(child) {
                app_handle.emit_all("alert", alerts::AlertEvent::new_trigger(&alert, &child)).unwrap();
                continue 'childs;
              }
            }
          }
        },
        _ => ()
      }
    }
  }
}

pub struct AppState {
  fs_manager: fsop::FSManager,
  route_notifier: RouteNotifier,
  alert_notifier: AlertNotifier
}

impl AppState {
  pub fn new() -> Self {
    let fs_manager = fsop::FSManager::new();
    let route_notifier = RouteNotifier::new();
    let alert_notifier = AlertNotifier::new();
    fs_manager.listenners().subscribe(route_notifier.clone());
    fs_manager.listenners().subscribe(alert_notifier.clone());
    AppState { fs_manager, route_notifier, alert_notifier }
  }

  pub fn fs_manager(&self) -> &fsop::FSManager {
    &self.fs_manager
  }

  pub fn route_notifier(&self) -> &RouteNotifier {
    &self.route_notifier
  }

  pub fn alert_notifier(&self) -> &AlertNotifier {
    &self.alert_notifier
  }
}