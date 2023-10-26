use std::{fs, path::PathBuf};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FSChildType {
  #[serde(rename = "file")]
  File,
  #[serde(rename = "directory")]
  Directory,
  #[serde(rename = "link")]
  Link,
  #[serde(rename = "other")]
  Other
}

impl From<fs::FileType> for FSChildType {
  fn from(file_type: fs::FileType) -> Self {
    if file_type.is_dir() {
      FSChildType::Directory
    } else if file_type.is_file() {
      FSChildType::File
    } else if file_type.is_symlink() {
      FSChildType::Link
    } else {
      FSChildType::Other
    }
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", content = "value")]
pub enum FSSizeStatus {
  #[serde(rename = "Not Calculated")]
  NotCalculated,
  Calculating,
  Calculated(u128),
  Known(u128)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FSChild {
  pub(super) name: String,
  pub(super) path: String,
  pub(super) size: FSSizeStatus,
  pub(super) modified: Option<u128>,
  pub(super) created: Option<u128>,
  #[serde(rename = "type")]
  pub(super) child_type: FSChildType
}

impl FSChild {
  pub fn new(file: &fs::DirEntry) -> Self {
    let metadata = file.metadata().unwrap();
    let child_type = metadata.file_type().into();
    FSChild {
      name: file.file_name().to_str().unwrap().into(),
      path: file.path().to_str().unwrap().into(),
      size: match child_type {
        FSChildType::Directory => FSSizeStatus::NotCalculated,
        _ => FSSizeStatus::Known(metadata.len().into()),
      },
      modified: metadata.modified().ok().and_then(|x| x.elapsed().ok().and_then(|x| Some(x.as_millis()))),
      created: metadata.created().ok().and_then(|x| x.elapsed().ok().and_then(|x| Some(x.as_millis()))),
      child_type
    }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn path(&self) -> &String {
    &self.path
  }

  pub fn size(&self) -> &FSSizeStatus {
    &self.size
  }

  pub fn modified(&self) -> &Option<u128> {
    &self.modified
  }

  pub fn created(&self) -> &Option<u128> {
    &self.created
  }

  pub fn child_type(&self) -> &FSChildType {
    &self.child_type
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum FSEvent {
  Entries {
    path: String,
    childs: Vec<FSChild>
  },
  Order {
    order: FSOrder
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FSOrderStatus {
  Unknown,
  Running,
  Finished
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FSOrder {
  pub(super) path: PathBuf,
  pub(super) status: FSOrderStatus
}

impl FSOrder {
  pub fn new(path: impl Into<PathBuf>) -> Self {
    FSOrder { path: path.into(), status: FSOrderStatus::Unknown }
  }

  pub fn is_child(&self, other: impl Into<PathBuf>) -> bool {
    let path: PathBuf = other.into();
    path.starts_with(&self.path)
  }
  
  pub fn set_status(&mut self, status: FSOrderStatus) {
    self.status = status
  }
}

impl PartialEq for FSOrder {
  fn eq(&self, other: &Self) -> bool {
    self.path == other.path
  }
}

impl PartialOrd for FSOrder {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    if self.path == other.path {
      return Some(std::cmp::Ordering::Equal);
    }
    if self.is_child(&other.path) {
      Some(std::cmp::Ordering::Greater)
    } else if other.is_child(&self.path) {
      Some(std::cmp::Ordering::Less)
    } else {
      None
    }
  }
}