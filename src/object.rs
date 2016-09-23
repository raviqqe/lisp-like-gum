use std::any::Any;
use std::fmt;
use std::fmt::Debug;

use serde::ser::Serialize;
use serde::de::Deserialize;



pub trait Object: Any + Serialize + Deserialize {
}

impl Debug for Box<Object> {
  fn fmt(&self, f: Formatter) -> fmt::Result {
    write!(f, "Object");
  }
}
