use address::{GlobalAddress, LocalAddress};
use processor::ProcessorId;
use reference::Ref;
use weight::Weight;
use weithed::Weighted;
use thunk::{Thunk, Waits};



pub trait ThunkMemory {
  fn store(&self, Thunk) -> Ref;
  fn load<'a>(&self, Ref) -> Option<&'a mut Thunk>;
}


#[derive(Debug)]
pub struct Memory {
  proc_id: ProcessorId,
}

impl ThunkMemory for Memory {
  fn store(&self, t: Thunk) -> Ref {
    Ref::new(self.proc_id, Box::into_raw(Box::new(Weighted::new(t))));
  }

  fn load<'a>(&self, r: Ref) -> Option<&'a mut Thunk> {
    if r.proc_id() != self.proc_id {
      return None
    }

    Some(r.local_address().value)
  }
}

impl Memory {
  pub fn new(id: ProcessorId) -> Self {
    Memory { proc_id: id }
  }
}
