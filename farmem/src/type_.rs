use std::any;
use std::any::Any;
use std::mem::size_of;
use std::ptr;
use std::ops::Deref;

use serde_cbor::de;
use serde_cbor::ser;

use object::Object;
use reference::Ref;
use serialized_object::SerializedObject;
use type_id::TypeId;



pub struct Type {
  id: TypeId,
  builtin_id: any::TypeId,
  size: usize,
  ref_extracter: Box<Fn (usize) -> Vec<Ref>>,
  serializer: Box<Fn (usize) -> Vec<u8>>,
  deserializer: Box<Fn (&[u8], usize)>,
}

impl Type {
  pub fn new<T: Object + Any>(i: TypeId) -> Self {
    Type {
      id: i,
      builtin_id: any::TypeId::of::<T>(),
      size: size_of::<T>(),
      ref_extracter: Box::new(move |p: usize| {
        unsafe { ptr::read(p as *const T).into_refs() }
      }),
      serializer: Box::new(move |p: usize| {
        unsafe { ser::to_vec(&mut *(p as *mut T)).unwrap() }
      }),
      deserializer: Box::new(move |data, p: usize| {
        unsafe { *(p as *mut T) = de::from_slice(data).unwrap() };
      }),
    }
  }

  pub fn builtin_id(&self) -> any::TypeId {
    self.builtin_id
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn extract_refs(&self, p: usize) -> Vec<Ref> {
    self.ref_extracter.deref()(p)
  }

  pub fn serialize(&self, p: usize) -> SerializedObject {
    SerializedObject::new(self.id, self.serializer.deref()(p))
  }

  pub fn deserialize(&self, s: &[u8], p: usize) {
    self.deserializer.deref()(s, p)
  }
}
