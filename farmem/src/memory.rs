use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::mem::size_of;

use libc::malloc;
use mpi;
use mpi::environment::Universe;
use mpi::traits::*;

use cell::Cell;
use consts::TYPE_ID_SIZE;
use global_address::GlobalAddress;
use reference::Ref;



#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct MemoryId(u64);

impl MemoryId {
  fn new(i: u64) -> Self {
    MemoryId(i)
  }
}

pub struct Memory {
  id: MemoryId,
  globals: BTreeMap<GlobalAddress, Box<Any>>,
  _universe: Universe,
}

impl Memory {
  pub fn new() -> Self {
    let u = mpi::initialize().unwrap();

    Memory {
      id: MemoryId::new(u.world().rank() as u64),
      globals: BTreeMap::new(),
      _universe: u,
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
    if self.check_id_and_type::<T>(r) {
      Some(unsafe { &&*(r.local_address().into(): u64 as *const Cell<T>) })
    } else {
      match self.globals.get(&r.global_address()) {
        Some(b) => b.downcast_ref(),
        None => {
          unimplemented!() // self.send_fetch()
        }
      }
    }
  }

  pub fn load_mut<T: Any>(&mut self, r: &Ref) -> Option<&mut T> {
    if self.check_id_and_type::<T>(r) {
      let o: &mut T = unsafe { &mut &mut *(r.local_address().into(): u64
                                           as *mut Cell<T>) };
      Some(unsafe {&mut *(o as *mut T)})
    } else {
      match self.globals.get_mut(&r.global_address()) {
        Some(b) => b.downcast_mut(),
        None => {
          unimplemented!() // self.send_fetch()
        }
      }
    }
  }

  fn check_id_and_type<T: Any>(&self, r: &Ref) -> bool {
    r.memory_id() == self.id && TypeId::of::<T>() == r.local_address().into()
  }

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
