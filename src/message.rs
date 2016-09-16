use object::Object;
use reference::Ref;
use thunk::Thunk;
use weight::Weight;



pub enum Message {
  Fetch { from: Ref, reference: Ref },
  Resume { to: Ref, reference: Ref, object: Box<Object> },

  Fish { from: ProcessorId },
  Schedule { spark: Thunk, neighbors: Vec<Thunk> },

  IncreWeight { reference: Ref, delta: Weight },
  DecreWeight { reference: Ref, delta: Weight },

  Ack { to: Ref },
  Finish,
}
