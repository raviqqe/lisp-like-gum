use std::any::{Any, TypeId};
use std::ops::{Deref, DerefMut, AddAssign, SubAssign};

use global_address::GlobalAddress;
use reference::Ref;
use typed::Typed;
use weight::Weight;



#[derive(Debug)]
pub struct Cell<T> {
  weight: Weight,
  type_id: TypeId,
  object: T,
}

impl<T: Any> Cell<T> {
  pub fn new(o: T) -> Self {
    Cell { weight: Weight::new(0), type_id: TypeId::of::<T>(), object: o }
  }

  pub fn is_orphan(&self) -> bool {
    self.weight == Weight::new(0)
  }
}

impl<T> Typed for Cell<T> {
  fn type_id(&self) -> TypeId {
    self.type_id
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

impl<'a, T> Into<Ref> for &'a mut Cell<T> {
  fn into(self) -> Ref {
    let w = Weight::default();
    *self += w;

    let a = GlobalAddress::new(unimplemented!() /* MEMORY.id() */,
                               (self as *mut Cell<T> as u64).into());

    Ref::new(a, w)
  }
}
