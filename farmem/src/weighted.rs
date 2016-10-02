use std::ops::{Deref, DerefMut, AddAssign, SubAssign};

use global_address::GlobalAddress;
use reference::Ref;
use weight::Weight;



#[derive(Debug)]
pub struct Weighted<T> {
  weight: Weight,
  object: T,
}

impl<T> Weighted<T> {
  pub fn new(o: T, w: Weight) -> Self {
    Weighted { weight: w, object: o }
  }

  pub fn is_orphan(&self) -> bool {
    self.weight == Weight::new(0)
  }
}

impl<T> Deref for Weighted<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.object
  }
}

impl<T> DerefMut for Weighted<T> {
  fn deref_mut(&mut self) -> &mut T {
    &mut self.object
  }
}

impl<T> AddAssign<Weight> for Weighted<T> {
  fn add_assign(&mut self, w: Weight) {
    self.weight += w;
  }
}

impl<T> SubAssign<Weight> for Weighted<T> {
  fn sub_assign(&mut self, w: Weight) {
    self.weight -= w;
  }
}

impl<'a, T> Into<Ref> for &'a mut Weighted<T> {
  fn into(self) -> Ref {
    let w = Weight::default();
    *self += w;

    let a = GlobalAddress::new(unimplemented!() /* MEMORY.id() */,
                               (self as *mut Weighted<T> as u64).into());

    Ref::new(a, w)
  }
}
