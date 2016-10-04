use std::any::{Any, TypeId};
use std::convert::Into;
use std::mem::size_of;

use libc::{c_void, malloc, free};

use cell::Cell;



lazy_static!{
  static ref TYPE_ID_SIZE: usize = ((size_of::<TypeId>() + 7) % 8) * 8;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct LocalAddress(u64);

impl LocalAddress {
  pub fn new<T: Any>(o: T) -> LocalAddress {
    let p = unsafe { malloc(*TYPE_ID_SIZE + size_of::<Cell<T>>()) };
    unsafe { *(p as *mut TypeId) = TypeId::of::<T>() };
    let c = unsafe { &mut *((p as usize + *TYPE_ID_SIZE) as *mut Cell<T>) };
    *c = Cell::new(o);
    LocalAddress(p as u64)
  }

  pub unsafe fn from_size(s: usize) -> LocalAddress {
    LocalAddress(malloc(s) as u64)
  }

  pub fn free(&self) {
    unsafe { free(self.0 as *mut c_void) }
  }

  fn cell_pointer(&self) -> usize {
    self.0 as usize + *TYPE_ID_SIZE
  }
}

impl<'a, T> Into<&'a Cell<T>> for LocalAddress {
  fn into(self) -> &'a Cell<T> {
    unsafe { &*(self.cell_pointer() as *const Cell<T>) }
  }
}

impl<'a, T> Into<&'a mut Cell<T>> for LocalAddress {
  fn into(self) -> &'a mut Cell<T> {
    unsafe { &mut *(self.cell_pointer() as *mut Cell<T>) }
  }
}

impl Into<usize> for LocalAddress {
  fn into(self) -> usize {
    self.cell_pointer()
  }
}

impl Into<TypeId> for LocalAddress {
  fn into(self) -> TypeId {
    unsafe { *(self.0 as *const TypeId) }
  }
}
