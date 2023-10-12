use crate::fsop::FSManager;

pub struct AppState {
  fs_manager: FSManager
}

impl AppState {
  pub fn new() -> Self {
    AppState { fs_manager: FSManager::new()  }
  }

  pub fn fs_manager(&self) -> &FSManager {
    &self.fs_manager
  }
}