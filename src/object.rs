use std::any::Any;
use std::fmt;
use std::fmt::Debug;



pub trait Object: Any {}

pub trait ObjectLike: Object {
  fn serialize(&self) -> Vec<u8>;
  fn deserialize(&Vec<u8>) -> Box<Object>;
}

impl Debug for Box<Object> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Object");
  }
}
