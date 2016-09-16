use address::{GlobalAddress, LocalAddress};
use processor::ProcessorId;
use weight::Weight;

// Don't implement From<Ref> for &mut Thunk or &Thunk!



pub struct Ref {
  global_address: GlobalAddress,
  weight: Weight,
}

impl Ref {
  pub fn new(a: GlobalAddress) -> Ref {
    Ref { global_address: a, weight: Weight::default() }
  }

  pub fn proc_id(&self) -> ProcessorId {
    self.global_address.proc_id
  }

  pub fn local_address(&self) -> LocalAddress {
    self.global_address.local_address
  }
}
