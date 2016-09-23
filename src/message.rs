use address::{GlobalAddress, LocalAddress};
use processor::ProcessorId;
use object::Object;
use reference::Ref;
use thunk::Thunk;
use weight::Weight;



#[derive(Debug)]
pub enum Message {
  Fetch { from: GlobalAddress, address: LocalAddress },
  Resume { to: LocalAddress, address: GlobalAddress, object: Box<Object> },

  Fish { from: ProcessorId },
  Schedule { task: Thunk, neighbors: Vec<(GlobalAddress, Thunk)> },

  AddWeight { address: LocalAddress, delta: Weight },
  SubWeight { address: LocalAddress, delta: Weight },

  // Ack { to: LocalAddress },
  Finish,
}
