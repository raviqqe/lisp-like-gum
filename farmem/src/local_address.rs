use std::any::TypeId;
use std::convert::{From, Into};

use consts::TYPE_ID_SIZE;



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

impl Into<TypeId> for LocalAddress {
  fn into(self) -> TypeId {
    unsafe { *((self.0 - *TYPE_ID_SIZE as u64) as *const TypeId) }
  }
}
