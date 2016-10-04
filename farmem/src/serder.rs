use std::any;
use std::any::Any;
use std::collections::HashMap;
use std::mem::size_of;

use libc::c_void;
use serde_cbor::de;

use object::Object;
use serialized_object::SerializedObject;
use type_id::TypeId;



#[derive(Default)]
pub struct Serder {
  from_builtin: HashMap<any::TypeId, TypeId>,
  to_builtin: Vec<any::TypeId>,
  sizes: Vec<usize>,
  deserializers: Vec<Box<Fn (&[u8], *mut c_void)>>,
}

impl Serder {
  pub fn new() -> Self {
    Serder::default()
  }

  pub fn register<T: Object + Any>(&mut self) {
    assert_eq!(self.to_builtin.len(), self.sizes.len());
    let t = TypeId::new(self.sizes.len() as u64);
    let builtin = any::TypeId::of::<T>();
    let i: usize = t.into();

    self.from_builtin.insert(builtin, t);
    self.to_builtin[i] = builtin;
    self.sizes.push(size_of::<T>());
    self.deserializers[i] = Box::new(move |data, p: *mut c_void| {
      unsafe { *(p as *mut T) = de::from_slice(data).unwrap() };
    });
  }

  pub fn serialize<T: Object + Any>(&self, o: &T) -> SerializedObject {
    SerializedObject::new(self.from_builtin[&any::TypeId::of::<T>()], o)
  }

  pub fn deserialize(&self, s: SerializedObject) -> *mut c_void {
    // let p = ;
    // self.deserializers[s.type_id().into()](s.data(), p)
    unimplemented!()
  }
}
