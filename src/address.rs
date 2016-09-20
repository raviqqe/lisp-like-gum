use memory::MemoryCell;
use processor::ProcessorId;
use thunk::Thunk;
use weight::Weight;



pub type LocalAddress = u64;

impl Deref for LocalAddress {
  type Target = MemoryCell;

  fn deref(&self) -> &Self::Target {
    &*(self as *const Self::Target)
  }
}

impl DerefMut for LocalAddress {
  type Target = MemoryCell;

  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut *(self as *mut Self::Target)
  }
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalAddress {
  proc_id: ProcessorId,
  local_address: LocalAddress,
}

impl GlobalAddress {
  fn new(id: ProcessorId, a: LocalAddress) -> Self {
    GlobalAddress { proc_id: id, local_address: a }
  }
}
