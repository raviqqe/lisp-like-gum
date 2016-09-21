use address::{GlobalAddress, LocalAddress};
use processor::ProcessorId;
use reference::Ref;
use weight::Weight;



#[derive(Debug)]
pub struct MemoryCell {
  pub thunk: Thunk,
  pub weight: Weight,
  waits: u64,
}

impl MemoryCell {
  fn new(t: Thunk) -> MemoryCell {
    MemoryCell {
      thunk: t,
      weight: Weight::new(),
    }
  }
}

#[derive(Debug)]
pub struct Memory {
  proc_id: ProcessorId,
}

impl Memory {
  pub fn new(id: ProcessorId) -> Self {
    Memory { proc_id: id }
  }

  pub fn store(&mut self, t: Thunk) -> Ref {
    Ref::new(self.proc_id, Box::into_raw(Box::new(MemoryCell::new(t))));
  }

  pub fn load<'a>(&self, r: Ref) -> Option<&'a mut Thunk> {
    if r.proc_id() != self.proc_id {
      return None
    }

    Some(r.local_address().thunk)
  }

  pub fn add_weight(&mut self, a: GlobalAddress, dw: Weight) {
    assert_eq!(a.proc_id, self.proc_id);
    r.local_address.weight += dw;
  }

  pub fn delete_ref(&mut self, r: Ref) {
    assert_eq!(r.global_address.proc_id, self.proc_id);
    let a = r.local_address();
    a.weight -= r.weight;

    if a.weight == 0 {
      let _ = unsafe { Box::from_raw(a) };
    }
  }

  pub fn incre_waits(&mut self, a: GlobalAddress) {
    assert_eq!(a.proc_id, self.proc_id);
    a.local_address.waits += 1
  }

  pub fn decre_waits(&mut self, a: GlobalAddress) {
    assert_eq!(a.proc_id, self.proc_id);
    a.local_address.waits -= 1
  }
}
