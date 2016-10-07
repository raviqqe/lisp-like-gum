use local_id::{LocalId, FriendlyLocalId};



#[derive(Debug, Default)]
pub struct LocalIdManager {
  max: LocalId,
  used: Vec<LocalId>,
}

impl LocalIdManager {
  fn new() -> Self {
    LocalIdManager::default()
  }

  fn new_id(&mut self) -> LocalId {
    let i = self.max;
    self.max.increment();
    i
  }

  fn return_id(&mut self, i: LocalId) {
    self.used.push(i);
  }
}
