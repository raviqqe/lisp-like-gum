use std::any::{Any, TypeId};
use std::mem::size_of;
use std::ops::Drop;

use libc::{c_void, malloc, free};

use weight::Weight;



#[derive(Debug)]
pub struct Cell {
  weight: Weight,
  type_id: TypeId,
  object_ptr: usize,
}

impl Cell {
  pub fn new<T: Any>(o: T) -> Self {
    let c = Cell::uninitialized(size_of::<T>(), TypeId::of::<T>());
    unsafe { *(c.object_ptr as *mut T) = o }
    c
  }

  pub fn uninitialized(s: usize, t: TypeId) -> Cell {
    Cell {
      weight: Weight::default(),
      type_id: t,
      object_ptr: alloc_memory(s),
    }
  }

  pub fn type_id(&self) -> TypeId {
    self.type_id
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

  pub fn object<T: Any>(&self) -> Option<*const T> {
    self.object_ptr(TypeId::of::<T>()).map(|p| p as *const T)
  }

  pub fn object_mut<T: Any>(&self) -> Option<*mut T> {
    self.object_ptr(TypeId::of::<T>()).map(|p| p as *mut T)
  }

  fn object_ptr(&self, t: TypeId) -> Option<usize> {
    if self.type_id == t {
      Some(self.object_ptr)
    } else {
      None
    }
  }
}

impl Drop for Cell {
  fn drop(&mut self) {
    unsafe { free(self.object_ptr as *mut c_void) }
  }
}


fn alloc_memory(s: usize) -> usize {
  let p = unsafe { malloc(s) as usize };

  if p == 0 {
    panic!("libc::malloc() failed to allocate memory.")
  }

  p
}
