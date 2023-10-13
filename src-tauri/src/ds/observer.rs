pub trait Subscriber<T>: Send + Sync {
  fn id(&self) -> Option<&str> {
    None
  }
  fn notify(&self, event: &T);
}

pub struct Publisher<T> {
  listenners: Vec<Box<dyn Subscriber<T>>>
}

impl<T> Publisher<T> {
  pub fn new() -> Self {
    Publisher { listenners: Vec::new() }
  }

  pub fn publish(&self, event: &T) {
    for listenner in self.listenners.iter() {
      listenner.notify(event);
    }
  }

  pub fn subscribe(&mut self, subscriber: impl Subscriber<T> + 'static) {
    self.listenners.push(Box::new(subscriber))
  }

  pub fn unsubscribe(&mut self, id: impl AsRef<str>) {
    self.listenners.retain(|x| !x.id().and_then(|xid| Some(xid == id.as_ref())).unwrap_or(false))
  }
}