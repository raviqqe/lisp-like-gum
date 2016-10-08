use std::any::{Any, TypeId};
use std::collections::{BTreeMap, VecDeque};

use mpi;
use mpi::environment::Universe;
use mpi::traits::*;

use cell::{Cell, CellState};
use demand;
use demand::FriendlyDemand;
use global_address::GlobalAddress;
use load_error::LoadError::*;
use load_result::LoadResult;
use local_cells::LocalCells;
use memory_id::MemoryId;
use message::Message::*;
use notice::Notice;
use object::Object;
use reference::{Ref, FriendlyRef};
use type_manager::TypeManager;
use transceiver::Transceiver;
use weight::Weight;



pub struct Memory {
  id: MemoryId,
  locals: LocalCells,
  globals: BTreeMap<GlobalAddress, Cell>,
  type_manager: TypeManager,
  transceiver: Transceiver,
  notices: VecDeque<Notice>,
  _universe: Universe,
}

impl Memory {
  pub fn new() -> Self {
    let u = mpi::initialize().unwrap();

    Memory {
      id: MemoryId::new(u.world().rank() as u64),
      locals: LocalCells::new(),
      globals: BTreeMap::new(),
      type_manager: TypeManager::new(),
      transceiver: Transceiver::new(u.world()),
      notices: VecDeque::new(),
      _universe: u,
    }
  }

  pub fn store<T: Any>(&mut self, o: T) -> Ref {
    let i = self.locals.store(o);
    let w = Weight::new();
    self.locals[i].add_weight(w);
    Ref::new(GlobalAddress::new(self.id, i), w)
  }

  pub fn load<T: Any>(&mut self, r: &Ref) -> LoadResult<&T> {
    if self.is_cached(r) {
      match (if self.id == r.memory_id() { &self.locals[r.local_id()] }
             else { &self.globals[&r.global_address()] }).state() {
        CellState::Local { type_id, object_ptr } => {
          if TypeId::of::<T>() == type_id {
            Ok(unsafe { &*(object_ptr as *const T) })
          } else {
            Err(TypeMismatch)
          }
        }
        CellState::Moving => unimplemented!(),
        CellState::Moved(_) => unimplemented!(),
      }
    } else {
      self.transceiver.send(
          r.memory_id(),
          Fetch { from: self.id, local_id: r.local_id() });
      Err(NotCached)
    }
  }

  pub fn load_mut<T: Any>(&mut self, r: &Ref) -> LoadResult<&mut T> {
    if self.is_cached(r) {
      match (if self.id == r.memory_id() { &self.locals[r.local_id()] }
             else { &self.globals[&r.global_address()] }).state() {
        CellState::Local { type_id, object_ptr } => {
          if TypeId::of::<T>() == type_id {
            Ok(unsafe { &mut *(object_ptr as *mut T) })
          } else {
            Err(TypeMismatch)
          }
        }
        CellState::Moving => unimplemented!(),
        CellState::Moved(_) => unimplemented!(),
      }
    } else {
      self.transceiver.send(
          r.memory_id(),
          Fetch { from: self.id, local_id: r.local_id() });
      Err(NotCached)
    }
  }

  pub fn is_cached(&mut self, r: &Ref) -> bool {
    self.process_messages();

    if r.memory_id() == self.id
        || self.globals.contains_key(&r.global_address()) {
      true
    } else {
      false
    }
  }

  pub fn register<T: Object + Any>(&mut self) {
    self.type_manager.register::<T>()
  }

  pub fn demand(&self) {
    self.transceiver.send_at_random(Demand { from: self.id });
  }

  pub fn feed(&self, d: demand::Demand, r: Ref) {
    let i = r.local_id();
    let m = Move {
      reference: r,
      object: self.type_manager.serialize(&self.locals[i]),
    };

    self.transceiver.send(d.memory_id(), m);
  }

  pub fn clone_ref(&self, r: &mut Ref) -> Ref {
    let (w, dw) = r.split_weight();

    if let Some(dw) = dw {
      let m = AddWeight { local_id: r.local_id(), delta: dw };
      self.transceiver.send(r.memory_id(), m);
    }

    Ref::new(r.global_address(), w)
  }

  pub fn delete_ref(&self, r: Ref) {
    let (a, w) = r.delete();
    self.transceiver.send(
        a.memory_id(),
        SubWeight { local_id: a.local_id(), delta: w });
  }

  fn process_messages(&mut self) {
    while let Some(m) = self.transceiver.receive() {
      match m {
        Fetch { from, local_id } => {
          let o = self.type_manager.serialize(&self.locals[local_id]);
          let m = Resume {
            global_address: GlobalAddress::new(self.id, local_id),
            object: o,
          };

          self.transceiver.send(from, m);
        }
        Resume { global_address, object } => {
          self.globals.insert(global_address,
                              self.type_manager.deserialize(object));
        }

        Demand { from } => {
          self.notices.push_back(Notice::Demand(demand::Demand::new(from)))
        }
        Move { reference, object } => {
          self.globals.insert(reference.global_address(),
                              self.type_manager.deserialize(object));
          self.notices.push_back(Notice::Feed(reference));
        }
        Moved { from, to } => unimplemented!(),

        AddWeight { local_id, delta } => {
          self.locals[local_id].add_weight(delta)
        }
        SubWeight { local_id, delta } => {
          self.locals[local_id].sub_weight(delta);

          if self.locals[local_id].is_orphan() {
            for r in self.type_manager.extract_refs(&self.locals[local_id]) {
              self.delete_ref(r);
            }

            self.locals.delete(local_id);
          }
        }
      }
    }
  }

  pub fn receive_notice(&mut self) -> Option<Notice> {
    self.process_messages();
    self.notices.pop_front()
  }
}
