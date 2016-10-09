use std::any;
use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use global_cell::GlobalCell;
use local_cell::LocalCell;
use object::Object;
use reference::Ref;
use serialized_object::SerializedObject;
use type_::Type;
use type_id::TypeId;



#[derive(Default)]
pub struct TypeManager {
  from_id: HashMap<TypeId, Rc<Type>>,
  from_builtin_id: HashMap<any::TypeId, Rc<Type>>,
}

impl TypeManager {
  pub fn new() -> Self {
    TypeManager::default()
  }

  pub fn register<T: Object + Any>(&mut self) {
    assert_eq!(self.from_id.len(), self.from_builtin_id.len());

    let i = TypeId::new(self.from_id.len() as u64);
    let t = Rc::new(Type::new::<T>(i));

    self.from_id.insert(i, t.clone());
    self.from_builtin_id.insert(any::TypeId::of::<T>(), t);
  }

  pub fn extract_refs(&self, c: &LocalCell) -> Vec<Ref> {
    self.from_builtin_id[&c.type_id()].extract_refs(c.unknown_object_ptr())
  }

  pub fn serialize(&self, c: &LocalCell) -> SerializedObject {
    self.from_builtin_id[&c.type_id()].serialize(c.unknown_object_ptr())
  }

  pub fn deserialize(&self, s: SerializedObject) -> GlobalCell {
    let t = &self.from_id[&s.type_id()];
    let c = GlobalCell::uninitialized(t.size(), t.builtin_id());
    t.deserialize(s.data(), c.unknown_object_ptr());
    c
  }
}
