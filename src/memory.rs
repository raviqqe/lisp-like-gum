use std::collections::BTreeMap;
use std::mem::size_of;

use address::{GlobalAddress, LocalAddress};
use processor::ProcessorId;
use reference::Ref;
use weight::Weight;
use weighted::Weighted;
use thunk::{Thunk, Waits};
use libc::{malloc, free, c_void};



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
    let p = malloc(size_of::<Weighted<Thunk>>());
    *p = Weighted::new(t);
    Ref::new(GlobalAddress::new(self.proc_id, p.into()));
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

  pub fn add_weight(&self, a: LocalAddress, dw: Weight) {
    a.add_weight(dw);
  }

  pub fn sub_weight(&self, a: LocalAddress, dw: Weight) {
    a.sub_weight(dw);

    if a.is_orphan() {
      free(a as *const c_void);
    }
  }
}
