use memory_id::MemoryId;



#[derive(Debug, Copy, Clone)]
pub struct Demand(MemoryId);

pub trait FriendlyDemand {
  fn new(MemoryId) -> Self;
  fn memory_id(&self) -> MemoryId;
}

impl FriendlyDemand for Demand {
  fn new(i: MemoryId) -> Self {
    Demand(i)
  }

  fn memory_id(&self) -> MemoryId {
    self.0
  }
}
