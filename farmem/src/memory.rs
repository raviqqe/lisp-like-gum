use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::mem::size_of;

use libc::malloc;

use cell::Cell;
use global_address::GlobalAddress;
use reference::Ref;



lazy_static!{
  static ref TYPE_ID_SIZE: usize = ((size_of::<TypeId>() + 7) % 8) * 8;
}

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

  pub fn store<T: Any>(&self, o: T) -> Ref {
    unsafe {
      let p = malloc(*TYPE_ID_SIZE + size_of::<Cell<T>>());
      *(p as *mut TypeId) = TypeId::of::<T>();
      let c = (p as usize + *TYPE_ID_SIZE) as *mut Cell<T>;
      *c = Cell::new(o);
      (&mut *c).into()
    }
  }

  pub fn load<T: Any>(&self, r: &Ref) -> Option<&T> {
    let t = unsafe { *((r.local_address().into(): u64 - *TYPE_ID_SIZE as u64)
                       as *const TypeId) };

    if r.memory_id() == self.id && TypeId::of::<T>() == t {
      Some(unsafe { &&*(r.local_address().into(): u64 as *const Cell<T>) })
    } else {
      self.globals[&r.global_address()].downcast_ref()
    }
  }

  // pub fn load_mut(&self, r: Ref) -> Option<&mut Thunk> {
  //   let a = r.local_address();

  //   if r.proc_id() == self.proc_id {
  //     let w: &mut Cell<Thunk> = r.local_address().into();
  //     Some(w.deref_mut())
  //   } else {
  //     self.globals.get_mut(&r.global_address())
  //   }
  // }

  // pub fn get_ref(&self, mut a: LocalAddress) -> Ref {
  //   a.get_ref(self.proc_id)
  // }

  // pub fn store_global(&mut self, a: GlobalAddress, o: Box<Object>) {
  //   self.globals.insert(a, o.into());
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
