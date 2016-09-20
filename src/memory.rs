use address::{GlobalAddress, LocalAddress};
use processor::ProcessorId;
use reference::Ref;
use weight::Weight;



#[derive(Debug)]
pub struct MemoryCell {
  pub thunk: Thunk,
  pub weight: Weight,
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
  fn new(id: ProcessorId) -> Self {
    Memory { proc_id: id }
  }

  fn store(&mut self, t: Thunk) -> Ref {
    Ref::new(self.proc_id, Box::into_raw(Box::new(MemoryCell::new(t))));
  }

  fn load<'a>(&self, r: Ref) -> Option<&'a mut Thunk> {
    if r.proc_id() != self.proc_id {
      return None
    }

    Some(r.local_address().thunk)
  }

  fn incre_weight(&mut self, a: GlobalAddress, dw: Weight) {
    r.local_address.weight += dw;
  }

  fn delete_ref(&mut self, r: Ref) {
    let a = r.local_address();
    a.weight -= r.weight;

    if a.weight == 0 {
      let _ = unsafe { Box::from_raw(a) };
    }
  }
}
