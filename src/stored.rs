use reference::Ref;
use memory::ThunkMemory;



pub trait Stored {
  fn stored(&self, m: &mut ThunkMemory) -> Ref;
}

// impl<&T> Stored for &T where T: Stored {
//   fn stored(&self, m: &ThunkMemory) -> Ref {
//   }
// }

impl Stored for str {
  fn stored(&self, m: &mut ThunkMemory) -> Ref {
    for c in self.chars().rev() {
      // m.store(c)
      unimplemented!()
    }

    unimplemented!()
  }
}
