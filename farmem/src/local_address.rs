use std::convert::{From, Into};



#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct LocalAddress(u64);

impl From<u64> for LocalAddress {
  fn from(n: u64) -> LocalAddress {
    LocalAddress(n)
  }
}

impl Into<u64> for LocalAddress {
  fn into(self) -> u64 {
    self.0
  }
}
