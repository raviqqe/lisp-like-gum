use std::any::Any;
use std::fmt;
use std::fmt::Debug;



pub trait Object: Any {
  fn serialize(&self) -> Vec<u8>;
}

pub trait ObjectLike: Object {
  fn deserialize(Vec<u8>) -> Box<Object>;
}

#[derive(Debug, Clone)]
pub struct SerializedObject {
  id: u64,
  data: Vec<u8>,
}

impl From<&Box<Object>> for SerializedObject {
  fn from(o: &Box<Object>) -> Self {
    let mut h = 0; // dummy value
    o.get_type_id().hash(&mut h);

    SerializedObject { id: h, data: o.serialize() }
  }
}

impl Debug for Box<Object> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Object");
  }
}
