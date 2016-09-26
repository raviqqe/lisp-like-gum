use std::ops::{Deref, DerefMut};
use std::convert::{From, Into};

use processor::ProcessorId;
use thunk::Thunk;
use weighted::Weighted;



#[derive(Debug, Clone, Copy)]
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

impl Deref for LocalAddress {
  type Target = Weighted<Thunk>;

  fn deref(&self) -> &Self::Target {
    &*(self as *const Self::Target)
  }
}

impl DerefMut for LocalAddress {
  fn deref_mut(&mut self) -> &mut Weighted<Thunk> {
    &mut *(self as *mut Weighted<Thunk>)
  }
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalAddress {
  pub proc_id: ProcessorId,
  pub local_address: LocalAddress,
}

impl GlobalAddress {
  pub fn new(id: ProcessorId, a: LocalAddress) -> Self {
    GlobalAddress { proc_id: id, local_address: a }
  }
}
