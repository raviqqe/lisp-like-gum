use serde_cbor::ser;
use serde_cbor::de;

use local_address::LocalAddress;
use global_address::GlobalAddress;
use memory::MemoryId;
use serialized_object::SerializedObject;
use weight::Weight;



#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
  Fetch  { from: MemoryId, local_address: LocalAddress },
  Demand { from: MemoryId },
  Resume { global_address: GlobalAddress, object: SerializedObject },

  AddWeight { local_address: LocalAddress, delta: Weight },
  SubWeight { local_address: LocalAddress, delta: Weight },
}

impl From<Vec<u8>> for Message {
  fn from(v: Vec<u8>) -> Self {
    de::from_slice(v.as_ref()).unwrap()
  }
}

impl From<Message> for Vec<u8> {
  fn from(m: Message) -> Self {
    ser::to_vec(&m).unwrap()
  }
}
