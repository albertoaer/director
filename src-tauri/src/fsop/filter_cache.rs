use std::{collections::HashSet, mem::swap};

use log::info;

use super::{Alert, Detection, FSChild};

#[derive(Clone)]
pub struct Filter(Vec<Alert>);

impl Filter {
  pub fn new(vec: Vec<Alert>) -> Self {
    Self(vec)
  }

  pub fn filter(&self, child: &FSChild) -> Option<Detection> {
    for alert in &self.0 {
      if alert.matches(child) {
        return Some(Detection::new(alert.clone(), child.clone()))
      }
    }
    None
  }
}

fn filter_split<I, T, K, F>(iter: I, predicate: F) -> (Vec<T>, Vec<K>) where I: Iterator<Item = T>, F: Fn(&T) -> Option<K> {
  let mut a = Vec::new();
  let mut b = Vec::new();
  for item in iter {
    if let Some(k) = predicate(&item) {
      b.push(k)
    } else {
      a.push(item)
    }
  }
  (a, b)
}

#[derive(Clone)]
pub struct FilterCache {
  cached: HashSet<String>,
  
  filter: Filter,

  filtered: Vec<Detection>,
  remain: Vec<FSChild>
}

impl FilterCache {
  pub fn new() -> Self {
    Self {
      cached: HashSet::new(),
      
      filter: Filter(Vec::new()),

      filtered: Vec::new(),
      remain: Vec::new()
    }
  }

  // returns whether whatever using the data should refresh due to the filter update
  pub fn set_filter(&mut self, filter: Filter) -> bool {
    // TODO: compare filters to avoid extra work
    let mut filtered = Vec::new();
    swap(&mut self.filtered, &mut filtered);
    let (remain_a, filtered_a) = filter_split(filtered.into_iter().map(|x| x.child), |x| filter.filter(x));

    let mut remain = Vec::new();
    swap(&mut self.remain, &mut remain);
    let (remain_b, filtered_b) = filter_split(remain.into_iter(), |x| filter.filter(x));

    self.filtered.extend_from_slice(&filtered_a);
    self.filtered.extend_from_slice(&filtered_b);

    self.remain.extend_from_slice(&remain_a);
    self.remain.extend_from_slice(&remain_b);

    info!("filtered: {} remain: {}", self.filtered.len(), self.remain.len());

    self.filter = filter;
    true
  }

  pub fn include(&mut self, child: FSChild) {
    if self.cached.insert(child.path.clone()) {
      if let Some(detection) = self.filter.filter(&child) {
        self.filtered.push(detection)
      } else {
        self.remain.push(child)
      }
    }
  }

  pub fn get_range<B>(&self, begin: usize, count: usize) -> B where B: FromIterator<Detection> {
    info!("requested, begin: {} count: {}, filtered: {}", begin, count, self.filtered.len());
    self.filtered.iter().skip(begin).take(count).map(|x| x.clone()).collect()
  }
}