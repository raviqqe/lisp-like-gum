use std::any::{Any, TypeId};
use std::mem::size_of;

use alloc::{alloc_memory, free_memory};
use global_address::GlobalAddress;
use serialized_object::SerializedObject;
use type_manager::TypeManager;
use weight::Weight;

use self::LocalCell::*;



#[derive(Debug)]
pub struct WeightedLocalCell {
  weight: Weight,
  cell: LocalCell,
}

#[derive(Debug, Copy, Clone)]
pub enum LocalCell {
  Local { type_id: TypeId, object_ptr: usize },
  Moving,
  Moved(GlobalAddress),
}

impl WeightedLocalCell {
  pub fn new<T: Any>(o: T) -> Self {
    let c = WeightedLocalCell::uninitialized(size_of::<T>(), TypeId::of::<T>());
    unsafe { *(c.unknown_object_ptr() as *mut T) = o }
    c
  }

  pub fn uninitialized(s: usize, t: TypeId) -> WeightedLocalCell {
    WeightedLocalCell {
      weight: Weight::default(),
      cell: Local { type_id: t, object_ptr: alloc_memory(s) },
    }
  }

  pub fn cell(&self) -> LocalCell {
    self.cell
  }

  pub fn mark_moving(&mut self, t: &TypeManager) -> SerializedObject {
    match self.cell {
      Local { object_ptr, .. } => {
        let o = t.serialize(self);
        free_memory(object_ptr);
        self.cell = Moving;
        o
      }
      _ => panic!("The object was moved!"),
    }
  }

  pub fn mark_moved(&mut self, a: GlobalAddress) {
    match self.cell {
      Moving => self.cell = Moved(a),
      _ => panic!("The cell should be Moved!"),
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
    match self.cell {
      Local { type_id, .. } => type_id,
      _ => panic!("The object was moved!"),
    }
  }

  pub fn unknown_object_ptr(&self) -> usize {
    match self.cell {
      Local { object_ptr, .. } => object_ptr,
      _ => panic!("The object was moved!"),
    }
  }

  // pub fn object<T: Any>(&self) -> Option<*const T> {
  //   self.object_ptr(TypeId::of::<T>()).map(|p| p as *const T)
  // }

  // pub fn object_mut<T: Any>(&self) -> Option<*mut T> {
  //   self.object_ptr(TypeId::of::<T>()).map(|p| p as *mut T)
  // }

  // fn object_ptr(&self, t: TypeId) -> Option<usize> {
  //   match self.cell {
  //     Local { type_id, object_ptr } => if type_id == t {
  //       Some(object_ptr)
  //     } else {
  //       None
  //     },
  //     _ => panic!("The object was moved!"),
  //   }
  // }
}

impl Drop for WeightedLocalCell {
  fn drop(&mut self) {
    if let Local { object_ptr, .. } = self.cell {
      free_memory(object_ptr)
    }
  }
}
