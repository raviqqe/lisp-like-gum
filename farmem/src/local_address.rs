use std::convert::{From, Into};



#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct LocalAddress(u64);

impl From<u64> for LocalAddress {
  fn from(n: u64) -> LocalAddress {
    LocalAddress(n)
  }
}

impl Into<u64> for LocalAddress {
  fn into(self) -> u64 {
    self.0
  }
}

// impl Into<*mut c_void> for LocalAddress {
//   fn into(self) -> *mut c_void {
//     self.0 as *mut c_void
//   }
// }

// impl<'a, T> From<&'a Cell<T>> for LocalAddress {
//   fn from(w: &'a Cell<T>) -> LocalAddress {
//     LocalAddress(w as *const Cell<T> as u64)
//   }
// }

// impl<'a, T> From<&'a mut Cell<T>> for LocalAddress {
//   fn from(w: &'a mut Cell<T>) -> LocalAddress {
//     LocalAddress(w as *const Cell<T> as u64)
//   }
// }

// impl<'a, T> From<LocalAddress> for &'a Cell<T> {
//   fn from(a: LocalAddress) -> &'a Cell<T> {
//     let p: u64 = a.into();
//     unsafe { &*(p as *const Cell<T>) }
//   }
// }

// impl<'a, T> From<LocalAddress> for &'a mut Cell<T> {
//   fn from(a: LocalAddress) -> &'a mut Cell<T> {
//     let p: u64 = a.into();
//     unsafe { &mut *(p as *mut Cell<T>) }
//   }
// }

// impl<T> Deref for LocalAddress {
//   type Target = Cell<T>;

//   fn deref(&self) -> &Self::Target {
//     let p: u64 = (*self).into();
//     unsafe { &*(p as *const Self::Target) }
//   }
// }

// impl<T> DerefMut for LocalAddress {
//   fn deref_mut(&mut self) -> &mut Cell<T> {
//     let p: u64 = (*self).into();
//     unsafe { &mut *(p as *mut Cell<T>) }
//   }
// }
