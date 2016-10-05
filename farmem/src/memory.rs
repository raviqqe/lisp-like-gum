use std::any::{Any, TypeId};
use std::collections::BTreeMap;

use mpi;
use mpi::environment::Universe;
use mpi::topology::SystemCommunicator;
use mpi::traits::*;

use global_address::GlobalAddress;
use local_address::LocalAddress;
use memory_id::MemoryId;
use message::Message::*;
use object::Object;
use reference::Ref;
use serder::Serder;
use transceiver::Transceiver;
use weight::Weight;



pub struct Memory {
  id: MemoryId,
  globals: BTreeMap<GlobalAddress, LocalAddress>,
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
        Some(a) => Some((*a).into()),
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
        Some(a) => Some((*a).into()),
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

  fn clone_ref(&self, r: &mut Ref) -> Ref {
    let (w, dw) = r.split_weight();

    if let Some(dw) = dw {
      let m = AddWeight { local_address: r.local_address(), delta: dw };
      self.transceiver.send(r.memory_id(), m);
    }

    Ref::new(r.global_address(), w)
  }

  fn delete_ref(&self, r: Ref) {
    // self.transceiver.send(r.memory_id(), SubWeight { local_address: r.local_address(), weight: r.weight() })
    unimplemented!()
  }

  fn process_messages(&mut self) {
    while let Some(m) = self.transceiver.receive() {
      match m {
        Fetch { from, local_address } => unimplemented!(),
        Demand { from } => unimplemented!(),
        Resume { global_address, object } => {
          self.globals.insert(global_address, self.serder.deserialize(object));
        }

        AddWeight { local_address, delta } => local_address.add_weight(delta),
        SubWeight { local_address, delta } => local_address.sub_weight(delta),
      }
    }
  }

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
