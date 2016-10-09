use std::any::{Any, TypeId};

use alloc::{alloc_memory, free_memory};
use global_address::GlobalAddress;

use self::GlobalCell::*;



#[derive(Debug)]
pub enum GlobalCell {
  Local { type_id: TypeId, object_ptr: usize },
  Moved(GlobalAddress),
}

impl GlobalCell {
  pub fn uninitialized(s: usize, t: TypeId) -> Self {
    Local {
      type_id: t,
      object_ptr: alloc_memory(s),
    }
  }

  // pub fn type_id(&self) -> TypeId {
  //   match self.cell {
  //     Local { type_id, .. } => type_id,
  //     _ => panic!("The object was moved!"),
  //   }
  // }

  pub fn unknown_object_ptr(&self) -> usize {
    match *self {
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
    match *self {
      Local { type_id, object_ptr }
          => if type_id == t { Some(object_ptr) } else { None },
      _ => panic!("The object was moved!"),
    }
  }
}

impl Drop for GlobalCell {
  fn drop(&mut self) {
    if let Local { object_ptr, .. } = *self {
      free_memory(object_ptr)
    }
  }
}
