#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct TypeId(u64);

impl TypeId {
  fn new(i: u64) -> Self {
    TypeId(i)
  }
}
