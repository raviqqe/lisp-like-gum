use processor::ProcessorId;
use thunk::Thunk;
use weighted::Weighted;



pub type LocalAddress = u64;

impl Deref for LocalAddress {
  type Target = Weighted<Thunk>;

  fn deref(&self) -> &Self::Target {
    &*(self as *const Self::Target)
  }
}

impl DerefMut for LocalAddress {
  type Target = Weighted<Thunk>;

  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut *(self as *mut Self::Target)
  }
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalAddress {
  pub proc_id: ProcessorId,
  pub local_address: LocalAddress,
}

impl GlobalAddress {
  fn new(id: ProcessorId, a: LocalAddress) -> Self {
    GlobalAddress { proc_id: id, local_address: a }
  }
}
