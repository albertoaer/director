use std::{env, path, ffi};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouteItem {
  name: String,
  path: String
}

impl RouteItem {
  pub fn new(name: String, path: String) -> Self {
    Self { name, path }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn path(&self) -> &String {
    &self.path
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Route {
  items: Vec<RouteItem>,
  path: String,
  prefixed: bool
}

impl Route {
  pub fn new(items: Vec<RouteItem>, path: String, prefixed: bool) -> Self {
    Self { items, path, prefixed }
  }

  pub fn items(&self) -> &Vec<RouteItem> {
    &self.items
  }

  pub fn path(&self) -> &String {
    &self.path
  }

  pub fn prefixed(&self) -> &bool {
    &self.prefixed
  }
}

impl TryFrom<String> for Route {
  type Error = String;

  fn try_from(route: String) -> Result<Self, Self::Error> {
    let path = if route.is_empty() {
      env::current_dir()
    } else {
      Ok(path::PathBuf::from(route))
    }.map_err(|err| err.to_string())?;
  
    fn obtain_name(path: &path::Path) -> Option<&ffi::OsStr> {
      let file_name = path.file_name();
      if let Some(file_name) = file_name {
        return Some(file_name);
      }
      if let Some(path::Component::Prefix(prefix)) = path.components().next() {
        return Some(prefix.as_os_str())
      }
      return None
    }
  
    let components: Vec<RouteItem> = path.ancestors().map(|x| RouteItem::new(
      obtain_name(x).unwrap_or_default().to_str().unwrap().into(),
      x.to_str().unwrap().into()
    )).filter(|x| !x.name().is_empty()).collect();
  
    let prefixed = path.components().next().and_then(|x| Some(matches!(x, path::Component::Prefix(_)))).unwrap_or(false);
    Ok(Route::new(components.into_iter().rev().collect(), path.to_str().unwrap().into(), prefixed))
  }
}