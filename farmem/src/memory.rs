use std::any::{Any, TypeId};
use std::collections::VecDeque;

use mpi;
use mpi::environment::Universe;
use mpi::traits::*;

use demand;
use demand::FriendlyDemand;
use global_cell::GlobalCell;
use global_cells::GlobalCells;
use global_address::GlobalAddress;
use load_error::LoadError::*;
use load_result::LoadResult;
use local_cell::LocalCell;
use local_cells::LocalCells;
use memory_id::MemoryId;
use message::Message::*;
use notice::Notice;
use object::Object;
use reference::{Ref, FriendlyRef};
use type_manager::TypeManager;
use transceiver::Transceiver;
use weight::Weight;



macro_rules! convert_object_ptr {
  ($T:ty, $t:expr, $p:expr) => {
    if TypeId::of::<$T>() == $t {
      Ok(unsafe { &*($p as *const $T) })
    } else {
      Err(TypeMismatch)
    }
  }
}


pub struct Memory {
  id: MemoryId,
  locals: LocalCells,
  globals: GlobalCells,
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
      globals: GlobalCells::new(),
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
    self.process_messages();

    let mut a = r.global_address();

    loop {
      if a.memory_id() == self.id {
        match self.locals[a.local_id()].cell() {
          LocalCell::Local { type_id, object_ptr }
              => return convert_object_ptr!(T, type_id, object_ptr),
          LocalCell::Moving => return Err(NotCached),
          LocalCell::Moved(new_a) => {
            assert!(new_a.memory_id() != self.id);
            a = new_a;
          }
        }
      } else {
        match self.globals.get(a) {
          Some(&GlobalCell::Local { type_id, object_ptr })
                => return convert_object_ptr!(T, type_id, object_ptr),
          Some(&GlobalCell::Moved(new_a)) => a = new_a,
          None => return Err(NotCached),
        }
      }
    }
  }

  pub fn load_mut<T: Any>(&mut self, r: &Ref) -> LoadResult<&mut T> {
    self.process_messages();

    let mut a = r.global_address();

    if r.memory_id() != self.id {
      return Err(NotCached)
    }

    loop {
      if a.memory_id() == self.id {
        match self.locals[a.local_id()].cell() {
          LocalCell::Local { type_id, object_ptr } => {
            return if TypeId::of::<T>() == type_id {
              Ok(unsafe { &mut *(object_ptr as *mut T) })
            } else {
              Err(TypeMismatch)
            }
          }
          LocalCell::Moving => return Err(NotCached),
          LocalCell::Moved(new_a) => {
            assert!(new_a.memory_id() != self.id);
            a = new_a;
          }
        }
      } else {
        match self.globals.get(a) {
          Some(&GlobalCell::Local { .. }) | None => return Err(NotCached),
          Some(&GlobalCell::Moved(new_a)) => a = new_a,
        }
      }
    }
  }

  pub fn is_cached(&mut self, r: &Ref) -> bool {
    self.process_messages();

    let mut a = r.global_address();

    loop {
      if a.memory_id() == self.id {
        match self.locals[a.local_id()].cell() {
          LocalCell::Local { .. } => return true,
          LocalCell::Moving => return false,
          LocalCell::Moved(new_a) => {
            assert!(new_a.memory_id() != self.id);
            a = new_a;
          }
        }
      } else {
        match self.globals.get(a) {
          Some(&GlobalCell::Local { .. }) => return true,
          Some(&GlobalCell::Moved(new_a)) => a = new_a,
          None => return false,
        }
      }
    }
  }

  pub fn register<T: Object + Any>(&mut self) {
    self.type_manager.register::<T>()
  }

  pub fn demand(&self) {
    self.transceiver.send_at_random(Demand { from: self.id });
  }

  pub fn feed(&mut self, d: demand::Demand, r: Ref) {
    let o = self.locals[r.local_id()].mark_moving(&self.type_manager);
    self.transceiver.send(d.memory_id(), Move { reference: r, object: o });
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
          self.globals.store(global_address,
                             self.type_manager.deserialize(object));
        }
        Moved { from, to } => self.globals.store(from, GlobalCell::Moved(to)),

        Demand { from } => {
          self.notices.push_back(Notice::Demand(demand::Demand::new(from)))
        }
        Move { reference, object } => {
          self.globals.store(reference.global_address(),
                             self.type_manager.deserialize(object));
          self.notices.push_back(Notice::Feed(reference));
        }
        Ack { from, to } => self.locals[from].mark_moved(to),

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
