#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
         Serialize, Deserialize)]
pub struct TypeId(u64);

impl TypeId {
  pub fn new(i: u64) -> Self {
    TypeId(i)
  }
}

impl From<TypeId> for usize {
  fn from(t: TypeId) -> Self {
    t.0 as usize
  }
}
