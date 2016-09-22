use std::collections::BTreeMap;

use address::{GlobalAddress, LocalAddress};
use processor::ProcessorId;
use reference::Ref;
use weight::Weight;
use weighted::Weighted;
use thunk::{Thunk, Waits};



pub trait ThunkMemory {
  fn store(&self, Thunk) -> Ref;
  fn load<'a>(&self, Ref) -> Option<&'a Thunk>;
}


#[derive(Debug)]
pub struct Memory {
  proc_id: ProcessorId,
  globals: BTreeMap<GlobalAddress, Thunk>,
}

impl ThunkMemory for Memory {
  fn store(&self, t: Thunk) -> Ref {
    Ref::new(self.proc_id, Box::into_raw(Box::new(Weighted::new(t))));
  }

  fn load<'a>(&self, r: Ref) -> Option<&'a Thunk> {
    if r.proc_id() == self.proc_id {
      Some(&*r.local_address())
    } else if self.globals.contains_key(r.global_address) {
      Some(self.globals[r.global_address])
    } else {
      None
    }
  }
}

impl Memory {
  pub fn new(id: ProcessorId) -> Self {
    Memory {
      proc_id: id,
      globals: BTreeMap::new(),
    }
  }

  pub fn store_global(&mut self, a: GlobalAddress, t: Thunk) {
    self.globals.insert(a, t);
  }

  pub fn load_mut<'a>(&self, r: Ref) -> Option<&'a mut Thunk> {
    if r.proc_id() == self.proc_id {
      Some(&mut *r.local_address())
    } else {
      None
    }
  }
}
