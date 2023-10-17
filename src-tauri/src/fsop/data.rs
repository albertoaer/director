use std::fs;

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
  name: String,
  path: String,
  size: FSSizeStatus,
  modified: Option<u128>,
  created: Option<u128>,
  #[serde(rename = "type")]
  child_type: FSChildType,
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
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FSEntry {
  name: String,
  childs: Vec<String>,
  size: u64
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FSEvent {
  #[serde(rename = "entry")]
  Entry {
    path: String,
    data: Vec<FSChild>
  }
}