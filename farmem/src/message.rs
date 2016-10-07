use local_address::LocalAddress;
use global_address::GlobalAddress;
use memory_id::MemoryId;
use reference::Ref;
use serialized_object::SerializedObject;
use weight::Weight;



#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
  Fetch  { from: MemoryId, local_address: LocalAddress },
  Resume { global_address: GlobalAddress, object: SerializedObject },

  Demand { from: MemoryId },
  Move { reference: Ref, object: SerializedObject },
  Moved { from: GlobalAddress, to: GlobalAddress },

  AddWeight { local_address: LocalAddress, delta: Weight },
  SubWeight { local_address: LocalAddress, delta: Weight },
}
