use std::any::TypeId;



pub trait Typed {
  fn type_id(&self) -> TypeId;
}
