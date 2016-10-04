#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct TypeId(u64);

impl TypeId {
  pub fn new(i: u64) -> Self {
    TypeId(i)
  }
}

impl Into<usize> for TypeId {
  fn into(self) -> usize {
    self.0 as usize
  }
}
