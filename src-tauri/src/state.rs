use std::{collections::HashMap, sync::{Arc, RwLock}};

use tauri::Window;

use crate::{fsop, ds::Subscriber};

#[derive(Clone)]
struct WindowNotifier {
  window_watcher: Arc<RwLock<HashMap<Window, String>>>,
}

impl WindowNotifier {
  pub fn new() -> Self {
    WindowNotifier { window_watcher: Arc::new(RwLock::new(HashMap::new())) }
  }

  pub fn set(&self, window: Window, path: String) {
    self.window_watcher.write().unwrap().insert(window, path);
  }
}

impl Subscriber<fsop::FSEvent> for WindowNotifier {
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

pub struct AppState {
  fs_manager: fsop::FSManager,
  window_notifier: WindowNotifier
}

impl AppState {
  pub fn new() -> Self {
    let fs_manager = fsop::FSManager::new();
    let window_notifier = WindowNotifier::new();
    fs_manager.listenners().subscribe(window_notifier.clone());
    AppState { fs_manager, window_notifier }
  }

  pub fn fs_manager(&self) -> &fsop::FSManager {
    &self.fs_manager
  }

  pub fn associate_folder(&self, window: Window, path: String) {
    self.window_notifier.set(window, path);
  }
}