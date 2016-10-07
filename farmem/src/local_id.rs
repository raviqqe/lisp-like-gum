#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default,
         Serialize, Deserialize)]
pub struct LocalId(u64);

pub trait FriendlyLocalId {
  fn increment(&mut self);
}

impl FriendlyLocalId for LocalId {
  fn increment(&mut self) {
    self.0 += 1
  }
}
