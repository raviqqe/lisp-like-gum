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
