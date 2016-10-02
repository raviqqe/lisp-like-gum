use memory::MemoryId;
use local_address::LocalAddress;



#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct GlobalAddress {
  memory_id: MemoryId,
  local_address: LocalAddress,
}

impl GlobalAddress {
  pub fn new(i: MemoryId, a: LocalAddress) -> Self {
    GlobalAddress { memory_id: i, local_address: a }
  }

  pub fn memory_id(&self) -> MemoryId { self.memory_id }
  pub fn local_address(&self) -> LocalAddress { self.local_address }
}
