use global_address::GlobalAddress;
use local_address::LocalAddress;
use memory_id::MemoryId;
use weight::Weight;



#[derive(Debug, Serialize, Deserialize)]
pub struct Ref {
  global_address: GlobalAddress,
  weight: Weight,
}

impl Ref {
  pub fn new(a: GlobalAddress, w: Weight) -> Ref {
    Ref { global_address: a, weight: w }
  }

  pub fn global_address(&self) -> GlobalAddress {
    self.global_address
  }

  pub fn memory_id(&self) -> MemoryId {
    self.global_address.memory_id()
  }

  pub fn local_address(&self) -> LocalAddress {
    self.global_address.local_address()
  }

  pub fn split_weight(&mut self) -> (Weight, Option<Weight>) {
    self.weight.split()
  }
}
