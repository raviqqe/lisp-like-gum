use address::{GlobalAddress, LocalAddress};
use processor::ProcessorId;
use object::SerializedObject;
use reference::Ref;
use thunk::Thunk;
use weight::Weight;



#[derive(Debug)]
pub enum Message {
  Fetch {
    from: GlobalAddress,
    address: LocalAddress
  },
  Resume {
    to: LocalAddress,
    address: GlobalAddress,
    object: SerializedObject
  },

  Fish { from: ProcessorId },
  Schedule { task: Thunk, neighbors: Vec<(GlobalAddress, SerializedObject)> },

  AddWeight { address: LocalAddress, delta: Weight },
  SubWeight { address: LocalAddress, delta: Weight },

  // Ack { to: LocalAddress },
  Finish,
}

impl From<Vec<u8>> for Message {
  fn from(data: Vec<u8>) -> Self {
    unimplemented!()
  }
}

impl From<Message> for Vec<u8> {
  fn from(m: Message) -> Self {
    unimplemented!()
  }
}
