use address::{GlobalAddress, LocalAddress};
use processor::ProcessorId;
use reference::Ref;
use weight::Weight;
use weithed::Weighted;



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

  pub fn add_weight(&mut self, a: LocalAddress, dw: Weight) {
    a.add_weight(dw);
  }

  pub fn sub_weight(&mut self, a: LocalAddress, dw: Weight) {
    a.sub_weight(dw);

    if a.is_orphan() {
      let _ = unsafe { Box::from_raw(a) };
    }
  }

  pub fn set_waits(&mut self, a: LocalAddress) {
    assert!(!a.is_object());
  }

  pub fn decre_waits(&mut self, a: LocalAddress) {
    assert!(!a.is_object());
    a.waits -= 1
  }
}
