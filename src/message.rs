use address::GlobalAddress;
use object::Object;
use reference::Ref;
use thunk::Thunk;
use weight::Weight;



pub enum Message {
  Fetch { from: Ref, address: GlobalAddress },
  Resume { to: Ref, address: GlobalAddress, object: Box<Object> },

  Fish { from: ProcessorId },
  Schedule { task: Thunk, neighbors: Vec<Thunk> },

  IncreWeight { address: GlobalAddress, delta: Weight },
  DecreWeight { address: GlobalAddress, delta: Weight },

  Ack { to: Ref },
  Finish,
}
