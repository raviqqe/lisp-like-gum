use local_id::LocalId;
use memory_id::MemoryId;



#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct GlobalAddress {
  memory_id: MemoryId,
  local_id: LocalId,
}

impl GlobalAddress {
  pub fn new(m: MemoryId, l: LocalId) -> Self {
    GlobalAddress { memory_id: m, local_id: l }
  }

  pub fn memory_id(&self) -> MemoryId { self.memory_id }
  pub fn local_id(&self) -> LocalId { self.local_id }
}
