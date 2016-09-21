use weight::Weight;
use std::ops::{Deref, DerefMut};



#[derive(Debug)]
pub struct Weighted<T> {
  value: T,
  weight: Weight,
}

impl<T> Weighted<T> {
  pub fn new(x: T) -> Self {
    Weighted { value: x, weight: Weight::default() }
  }

  pub fn add_weight(&mut self, w: Weight) {
    self.weight += w;
  }

  pub fn sub_weight(&mut self, w: Weight) {
    self.weight -= w;
  }

  pub fn is_orphan(&self) -> bool {
    self.weight == Weight::new(0)
  }
}

impl<T> Deref for Weighted<T> {
  fn deref(&self) -> &T {
    self.value
  }
}

impl<T> DerefMut for Weighted<T> {
  fn deref_mut(&mut self) -> &mut T {
    self.value
  }
}
