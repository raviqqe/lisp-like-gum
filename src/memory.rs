use std::collections::BTreeMap;
use std::mem::size_of;

use address::{GlobalAddress, LocalAddress};
use processor::ProcessorId;
use reference::Ref;
use weight::Weight;
use weighted::Weighted;
use object::Object;
use thunk::{Thunk, Waits};
use libc::{malloc, free, c_void};



pub trait ThunkMemory {
  fn store(&self, Thunk) -> Ref;
  fn load(&self, Ref) -> Option<&Thunk>;
}


#[derive(Debug)]
pub struct Memory {
  proc_id: ProcessorId,
  globals: BTreeMap<GlobalAddress, Thunk>,
}

impl ThunkMemory for Memory {
  fn store(&self, t: Thunk) -> Ref {
    let w: &mut Weighted<Thunk> = malloc(size_of::<Weighted<Thunk>>()).into();
    *w = Weighted::new(t);
    w.get_ref(self.proc_id)
  }

  fn load(&self, r: Ref) -> Option<&Thunk> {
    if r.proc_id() == self.proc_id {
      Some(&**r.local_address())
    } else {
      self.globals.get(&r.global_address())
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

  pub fn get_ref(&self, a: LocalAddress) -> Ref {
    a.get_ref(self.proc_id)
  }

  pub fn store_global(&mut self, a: GlobalAddress, o: Box<Object>) {
    self.globals.insert(a, o.into());
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
      free(a.into());
    }
  }
}
