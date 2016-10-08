use std::any::{Any, TypeId};
use std::mem::size_of;

use libc::{c_void, malloc, free};

use global_address::GlobalAddress;
use serialized_object::SerializedObject;
use type_manager::TypeManager;
use weight::Weight;

use self::CellState::*;



#[derive(Debug)]
pub struct Cell {
  weight: Weight,
  state: CellState,
}

#[derive(Debug, Copy, Clone)]
pub enum CellState {
  Local { type_id: TypeId, object_ptr: usize },
  Moving,
  Moved(GlobalAddress),
}

impl Cell {
  pub fn new<T: Any>(o: T) -> Self {
    let c = Cell::uninitialized(size_of::<T>(), TypeId::of::<T>());
    unsafe { *(c.unknown_object_ptr() as *mut T) = o }
    c
  }

  pub fn uninitialized(s: usize, t: TypeId) -> Cell {
    Cell {
      weight: Weight::default(),
      state: Local { type_id: t, object_ptr: alloc_memory(s) },
    }
  }

  pub fn state(&self) -> CellState {
    self.state
  }

  pub fn mark_moving(&mut self, t: &TypeManager) -> SerializedObject {
    match self.state {
      Local { object_ptr, .. } => {
        let o = t.serialize(self);
        unsafe { free(object_ptr as *mut c_void) }
        self.state = Moving;
        o
      }
      _ => panic!("The object was moved!"),
    }
  }

  pub fn mark_moved(&mut self, a: GlobalAddress) {
    match self.state {
      Moving => self.state = Moved(a),
      _ => panic!("The state should be Moved!"),
    }
  }

  pub fn add_weight(&mut self, w: Weight) {
    self.weight += w
  }

  pub fn sub_weight(&mut self, w: Weight) {
    self.weight -= w
  }

  pub fn is_orphan(&self) -> bool {
    self.weight == Weight::default()
  }

  pub fn type_id(&self) -> TypeId {
    match self.state {
      Local { type_id, .. } => type_id,
      _ => panic!("The object was moved!"),
    }
  }

  pub fn unknown_object_ptr(&self) -> usize {
    match self.state {
      Local { object_ptr, .. } => object_ptr,
      _ => panic!("The object was moved!"),
    }
  }

  pub fn object<T: Any>(&self) -> Option<*const T> {
    self.object_ptr(TypeId::of::<T>()).map(|p| p as *const T)
  }

  pub fn object_mut<T: Any>(&self) -> Option<*mut T> {
    self.object_ptr(TypeId::of::<T>()).map(|p| p as *mut T)
  }

  fn object_ptr(&self, t: TypeId) -> Option<usize> {
    match self.state {
      Local { type_id, object_ptr } => if type_id == t {
        Some(object_ptr)
      } else {
        None
      },
      _ => panic!("The object was moved!"),
    }
  }
}

impl Drop for Cell {
  fn drop(&mut self) {
    if let Local { object_ptr, .. } = self.state {
      unsafe { free(object_ptr as *mut c_void) }
    }
  }
}


fn alloc_memory(s: usize) -> usize {
  let p = unsafe { malloc(s) as usize };

  if p == 0 {
    panic!("libc::malloc() failed to allocate memory.")
  }

  p
}
