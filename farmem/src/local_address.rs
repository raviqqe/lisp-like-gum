use std::any::{Any, TypeId};
use std::convert::Into;
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
    let a = LocalAddress(unsafe { malloc(*TYPE_ID_SIZE + *WEIGHT_SIZE
                                         + size_of::<T>()) } as u64);

    unsafe {
      *(a.type_id_pointer() as *mut TypeId) = TypeId::of::<T>();
      *a.weight_mut() = Weight::default();
      *(a.object_pointer() as *mut T) = o;
    }

    a
  }

  pub unsafe fn from_size(s: usize) -> LocalAddress {
    let a = LocalAddress(malloc(s) as u64);
    *a.weight_mut() = Weight::default();
    a
  }

  pub fn type_id(&self) -> TypeId {
    unsafe { *(self.type_id_pointer() as *const TypeId) }
  }

  pub fn add_weight(&self, w: Weight) {
    *self.weight_mut() += w
  }

  pub fn sub_weight(&self, w: Weight) {
    *self.weight_mut() -= w;

    if *self.weight_mut() == Weight::default() {
      unsafe { free(self.0 as *mut c_void) }
    }
  }

  fn weight_mut(&self) -> &mut Weight {
    unsafe { &mut *(self.weight_pointer() as *mut Weight) }
  }

  fn type_id_pointer(&self) -> usize {
    self.0 as usize
  }

  fn weight_pointer(&self) -> usize {
    self.type_id_pointer() + *TYPE_ID_SIZE
  }

  pub fn object_pointer(&self) -> usize {
    self.weight_pointer() + *WEIGHT_SIZE
  }
}

impl<'a, T> Into<&'a T> for LocalAddress {
  fn into(self) -> &'a T {
    unsafe { &*(self.object_pointer() as *const T) }
  }
}

impl<'a, T> Into<&'a mut T> for LocalAddress {
  fn into(self) -> &'a mut T {
    unsafe { &mut *(self.object_pointer() as *mut T) }
  }
}
