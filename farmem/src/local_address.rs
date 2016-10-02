use std::ops::{Deref, DerefMut};
use std::convert::{From, Into};
// use weighted::Weighted;



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

// impl<'a, T> From<&'a Weighted<T>> for LocalAddress {
//   fn from(w: &'a Weighted<T>) -> LocalAddress {
//     LocalAddress(w as *const Weighted<T> as u64)
//   }
// }

// impl<'a, T> From<&'a mut Weighted<T>> for LocalAddress {
//   fn from(w: &'a mut Weighted<T>) -> LocalAddress {
//     LocalAddress(w as *const Weighted<T> as u64)
//   }
// }

// impl<'a, T> From<LocalAddress> for &'a Weighted<T> {
//   fn from(a: LocalAddress) -> &'a Weighted<T> {
//     let p: u64 = a.into();
//     unsafe { &*(p as *const Weighted<T>) }
//   }
// }

// impl<'a, T> From<LocalAddress> for &'a mut Weighted<T> {
//   fn from(a: LocalAddress) -> &'a mut Weighted<T> {
//     let p: u64 = a.into();
//     unsafe { &mut *(p as *mut Weighted<T>) }
//   }
// }

// impl<T> Deref for LocalAddress {
//   type Target = Weighted<T>;

//   fn deref(&self) -> &Self::Target {
//     let p: u64 = (*self).into();
//     unsafe { &*(p as *const Self::Target) }
//   }
// }

// impl<T> DerefMut for LocalAddress {
//   fn deref_mut(&mut self) -> &mut Weighted<T> {
//     let p: u64 = (*self).into();
//     unsafe { &mut *(p as *mut Weighted<T>) }
//   }
// }
