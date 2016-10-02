use std::any::Any;
use std::collections::BTreeMap;
use std::mem::size_of;

use libc::{malloc, free};

use global_address::GlobalAddress;
use object::Object;
use reference::Ref;
use weighted::Weighted;



#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct MemoryId(u64);

#[derive(Debug)]
pub struct Memory {
  id: MemoryId,
  globals: BTreeMap<GlobalAddress, Box<Any>>,
}

impl Memory {
  pub fn new(i: MemoryId) -> Self {
    Memory {
      id: i,
      globals: BTreeMap::new(),
    }
  }

  fn store<T: Object>(&self, o: T) -> Ref {
    let w = unsafe { &mut *(malloc(size_of::<Weighted<T>>())
                            as *mut Weighted<T>) };
    *w = Weighted::new(o);

    w.into()
  }

  // fn load(&self, r: Ref) -> Option<&Thunk> {
  //   if r.proc_id() == self.proc_id {
  //     let w: &Weighted<Thunk> = r.local_address().into();
  //     Some(w.deref())
  //   } else {
  //     self.globals.get(&r.global_address())
  //   }
  // }

  // pub fn get_ref(&self, mut a: LocalAddress) -> Ref {
  //   a.get_ref(self.proc_id)
  // }

  // pub fn store_global(&mut self, a: GlobalAddress, o: Box<Object>) {
  //   self.globals.insert(a, o.into());
  // }

  // pub fn load_mut(&self, r: Ref) -> Option<&mut Thunk> {
  //   let a = r.local_address();

  //   if r.proc_id() == self.proc_id {
  //     let w: &mut Weighted<Thunk> = a.into();
  //     Some(w.deref_mut())
  //   } else {
  //     None
  //   }
  // }

  // pub fn add_weight(&self, mut a: LocalAddress, dw: Weight) {
  //   a.add_weight(dw);
  // }

  // pub fn sub_weight(&self, mut a: LocalAddress, dw: Weight) {
  //   a.sub_weight(dw);

  //   if a.is_orphan() {
  //     unsafe { free(a.into()) }
  //   }
  // }
}
