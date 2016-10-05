use std::any::Any;
use std::collections::BTreeMap;

use mpi;
use mpi::environment::Universe;
use mpi::traits::*;

use global_address::GlobalAddress;
use load_error::LoadError::*;
use load_result::LoadResult;
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
  _universe: Universe,
}

impl Memory {
  pub fn new() -> Self {
    let u = mpi::initialize().unwrap();

    Memory {
      id: MemoryId::new(u.world().rank() as u64),
      globals: BTreeMap::new(),
      serder: Serder::new(),
      transceiver: Transceiver::new(u.world()),
      _universe: u,
    }
  }

  pub fn store<T: Any>(&self, o: T) -> Ref {
    let a = LocalAddress::new(o);
    let w = Weight::new();
    a.add_weight(w);
    Ref::new(GlobalAddress::new(self.id, a), w)
  }

  pub fn load<T: Any>(&self, r: &Ref) -> LoadResult<&T> {
    if self.is_cached(r) {
      (if self.id == r.memory_id() { r.local_address() }
       else { self.globals[&r.global_address()] })
      .object::<T>().map(|p| unsafe { &*p }).ok_or(TypeMismatch)
    } else {
      self.transceiver.send(
          r.memory_id(),
          Fetch { from: self.id, local_address: r.local_address() });
      Err(NotCached)
    }
  }

  pub fn load_mut<T: Any>(&mut self, r: &Ref) -> LoadResult<&mut T> {
    if self.is_cached(r) {
      (if self.id == r.memory_id() { r.local_address() }
       else { self.globals[&r.global_address()] })
      .object_mut::<T>().map(|p| unsafe { &mut *p}).ok_or(TypeMismatch)
    } else {
      self.transceiver.send(
          r.memory_id(),
          Fetch { from: self.id, local_address: r.local_address() });
      Err(NotCached)
    }
  }

  pub fn is_cached(&self, r: &Ref) -> bool {
    r.memory_id() == self.id || self.globals.contains_key(&r.global_address())
  }

  pub fn register<T: Object + Any>(&mut self) {
    self.serder.register::<T>()
  }

  pub fn demand(&self) {
    self.transceiver.send_at_random(Demand { from: self.id });
  }

  pub fn clone_ref(&self, r: &mut Ref) -> Ref {
    let (w, dw) = r.split_weight();

    if let Some(dw) = dw {
      let m = AddWeight { local_address: r.local_address(), delta: dw };
      self.transceiver.send(r.memory_id(), m);
    }

    Ref::new(r.global_address(), w)
  }

  pub fn delete_ref(&self, r: Ref) {
    let (a, w) = r.delete();
    self.transceiver.send(
        a.memory_id(),
        SubWeight { local_address: a.local_address(), delta: w });
  }

  fn process_messages(&mut self) {
    while let Some(m) = self.transceiver.receive() {
      match m {
        Fetch { from, local_address } => {
          let o = self.serder.serialize(local_address.type_id(),
                                        local_address.unknown_object_ptr());
          let m = Resume {
            global_address: GlobalAddress::new(self.id, local_address),
            object: o,
          };

          self.transceiver.send(from, m);
        }
        Demand { from } => unimplemented!(), // send Notice
        Resume { global_address, object } => {
          self.globals.insert(global_address, self.serder.deserialize(object));
        }

        AddWeight { local_address, delta } => local_address.add_weight(delta),
        SubWeight { local_address, delta } => local_address.sub_weight(delta),
      }
    }
  }
}
