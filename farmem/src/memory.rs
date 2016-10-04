use std::any::{Any, TypeId};
use std::collections::BTreeMap;

use mpi;
use mpi::environment::Universe;
use mpi::topology::SystemCommunicator;
use mpi::traits::*;

use global_address::GlobalAddress;
use local_address::LocalAddress;
use memory_id::MemoryId;
use object::Object;
use reference::Ref;
use serder::Serder;
use transceiver::Transceiver;
use weight::Weight;



pub struct Memory {
  id: MemoryId,
  globals: BTreeMap<GlobalAddress, Box<Any>>,
  serder: Serder,
  transceiver: Transceiver,
  world: SystemCommunicator,
  _universe: Universe,
}

impl Memory {
  pub fn new() -> Self {
    let u = mpi::initialize().unwrap();
    let w = u.world();

    Memory {
      id: MemoryId::new(u.world().rank() as u64),
      globals: BTreeMap::new(),
      serder: Serder::new(),
      transceiver: Transceiver::new(w),
      world: w,
      _universe: u,
    }
  }

  pub fn store<T: Any>(&self, o: T) -> Ref {
    let a = LocalAddress::new(o);
    let w = Weight::new();
    a.add_weight(w);
    Ref::new(GlobalAddress::new(self.id, a), w)
  }

  pub fn load<T: Any>(&self, r: &Ref) -> Option<&T> {
    if self.check_id_and_type::<T>(r) {
      Some(r.local_address().into())
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
      Some(r.local_address().into())
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
    r.memory_id() == self.id
        && TypeId::of::<T>() == r.local_address().type_id()
  }

  pub fn register<T: Object + Any>(&mut self) {
    self.serder.register::<T>()
  }

  fn process_messages(&self) {
    while let Some(m) = self.transceiver.receive() {
      match m {
        _ => unimplemented!(),
      }
    }
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
