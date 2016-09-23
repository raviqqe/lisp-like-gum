use std::any::Any;
use std::fmt;
use std::fmt::Debug;



pub trait Object: Any {
}

impl Debug for Box<Object> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Object");
  }
}
