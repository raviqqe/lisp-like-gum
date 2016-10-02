use global_address::GlobalAddress;
use local_address::LocalAddress;
use memory::MemoryId;
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
}

impl Drop for Ref {
  fn drop(&mut self) {
    unimplemented!() // Send SubWeight
  }
}

impl Clone for Ref {
  fn clone(&self) -> Self {
    let r = unsafe { &mut *(self as *const Self as *mut Self) };

    let (w, dw) = r.weight.split();

    if let Some(dw) = dw {
      unimplemented!(); // Send AddWeight { r.local_address(), dw }
    }

    Ref {
      global_address: r.global_address,
      weight: w,
    }
  }
}
