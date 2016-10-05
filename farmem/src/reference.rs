use global_address::GlobalAddress;
use local_address::LocalAddress;
use memory_id::MemoryId;
use weight::Weight;



#[derive(Debug, Serialize, Deserialize)]
pub struct Ref {
  global_address: GlobalAddress,
  weight: Weight,
}

pub trait FriendlyRef {
  fn new(a: GlobalAddress, w: Weight) -> Self;
  fn global_address(&self) -> GlobalAddress;
  fn local_address(&self) -> LocalAddress;
  fn memory_id(&self) -> MemoryId;
  fn split_weight(&mut self) -> (Weight, Option<Weight>);
  fn delete(self) -> (GlobalAddress, Weight);
}

impl FriendlyRef for Ref {
  fn new(a: GlobalAddress, w: Weight) -> Self {
    Ref { global_address: a, weight: w }
  }

  fn global_address(&self) -> GlobalAddress {
    self.global_address
  }

  fn local_address(&self) -> LocalAddress {
    self.global_address.local_address()
  }

  fn memory_id(&self) -> MemoryId {
    self.global_address.memory_id()
  }

  fn split_weight(&mut self) -> (Weight, Option<Weight>) {
    self.weight.split()
  }

  fn delete(self) -> (GlobalAddress, Weight) {
    (self.global_address, self.weight)
  }
}
