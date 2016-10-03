use std::ops::{Deref, DerefMut, AddAssign, SubAssign};

use weight::Weight;



#[derive(Debug)]
pub struct Cell<T> {
  weight: Weight,
  object: T,
}

impl<T> Cell<T> {
  pub fn new(o: T) -> Self {
    Cell { weight: Weight::new(0), object: o }
  }

  pub fn is_orphan(&self) -> bool {
    self.weight == Weight::new(0)
  }
}

impl<T> Deref for Cell<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.object
  }
}

impl<T> DerefMut for Cell<T> {
  fn deref_mut(&mut self) -> &mut T {
    &mut self.object
  }
}

impl<T> AddAssign<Weight> for Cell<T> {
  fn add_assign(&mut self, w: Weight) {
    self.weight += w;
  }
}

impl<T> SubAssign<Weight> for Cell<T> {
  fn sub_assign(&mut self, w: Weight) {
    self.weight -= w;
  }
}
