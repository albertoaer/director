use crate::fsop::{FSChild, FSChildType, FSSizeStatus};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Unit {
  symbol: String,
  factor: u32
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum AlertItem {
  #[serde(rename = "folders")]
  Folders,
  #[serde(rename = "files")]
  Files,
  #[serde(rename = "any")]
  Any
}

impl AlertItem {
  pub fn matches(&self, child_type: &FSChildType) -> bool {
    match (self, child_type) {
      (Self::Folders, FSChildType::Directory) |
      (Self::Files, FSChildType::File) |
      (Self::Any, FSChildType::Directory | FSChildType::File) => true,
      _ => false
    }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlertFilter {
  #[serde(rename = "minSize")]
  pub(super) min_size: u128,
  #[serde(rename = "sizeUnit")]
  pub(super) unit: Unit,
  pub(super) item: AlertItem
}

impl AlertFilter {
  pub fn size(&self) -> u128 {
    self.min_size * 10_u128.pow(self.unit.factor)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Alert {
  pub(super) name: String,
  pub(super) filter: AlertFilter
}

impl Alert {
  pub fn matches(&self, child: &FSChild) -> bool {
    if !self.filter.item.matches(child.child_type()) {
      return false
    }
    match child.size() {
      FSSizeStatus::Calculated(size) | FSSizeStatus::Known(size) if *size >= self.filter.size() => true,
      _ => false
    }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Detection {
  pub(super) alert: Alert,
  pub(super) child: FSChild
}

impl Detection {
  pub fn new(alert: Alert, child: FSChild) -> Self {
    Self { alert, child }
  }
}