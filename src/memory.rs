use std::collections::BTreeMap;

use address::LocalAddress;
use processor::ProcessorId;
use reference::Ref;
use weight::Weight;



pub struct Memory {
  proc_id: ProcessorId,
  weights: BTreeMap<LocalAddress, Weight>
}

impl Memory {
  fn new(id: ProcessorId) -> Self {
    Memory { id: id, weights: BTreeMap::new() }
  }

  fn store(&mut self, t: Thunk) -> Ref {
    let r = Ref::new(self.proc_id, Box::into_raw(Box::new()));

    self.weights.insert(r.local_address, r.weight);

    r
  }

  fn load<'a>(&self, r: Ref) -> Option<&'a mut Thunk> {
    if r.proc_id != self.proc_id {
      return None
    }

    Some(&mut *(r.local_address as *mut Thunk))
  }

  fn incre_weight(&mut self, r: Ref, dw: Weight) {
    *self.weights.get_mut(r.local_address).unwrap() += dw;
  }

  fn decre_weight(&mut self, r: Ref, dw: Weight) {
    let a = r.local_address;
    let w = self.weights.get_mut(a).unwrap();

    if *w == dw {
      self.weights.remove(a);
      unsafe { Box::from_raw(a) };
    } else {
      *w -= dw;
    }
  }
}
