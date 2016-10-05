use std::any;
use std::any::Any;
use std::collections::HashMap;
use std::mem::size_of;
use std::ptr;
use std::ops::Deref;

use serde_cbor::de;
use serde_cbor::ser;

use object::Object;
use local_address::LocalAddress;
use reference::Ref;
use serialized_object::SerializedObject;
use type_id::TypeId;



#[derive(Default)]
pub struct TypeManager {
  from_builtin: HashMap<any::TypeId, TypeId>,
  to_builtin: Vec<any::TypeId>,
  sizes: Vec<usize>,
  ref_extracters: Vec<Box<Fn (usize) -> Vec<Ref>>>,
  serializers: Vec<Box<Fn (usize) -> Vec<u8>>>,
  deserializers: Vec<Box<Fn (&[u8], usize)>>,
}

impl TypeManager {
  pub fn new() -> Self {
    TypeManager::default()
  }

  pub fn register<T: Object + Any>(&mut self) {
    assert_eq!(self.to_builtin.len(), self.sizes.len());
    let t = TypeId::new(self.sizes.len() as u64);
    let builtin = any::TypeId::of::<T>();
    let i: usize = t.into();

    self.from_builtin.insert(builtin, t);
    self.to_builtin[i] = builtin;
    self.sizes.push(size_of::<T>());

    self.ref_extracters[i] = Box::new(move |p: usize| {
      unsafe { ptr::read(p as *const T).into_refs() }
    });

    self.serializers[i] = Box::new(move |p: usize| {
      unsafe { ser::to_vec(&mut *(p as *mut T)).unwrap() }
    });

    self.deserializers[i] = Box::new(move |data, p: usize| {
      unsafe { *(p as *mut T) = de::from_slice(data).unwrap() };
    });
  }

  pub fn extract_refs(&self, a: LocalAddress) -> Vec<Ref> {
    self.ref_extracters[self.from_builtin[&a.type_id()].into(): usize]
        .deref()(a.unknown_object_ptr())
  }

  pub fn serialize(&self, a: LocalAddress) -> SerializedObject {
    let t = self.from_builtin[&a.type_id()];

    SerializedObject::new(
        t,
        self.serializers[t.into(): usize].deref()(a.unknown_object_ptr()))
  }

  pub fn deserialize(&self, s: SerializedObject) -> LocalAddress {
    let i: usize = s.type_id().into();

    let a = LocalAddress::uninitialized(self.sizes[i], self.to_builtin[i]);
    self.deserializers[i].deref()(s.data(), a.unknown_object_ptr());
    a
  }
}
