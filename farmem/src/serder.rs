use std::any;
use std::any::Any;
use std::collections::HashMap;
use std::mem::size_of;
use std::ops::Deref;

use serde_cbor::de;
use serde_cbor::ser;

use object::Object;
use local_address::LocalAddress;
use serialized_object::SerializedObject;
use type_id::TypeId;



#[derive(Default)]
pub struct Serder {
  from_builtin: HashMap<any::TypeId, TypeId>,
  to_builtin: Vec<any::TypeId>,
  sizes: Vec<usize>,
  serializers: Vec<Box<Fn (usize) -> Vec<u8>>>,
  deserializers: Vec<Box<Fn (&[u8], usize)>>,
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

    self.serializers[i] = Box::new(move |p: usize| {
      unsafe { ser::to_vec(&mut *(p as *mut T)).unwrap() }
    });

    self.deserializers[i] = Box::new(move |data, p: usize| {
      unsafe { *(p as *mut T) = de::from_slice(data).unwrap() };
    });
  }

  pub fn serialize(&self, builtin: any::TypeId, o: usize) -> SerializedObject {
    let t = self.from_builtin[&builtin];
    SerializedObject::new(t, self.serializers[t.into(): usize].deref()(o))
  }

  pub fn deserialize(&self, s: SerializedObject) -> LocalAddress {
    let i: usize = s.type_id().into();

    let a = LocalAddress::uninitialized(self.sizes[i], self.to_builtin[i]);
    self.deserializers[i].deref()(s.data(), a.object_pointer());
    a
  }
}
