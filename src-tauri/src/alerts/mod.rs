use crate::fsop::{FSChild, FSChildType, FSSizeStatus};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Unit {
  symbol: String,
  factor: u32
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AlertItem {
  #[serde(rename = "folders")]
  Folders,
  #[serde(rename = "files")]
  Files,
  #[serde(rename = "all")]
  All
}

impl AlertItem {
  pub fn matches(&self, child_type: &FSChildType) -> bool {
    match (self, child_type) {
      (Self::Folders, FSChildType::Directory) |
      (Self::Files, FSChildType::File) |
      (Self::All, FSChildType::Directory | FSChildType::File) => true,
      _ => false
    }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlertFilter {
  #[serde(rename = "minSize")]
  min_size: u128,
  #[serde(rename = "sizeUnit")]
  unit: Unit,
  item: AlertItem
}

impl AlertFilter {
  pub fn size(&self) -> u128 {
    self.min_size * 10_u128.pow(self.unit.factor)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Alert {
  name: String,
  filter: AlertFilter
}

impl Alert {
  pub fn matches(&self, child: &FSChild) -> bool {
    if !self.filter.item.matches(child.child_type()) {
      return false
    }
    match child.size() {
      FSSizeStatus::Calculated(size) | FSSizeStatus::Known(size) if *size < self.filter.size() => true,
      _ => false
    }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Detection {
  alert: Alert,
  child: FSChild
}

impl Detection {
  pub fn new(alert: Alert, child: FSChild) -> Self {
    Self { alert, child }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AlertEvent {
  #[serde(rename = "load")]
  Load {
    alerts: Vec<Alert>
  },
  #[serde(rename = "trigger")]
  Trigger(Detection)
}

impl AlertEvent {
  pub fn new_load(alerts: Vec<Alert>) -> Self {
    Self::Load { alerts }
  }

  pub fn new_trigger(alert: &Alert, child: &FSChild) -> Self {
    Self::Trigger(Detection::new(alert.clone(), child.clone()))
  }
}