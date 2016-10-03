use serde_cbor::ser;

use object::Object;
use type_id::TypeId;



#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedObject {
  type_id: TypeId,
  data: Vec<u8>,
}

impl SerializedObject {
  fn new<T: Object>(i: TypeId, o: &T) -> Self {
    SerializedObject {
      type_id: i,
      data: ser::to_vec(o).unwrap(),
    }
  }

  fn type_id(&self) -> TypeId {
    self.type_id
  }

  fn data(&self) -> &[u8] {
    &self.data[..]
  }
}
