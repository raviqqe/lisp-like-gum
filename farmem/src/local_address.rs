use std::any::{Any, TypeId};
use std::mem::size_of;

use libc::{c_void, malloc, free};

use weight::Weight;



lazy_static!{
  static ref TYPE_ID_SIZE: usize = max_size_of::<TypeId>();
  static ref WEIGHT_SIZE: usize = max_size_of::<Weight>();
}

const MAX_ALIGN: usize = 8;

fn max_size_of<T>() -> usize {
  ((size_of::<T>() + MAX_ALIGN - 1) % MAX_ALIGN) * MAX_ALIGN
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct LocalAddress(u64);

impl LocalAddress {
  pub fn new<T: Any>(o: T) -> LocalAddress {
    let a = LocalAddress(alloc_memory(size_of::<T>()));

    *a.type_id_mut() = TypeId::of::<T>();
    *a.weight_mut() = Weight::default();
    unsafe { *a.object_mut::<T>().unwrap() = o }

    a
  }

  pub fn uninitialized(s: usize, t: TypeId) -> LocalAddress {
    let a = LocalAddress(alloc_memory(s));
    *a.type_id_mut() = t;
    *a.weight_mut() = Weight::default();
    a
  }

  pub fn type_id(&self) -> TypeId {
    *self.type_id_mut()
  }

  pub fn add_weight(&self, w: Weight) {
    *self.weight_mut() += w
  }

  pub fn sub_weight(&self, w: Weight) {
    *self.weight_mut() -= w;
  }

  pub fn is_orphan(&self) -> bool {
    *self.weight_mut() == Weight::default()
  }

  pub fn free(&self) {
    unsafe { free(self.0 as *mut c_void) }
  }

  fn type_id_mut(&self) -> &mut TypeId {
    unsafe { &mut *(self.type_id_ptr() as *mut TypeId) }
  }

  fn weight_mut(&self) -> &mut Weight {
    unsafe { &mut *(self.weight_ptr() as *mut Weight) }
  }

  fn type_id_ptr(&self) -> usize {
    self.0 as usize
  }

  fn weight_ptr(&self) -> usize {
    self.type_id_ptr() + *TYPE_ID_SIZE
  }

  pub fn unknown_object_ptr(&self) -> usize {
    self.weight_ptr() + *WEIGHT_SIZE
  }

  pub fn object<T: Any>(&self) -> Option<*const T> {
    self.object_ptr(TypeId::of::<T>()).map(|p| p as *const T)
  }

  pub fn object_mut<T: Any>(&self) -> Option<*mut T> {
    self.object_ptr(TypeId::of::<T>()).map(|p| p as *mut T)
  }

  fn object_ptr(&self, t: TypeId) -> Option<usize> {
    if self.type_id() == t {
      Some(self.unknown_object_ptr())
    } else {
      None
    }
  }
}


fn alloc_memory(s: usize) -> u64 {
  let p = unsafe { malloc(*TYPE_ID_SIZE + *WEIGHT_SIZE + s) as u64 };

  if p == 0 {
    panic!("libc::malloc() failed to allocate memory.")
  }

  p
}
