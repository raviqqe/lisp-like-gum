use type_id::TypeId;



#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedObject {
  type_id: TypeId,
  data: Vec<u8>,
}

impl SerializedObject {
  pub fn new(i: TypeId, v: Vec<u8>) -> Self {
    SerializedObject { type_id: i, data: v }
  }

  pub fn type_id(&self) -> TypeId {
    self.type_id
  }

  pub fn data(&self) -> &[u8] {
    &self.data[..]
  }
}
